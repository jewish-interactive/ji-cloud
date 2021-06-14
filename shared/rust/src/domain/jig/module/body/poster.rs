use crate::domain::jig::module::{
    body::{
        Backgrounds, Body, BodyExt, EditorState, Instructions, ModeExt, StepExt, Sticker,
        ThemeChoice,
    },
    ModuleKind,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// The body for [`Poster`](crate::domain::jig::module::ModuleKind::Poster) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<Mode> for ModuleData {
    fn as_body(&self) -> Body {
        Body::Poster(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Poster
    }
    fn new_mode(mode: Mode) -> Self {
        ModuleData {
            content: Some(Content {
                mode,
                ..Content::default()
            }),
        }
    }

    fn requires_choose_mode(&self) -> bool {
        self.content.is_none()
    }
}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::Poster(data) => Ok(data),
            _ => Err("cannot convert body to poster!"),
        }
    }
}

/// The body for [`Poster`](crate::domain::jig::module::ModuleKind::Poster) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Content {
    /// The editor state
    pub editor_state: EditorState<Step>,

    /// The mode
    pub mode: Mode,

    /// The instructions for the module.
    pub instructions: Instructions,

    /// The module's theme.
    pub theme: ThemeChoice,

    /// Backgrounds
    pub backgrounds: Backgrounds,

    /// Stickers
    pub stickers: Vec<Sticker>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// The mode
pub enum Mode {
    /// Printables
    Printables,
    /// TalkingPictures
    TalkingPictures,
    /// Comics
    Comics,
    /// Timeline
    Timeline,
    /// Family Tree
    FamilyTree,
    /// Poster
    Poster,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Poster
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        vec![
            Self::Printables,
            Self::TalkingPictures,
            Self::Comics,
            Self::Timeline,
            Self::FamilyTree,
            Self::Poster,
        ]
    }

    fn title() -> &'static str {
        const STR_TITLE: &'static str = "Create a Poster";
        STR_TITLE
    }

    fn module_str_id() -> &'static str {
        "poster"
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::Printables => "printables",
            Self::TalkingPictures => "talking-pictures",
            Self::Comics => "comics",
            Self::Timeline => "timeline",
            Self::FamilyTree => "family-tree",
            Self::Poster => "poster",
        }
    }

    fn as_str_label(&self) -> &'static str {
        const STR_PRINTABLES_LABEL: &'static str = "Printables";
        const STR_TALKING_PICTURES_LABEL: &'static str = "Talking Pictures";
        const STR_COMICS_LABEL: &'static str = "Comics";
        const STR_TIMELINE_LABEL: &'static str = "Timeline";
        const STR_FAMILY_TREE_LABEL: &'static str = "Family Tree";
        const STR_POSTER_LABEL: &'static str = "Poster";

        match self {
            Self::Printables => STR_PRINTABLES_LABEL,
            Self::TalkingPictures => STR_TALKING_PICTURES_LABEL,
            Self::Comics => STR_COMICS_LABEL,
            Self::Timeline => STR_TIMELINE_LABEL,
            Self::FamilyTree => STR_FAMILY_TREE_LABEL,
            Self::Poster => STR_POSTER_LABEL,
        }
    }
}

/// The Steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Step {
    /// Step 1
    One,
    /// Step 2
    Two,
    /// Step 3
    Three,
    /// Step 4
    Four,
}

impl Default for Step {
    fn default() -> Self {
        Self::One
    }
}

impl StepExt for Step {
    fn next(&self) -> Option<Self> {
        match self {
            Self::One => Some(Self::Two),
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Four),
            Self::Four => None,
        }
    }

    fn as_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }

    fn label(&self) -> &'static str {
        //TODO - localizaton
        const STR_THEMES: &'static str = "Themes";
        const STR_BACKGROUND: &'static str = "Background";
        const STR_CONTENT: &'static str = "Content";
        const STR_PREVIEW: &'static str = "Preview";

        match self {
            Self::One => STR_THEMES,
            Self::Two => STR_BACKGROUND,
            Self::Three => STR_CONTENT,
            Self::Four => STR_PREVIEW,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![Self::One, Self::Two, Self::Three, Self::Four]
    }
    fn get_preview() -> Self {
        Self::Four
    }
}

use paperclip::v2::{schema::TypedData, models::{DataType, DataTypeFormat}};

impl TypedData for Step {
    /// The OpenAPI type for `poster::Step`.
    fn data_type() -> DataType {
        DataType::Integer
    }

    /// The optional OpenAPI data format for `poster::Step`.
    fn format() -> Option<DataTypeFormat> {
        None
    }
}
