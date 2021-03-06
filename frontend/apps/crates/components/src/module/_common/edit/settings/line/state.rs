// These must match the typescript / custom element variants

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineKind {
    CardView,
    GameDisplay,
    Rounds,
    TimeLimit,
    Attempts,
    Score
}

impl LineKind {
    pub fn as_str_id(&self) -> &'static str {
        match self {
            Self::CardView => "card-view",
            Self::GameDisplay => "game-display",
            Self::Rounds => "rounds",
            Self::TimeLimit => "time-limit",
            Self::Attempts => "attempts",
            Self::Score => "score",

        }
    }
}
