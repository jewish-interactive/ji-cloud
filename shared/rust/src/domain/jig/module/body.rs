use crate::{
    domain::{audio::AudioId, image::ImageId},
    media::MediaLibrary,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::Debug;

/// Memory Game Body.
pub mod memory;

/// Poster Body.
pub mod poster;

/// Tapping Board Body.
pub mod tapping_board;

/// Cover Body.
pub mod cover;

/// Body kinds for Modules.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Body {
    /// Module is a memory game, and has a memory game's body.
    MemoryGame(memory::ModuleData),

    /// Module is a poster, and has a poster's body.
    Poster(poster::ModuleData),

    /// Module is a tapping board, and has a tapping board's body.
    TappingBoard(tapping_board::ModuleData),

    /// Module is a [`Cover`](super::ModuleKind::Cover).
    ///
    /// DEPRECATED INFO: This exists as an empty enum because cover *needs* to exist, but it also isn't decided yet.
    Cover(cover::ModuleData),
}

/// Extension trait for interop
/// impl on inner body data
pub trait BodyExt<Mode>: TryFrom<Body> + Serialize + DeserializeOwned + Clone + Debug {
    /// get self as a Body
    fn as_body(&self) -> Body;

    /// is complete
    fn is_complete(&self) -> bool;

    /// get the kind from the type itself
    fn kind() -> super::ModuleKind;

    /// given a Mode, get a new Self
    /// will usually populate an inner .content
    fn new_mode(mode: Mode) -> Self;

    /// requires an additional step of choosing the mode
    fn requires_choose_mode(&self) -> bool;
}

impl Body {
    /// Gets this body's related [`ModuleKind`](super::ModuleKind)
    pub fn kind(&self) -> super::ModuleKind {
        match self {
            Self::Cover(_) => super::ModuleKind::Cover,
            Self::MemoryGame(_) => super::ModuleKind::Memory,
            Self::Poster(_) => super::ModuleKind::Poster,
            Self::TappingBoard(_) => super::ModuleKind::TappingBoard,
        }
    }
}

/// Theme Ids. Used in various modules
/// See the frontend extension trait for more info
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
pub enum ThemeId {
    /// Empty theme (white bg, no text, etc.)
    Blank = 0,
    /// Blueish theme
    Chalkboard = 1,
    /// Orangeish theme
    HappyBrush = 2,
}

impl Default for ThemeId {
    fn default() -> Self {
        Self::Blank
    }
}

/// Theme choice, either jig or override
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum ThemeChoice {
    /// Use the jig's theme
    Jig,

    /// Override it with a per-module choice
    Override(ThemeId),
}

impl Default for ThemeChoice {
    fn default() -> Self {
        Self::Jig
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Audio
pub struct Audio {
    /// The Audio Id
    pub id: AudioId,
    /// The Media Library
    pub lib: MediaLibrary,
}

/// Instructions for a module.
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Instructions {
    /// Text displayed in banner
    pub text: Option<String>,

    /// Audio played on module start
    pub audio: Option<Audio>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Background
/// although it's simply a list of layers
/// the number of layers is predefined
/// and has special meaning from a UI perspective
pub struct Backgrounds {
    /// Layer 1
    pub layer_1: Option<Background>,
    /// Layer 2
    pub layer_2: Option<Background>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Background
pub enum Background {
    /// Color
    Color(rgb::RGBA8),
    /// Theme-based
    Theme(ThemeId),
    /// Any other image
    Image(Image),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Stickers are things that can be rendered and transformed
pub enum Sticker {
    /// Sprites
    Sprite(Sprite),
    /// Text
    Text(Text),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Text are serialized text things
pub struct Text {
    /// the raw text
    pub value: String,
    /// The Transform
    pub transform: Transform,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Images need id and lib
pub struct Image {
    /// The Image Id
    pub id: ImageId,
    /// The MediaLibrary
    pub lib: MediaLibrary,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Sprites are a combo of image + transform
pub struct Sprite {
    /// The Image
    pub image: Image,
    /// The Transform
    pub transform: Transform,
    /// Effects
    pub effects: Vec<SpriteEffect>,

    /// Flip horizontal
    pub flip_horizontal: bool,

    /// Flip vertical
    pub flip_vertical: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Sprite Effects
pub enum SpriteEffect {
    /// Remove White
    RemoveWhite,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Vector of 3 floats
pub struct Vec3(pub [f64; 3]);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Vector of 4 floats, also used as a Quaternion
pub struct Vec4(pub [f64; 4]);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Visual Transform
pub struct Transform {
    /// Translation
    pub translation: Vec3,
    /// Rotation Quaternion
    pub rotation: Vec4,
    /// Scale for each axis
    pub scale: Vec3,
    /// Origin
    pub origin: Vec3,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Trace
pub struct Trace {
    /// The Transform
    pub transform: Transform,
    /// The Shape
    pub shape: TraceShape,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Trace shape
pub enum TraceShape {
    /// width and height
    Rect(f64, f64),
    /// radius
    Ellipse(f64, f64),
    /// points
    Path(Vec<(f64, f64)>),
}
