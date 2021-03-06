//! Types for metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;

/// Wrapper type around [`Uuid`], represents [`ImageStyle::id`].
#[derive(Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ImageStyleId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`AnimationStyle::id`].
#[derive(Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AnimationStyleId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`AudioStyle::id`].
#[derive(Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AudioStyleId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`AgeRange::id`].
#[derive(Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AgeRangeId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`Affiliation::id`].
#[derive(Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AffiliationId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`Subject::id`].
#[derive(Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct SubjectId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`Goal::id`].
#[derive(Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct GoalId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`Tag::id`].
#[derive(Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct TagId(pub Uuid);

into_uuid!(
    ImageStyleId,
    AnimationStyleId,
    AffiliationId,
    AgeRangeId,
    SubjectId,
    GoalId,
    TagId
);

/// Represents an image style.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ImageStyle {
    /// The id of the image style.
    pub id: ImageStyleId,

    /// The image style's name.
    pub display_name: String,

    /// When the image style was created.
    pub created_at: DateTime<Utc>,

    /// When the image style was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents an animation style.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AnimationStyle {
    /// The id of the animation style.
    pub id: AnimationStyleId,

    /// The animation style's name.
    pub display_name: String,

    /// When the animation style was created.
    pub created_at: DateTime<Utc>,

    /// When the animation style was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents a age range.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AgeRange {
    /// The id of the age range.
    pub id: AgeRangeId,

    /// The age range's name.
    pub display_name: String,

    /// When the age range was created.
    pub created_at: DateTime<Utc>,

    /// When the age range was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents an affiliation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Affiliation {
    /// The id of the affiliation.
    pub id: AffiliationId,

    /// The affiliation's name.
    pub display_name: String,

    /// When the affiliation was created.
    pub created_at: DateTime<Utc>,

    /// When the affiliation was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents a subject.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Subject {
    /// The id of the subject.
    pub id: SubjectId,

    /// The subject's name.
    pub display_name: String,

    /// When the subject was created.
    pub created_at: DateTime<Utc>,

    /// When the subject was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents a goal.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Goal {
    /// The id of the goal.
    pub id: GoalId,

    /// The goal's name.
    pub display_name: String,

    /// When the goal was created.
    pub created_at: DateTime<Utc>,

    /// When the goal was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents a tag.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Tag {
    /// Index of the tag.
    pub index: i16,

    /// The id of the tag.
    pub id: TagId,

    /// The tag's name.
    pub display_name: String,

    /// When the tag was created.
    pub created_at: DateTime<Utc>,

    /// When the tag was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Response for fetching all metadata.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct MetadataResponse {
    /// All image styles the server has.
    pub image_styles: Vec<ImageStyle>,

    /// All animation styles the server has.
    pub animation_styles: Vec<AnimationStyle>,

    /// All audio...
    // TODO

    /// All age ranges the server has.
    pub age_ranges: Vec<AgeRange>,

    /// All affiliations the server has.
    pub affiliations: Vec<Affiliation>,

    /// All subjects the server has.
    pub subjects: Vec<Subject>,

    /// All goals.
    pub goals: Vec<Goal>,

    /// All tags for images.
    pub image_tags: Vec<Tag>,
}

/// Metadata kinds.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum MetaKind {
    /// [`Affiliation`]
    Affiliation,

    /// [`ImageStyle`]
    ImageStyle,

    /// [`AnimationStyle`]
    AnimationStyle,

    /// [`AgeRange`]
    AgeRange,

    /// [`Category`](super::category::Category)
    Category,

    /// [`Subject`]
    Subject,

    /// [`Goal`]
    Goal,

    /// [`Tag`]
    Tag,
}
