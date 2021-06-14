use crate::domain::jig::module::{
    body::{Body, BodyExt, EditorState, Image, Instructions, ModeExt, StepExt, ThemeChoice},
    ModuleKind,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// The body for [`Memory`](crate::domain::jig::module::ModuleKind::Memory) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

/// The content for [`Memory`](crate::domain::jig::module::ModuleKind::Memory) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Content {
    /// The editor state
    pub editor_state: EditorState<Step>,

    /// The instructions for the module.
    pub instructions: Instructions,

    /// The mode the module uses.
    pub mode: Mode,

    /// The pairs of cards that make up the module.
    pub pairs: Vec<CardPair>,

    /// The ID of the module's theme.
    pub theme: ThemeChoice,
}

impl BodyExt<Mode> for ModuleData {
    fn as_body(&self) -> Body {
        Body::MemoryGame(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Memory
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
            Body::MemoryGame(data) => Ok(data),
            _ => Err("cannot convert body to memory game!"),
        }
    }
}

/// A pair of cards
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct CardPair(pub Card, pub Card);

/// An individual card.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum Card {
    // todo(@dakom): document this
    #[allow(missing_docs)]
    Text(String),

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Image(Option<Image>),
}

/// What mode the module runs in.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
pub enum Mode {
    // todo(@dakom): document this
    #[allow(missing_docs)]
    Duplicate = 0,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    WordsAndImages = 1,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    BeginsWith = 2,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Lettering = 3,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Riddles = 4,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Opposites = 5,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Synonymns = 6,

    /// Translate from one language to another.
    Translate = 7,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Duplicate
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        vec![
            Self::Duplicate,
            Self::WordsAndImages,
            Self::BeginsWith,
            Self::Lettering,
            Self::Riddles,
            Self::Opposites,
            Self::Synonymns,
            Self::Translate,
        ]
    }

    fn title() -> &'static str {
        const STR_TITLE: &'static str = "Create a Memory Game";
        STR_TITLE
    }

    fn module_str_id() -> &'static str {
        "memory"
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::WordsAndImages => "words-images",
            Self::BeginsWith => "begins-with",
            Self::Lettering => "lettering",
            Self::Riddles => "riddles",
            Self::Opposites => "opposites",
            Self::Synonymns => "synonymns",
            Self::Translate => "translate",
        }
    }

    fn as_str_label(&self) -> &'static str {
        const STR_DUPLICATE: &'static str = "Duplicate";
        const STR_WORDS_IMAGES: &'static str = "Words & Images";
        const STR_BEGINS_WITH: &'static str = "What begins with...";
        const STR_LETTERING: &'static str = "Lettering";
        const STR_RIDDLES: &'static str = "Riddles";
        const STR_OPPOSITES: &'static str = "Opposites";
        const STR_SYNONYMNS: &'static str = "Synonymns";
        const STR_TRANSLATE: &'static str = "Translate";

        match self {
            Self::Duplicate => STR_DUPLICATE,
            Self::WordsAndImages => STR_WORDS_IMAGES,
            Self::BeginsWith => STR_BEGINS_WITH,
            Self::Lettering => STR_LETTERING,
            Self::Riddles => STR_RIDDLES,
            Self::Opposites => STR_OPPOSITES,
            Self::Synonymns => STR_SYNONYMNS,
            Self::Translate => STR_TRANSLATE,
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
        const STR_CONTENT: &'static str = "Content";
        const STR_DESIGN: &'static str = "Design";
        const STR_SETTINGS: &'static str = "Settings";
        const STR_PREVIEW: &'static str = "Preview";

        match self {
            Self::One => STR_CONTENT,
            Self::Two => STR_DESIGN,
            Self::Three => STR_SETTINGS,
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
    /// The OpenAPI type for `memory::Step`.
    fn data_type() -> DataType {
        DataType::Integer
    }

    /// The optional OpenAPI data format for `memory::Step`.
    fn format() -> Option<DataTypeFormat> {
        None
    }
}
