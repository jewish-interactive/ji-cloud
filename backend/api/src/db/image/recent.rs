use crate::error;
use chrono::{DateTime, Utc};
use shared::{
    domain::image::{recent::UserRecentImageResponse, ImageId},
    media::MediaLibrary,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    db: &PgPool,
    user_id: Uuid,
    image_id: ImageId,
    library: MediaLibrary,
) -> anyhow::Result<(ImageId, MediaLibrary, DateTime<Utc>), error::UserRecentImage> {
    let mut txn = db.begin().await?;

    let exists = sqlx::query!(
        r#"
select exists(select 1 from user_recent_image where user_id = $1 and image_id = $2) as "exists!"
            "#,
        user_id,
        image_id.0,
    )
    .fetch_one(&mut txn)
    .await?
    .exists;

    if exists {
        return Err(error::UserRecentImage::Conflict);
    }

    let res = sqlx::query!(
            r#"
insert into user_recent_image (user_id, image_id, media_library)
values ($1, $2, $3)
returning image_id as "id: ImageId", media_library as "library: MediaLibrary", last_used as "last_used: DateTime<Utc>";
            "#,
            user_id,
            image_id.0,
            library as i16,
        )
        .fetch_one(&mut txn)
        .await?;

    txn.commit().await?;

    Ok((res.id, res.library, res.last_used))
}

pub async fn update(
    db: &PgPool,
    user_id: Uuid,
    image_id: ImageId,
) -> anyhow::Result<(), error::UserRecentImage> {
    let mut txn = db.begin().await?;

    let exists = sqlx::query!(
        r#"
select exists(select 1 from user_recent_image where user_id = $1 and image_id = $2) as "exists!"
            "#,
        user_id,
        image_id.0,
    )
    .fetch_one(&mut txn)
    .await?
    .exists;

    if !exists {
        return Err(error::UserRecentImage::ResourceNotFound);
    }

    sqlx::query!(
        r#"
update user_recent_image
set last_used = now()
where user_id = $1 and image_id = $2
            "#,
        user_id,
        image_id.0,
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await.map_err(Into::into)
}

pub async fn delete(db: &PgPool, user_id: Uuid, image_id: ImageId) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
delete from user_recent_image
where user_id = $1 and image_id = $2
            "#,
        user_id,
        image_id.0
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn list(
    db: &PgPool,
    user_id: Uuid,
    limit: Option<i64>,
) -> sqlx::Result<Vec<UserRecentImageResponse>> {
    // if let Some(limit) = limit { assert!(limit >= 0); }

    sqlx::query_as!(
            UserRecentImageResponse,
            r#"
select image_id as "id: ImageId", media_library as "library: MediaLibrary", last_used as "last_used: DateTime<Utc>"
from user_recent_image
where user_id = $1
order by last_used desc
limit $2
            "#,
            user_id,
            limit
        )
        .fetch_all(db)
        .await
}
