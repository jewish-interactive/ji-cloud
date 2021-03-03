#[derive(Debug, Copy, Clone)]
pub enum Fixture {
    User,
    MetaKinds,
    CategoryOrdering,
    CategoryNesting,
    Image,
    UserNoPerms,
}

impl Fixture {
    pub const fn as_query(self) -> &'static str {
        match self {
            Self::User => include_str!("../../fixtures/1_user.sql"),
            Self::MetaKinds => include_str!("../../fixtures/2_meta_kinds.sql"),
            Self::CategoryOrdering => {
                include_str!("../../fixtures/3_category_ordering.sql")
            }
            Self::CategoryNesting => include_str!("../../fixtures/4_category_nesting.sql"),
            Self::Image => include_str!("../../fixtures/5_image.sql"),
            Self::UserNoPerms => include_str!("../../fixtures/6_user_no_perms.sql"),
        }
    }
}