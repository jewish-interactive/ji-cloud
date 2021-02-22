//! Types for jig Modules.

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

/// Wrapper type around [`Uuid`](Uuid), represents the ID of a module.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleId(pub Uuid);

/// Represents the various kinds of data a module can represent.
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum ModuleKind {
    /// This is a sortof special module, every jig has one and it can't be deleted
    Cover = 0,

    /// Flashcards
    Flashcards = 1,
    /// Matching
    Matching = 2,
    /// Memory Game
    Memory = 3,
    /// Poster
    Poster = 4,
    /// Tapping Board
    TappingBoard = 5,
    /// Tracing
    Tracing = 6,
    /// Video
    Video = 7,
    /// Visual Quiz
    VisualQuiz = 8,
}

impl ModuleKind {
    /// casts `self` to a string
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cover => "cover",
            Self::Flashcards => "flashcards",
            Self::Matching => "matching",
            Self::Memory => "memory",
            Self::Poster => "poster",
            Self::TappingBoard => "tapping-board",
            Self::Tracing => "tracing",
            Self::Video => "video",
            Self::VisualQuiz => "visual-quiz",
        }
    }
}

impl FromStr for ModuleKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "cover" => Self::Cover,
            "flashcards" => Self::Flashcards,
            "matching" => Self::Matching,
            "memory" => Self::Memory,
            "poster" => Self::Poster,
            "tapping-board" => Self::TappingBoard,
            "tracing" => Self::Tracing,
            "video" => Self::Video,
            "visual-quiz" => Self::VisualQuiz,
            _ => anyhow::bail!("Invalid ModuleKind: {}", s),
        };

        Ok(res)
    }
}

/// Minimal information about a module.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct LiteModule {
    /// The module's ID.
    pub id: ModuleId,

    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,
}

/// Over the wire representation of a module.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Module {
    /// The module's ID.
    pub id: ModuleId,

    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,

    /// The module's json contents.
    pub body: Option<serde_json::Value>,
}

/// Request to create a new `Module`.
#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleCreateRequest {
    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,

    /// The module's json contents.
    pub body: Option<serde_json::Value>,
}

/// Response for successfully finding a module
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleResponse {
    /// The module we found
    pub module: Module,
}

/// Request to update a `Module`.
/// note: fields here cannot be nulled out (`None` means "don't change").
#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleUpdateRequest {
    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,

    /// The module's json contents.
    pub body: Option<serde_json::Value>,
}

into_uuid![ModuleId];
