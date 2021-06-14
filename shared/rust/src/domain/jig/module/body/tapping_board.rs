use crate::domain::jig::module::{
    body::{
        Audio, Backgrounds, Body, BodyExt, EditorState, Instructions, ModeExt, StepExt, Sticker,
        ThemeChoice, Trace,
    },
    ModuleKind,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

mod play_settings;
pub use play_settings::*;

/// The body for [`TappingBoard`](crate::domain::jig::module::ModuleKind::TappingBoard) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<Mode> for ModuleData {
    fn as_body(&self) -> Body {
        Body::TappingBoard(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::TappingBoard
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
            Body::TappingBoard(data) => Ok(data),
            _ => Err("cannot convert body to tapping board!"),
        }
    }
}

/// The body for [`TappingBoard`](crate::domain::jig::module::ModuleKind::TappingBoard) modules.
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

    /// Traces
    pub traces: Vec<TappingTrace>,

    /// play settings
    pub play_settings: PlaySettings,
}

/// Tapping board trace w/ metadata
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct TappingTrace {
    /// the trace
    pub trace: Trace,

    /// audio
    pub audio: Option<Audio>,

    /// text
    pub text: Option<String>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// The mode
pub enum Mode {
    /// Words mode
    Words,
    /// Images mode
    Images,
    /// Talk mode
    Talk,
    /// Read mode
    Read,
    /// Draw mode
    Draw,
    /// Scene mode
    Scene,
    /// Photo album mode
    PhotoAlbum,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Words
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        vec![
            Self::Words,
            Self::Images,
            Self::Talk,
            Self::Read,
            Self::Draw,
            Self::Scene,
            Self::PhotoAlbum,
        ]
    }

    fn title() -> &'static str {
        const STR_TITLE: &'static str = "Create a Tapping Board";

        STR_TITLE
    }

    fn module_str_id() -> &'static str {
        "tapping-board"
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::Words => "words",
            Self::Images => "images",
            Self::Talk => "talk",
            Self::Read => "read",
            Self::Draw => "draw",
            Self::Scene => "scene",
            Self::PhotoAlbum => "photo-album",
        }
    }

    fn as_str_label(&self) -> &'static str {
        const STR_WORDS_LABEL: &'static str = "Tap words & hear";
        const STR_IMAGES_LABEL: &'static str = "Tap images & hear";
        const STR_TALK_LABEL: &'static str = "Tap & talk";
        const STR_READ_LABEL: &'static str = "Tap & read";
        const STR_DRAW_LABEL: &'static str = "Interactive drawing";
        const STR_SCENE_LABEL: &'static str = "Interactive scene";
        const STR_PHOTO_ALBUM_LABEL: &'static str = "Interactive photo album";

        match self {
            Self::Words => STR_WORDS_LABEL,
            Self::Images => STR_IMAGES_LABEL,
            Self::Talk => STR_TALK_LABEL,
            Self::Read => STR_READ_LABEL,
            Self::Draw => STR_DRAW_LABEL,
            Self::Scene => STR_SCENE_LABEL,
            Self::PhotoAlbum => STR_PHOTO_ALBUM_LABEL,
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
    /// Step 5
    Five,
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
            Self::Four => Some(Self::Five),
            Self::Five => None,
        }
    }

    fn as_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 4,
        }
    }

    fn label(&self) -> &'static str {
        //TODO - localizaton
        const STR_BACKGROUND: &'static str = "Background";
        const STR_CONTENT: &'static str = "Content";
        const STR_INTERACTION: &'static str = "Interaction";
        const STR_SETTINGS: &'static str = "Settings";
        const STR_PREVIEW: &'static str = "Preview";
        match self {
            Self::One => STR_BACKGROUND,
            Self::Two => STR_CONTENT,
            Self::Three => STR_INTERACTION,
            Self::Four => STR_SETTINGS,
            Self::Five => STR_PREVIEW,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![Self::One, Self::Two, Self::Three, Self::Four, Self::Five]
    }
    fn get_preview() -> Self {
        Self::Five
    }
}

use paperclip::v2::{schema::TypedData, models::{DataType, DataTypeFormat}};

impl TypedData for Step {
    /// The OpenAPI type for `tapping_board::Step`.
    fn data_type() -> DataType {
        DataType::Integer
    }

    /// The optional OpenAPI data format for `tapping_board::Step`.
    fn format() -> Option<DataTypeFormat> {
        None
    }
}
