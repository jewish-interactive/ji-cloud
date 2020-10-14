use core::settings::{self, SettingsManager};
use ji_cloud_api::{algolia, db, http, jwkkeys, logger, s3};
use std::thread;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();

    logger::init()?;

    let (runtime_settings, jwk_verifier, s3, algolia, db_pool) = {
        let remote_target = settings::read_remote_target()?;

        let settings: SettingsManager = SettingsManager::new(remote_target).await?;

        let runtime_settings = settings.runtime_settings().await?;

        let jwk_verifier = jwkkeys::create_verifier(settings.jwk_settings().await?);

        let _ = jwkkeys::run_task(jwk_verifier.clone());

        let s3 = s3::S3Client::new(settings.s3_settings().await?)?;

        let algolia = algolia::AlgoliaClient::new(settings.algolia_settings().await?)?;

        let db_pool = db::get_pool(
            settings
                .db_connect_options(settings::read_sql_proxy())
                .await?,
        )
        .await?;

        (runtime_settings, jwk_verifier, s3, algolia, db_pool)
    };

    let algolia_syncer = algolia::Updater {
        db: db_pool.clone(),
        algolia_client: algolia.clone(),
    };

    let _ = algolia_syncer.spawn();

    let handle = thread::spawn(|| http::run(db_pool, runtime_settings, jwk_verifier, s3, algolia));

    log::info!("app started!");

    handle.join().unwrap()?;

    Ok(())
}
