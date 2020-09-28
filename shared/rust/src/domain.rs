//! Types that travel over the wire.

use serde::de::DeserializeOwned;
use std::fmt::Write;
use uuid::Uuid;

pub mod auth;
pub mod category;
pub mod image;
pub mod meta;
pub mod user;

/// Hack to deserialize an Optional [`Option<T>`]
///
/// This is to differentiate between "missing" values and null values.
/// For example in json `{"v": null}` and `{}` are different things, in the first one, `v` is `null`, but in the second, v is `undefined`.
///
/// [`Option<T>`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    Ok(Some(serde::Deserialize::deserialize(deserializer)?))
}

fn csv_encode_uuids<S>(uuids: &[Uuid], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // todo: use a `Display` struct to use `collect_str`
    // but for now, pre-allocate the whole string.

    // a hyphenated uuid is 36 bytes long, we have `n` of those, then we also have `n - 1` 1 byte separators.
    let len = uuids.len() * 36 + uuids.len().saturating_sub(1);

    let mut out = String::with_capacity(len);
    let mut iter = uuids.iter();
    if let Some(item) = iter.next() {
        write!(&mut out, "{}", item.to_hyphenated())
            .expect("`String` call to `write!` shouldn't fail.");
    }

    for item in iter {
        write!(&mut out, ",{}", item.to_hyphenated())
            .expect("`String` call to `write!` shouldn't fail");
    }

    serializer.serialize_str(&out)
}

fn from_csv<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: DeserializeOwned,
{
    deserializer.deserialize_str(CSVVecVisitor::<T>::default())
}

/// Visits a string value of the form "v1,v2,v3" into a vector of bytes Vec<u8>
struct CSVVecVisitor<T: DeserializeOwned>(std::marker::PhantomData<T>);

impl<T: DeserializeOwned> Default for CSVVecVisitor<T> {
    fn default() -> Self {
        CSVVecVisitor(std::marker::PhantomData)
    }
}

impl<'de, T: DeserializeOwned> serde::de::Visitor<'de> for CSVVecVisitor<T> {
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a str")
    }

    fn visit_str<E>(self, s: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(s.as_bytes())
            .into_deserialize()
            .next()
            .unwrap_or_else(|| Ok(Vec::new()))
            .map_err(|e| E::custom(format!("could not deserialize sequence value: {:?}", e)))
    }
}
