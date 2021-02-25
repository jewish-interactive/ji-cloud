use shared::domain::{
    meta::{StyleId, AffiliationId, AgeRangeId},
    category::CategoryId,
    image::{Image, ImageId, ImageSearchQuery},
};
use futures_signals::signal::Mutable;
use dominator_helpers::futures::AsyncLoader;
use std::collections::HashSet;
use web_sys::HtmlInputElement;
use std::cell::RefCell;

pub struct State {
    pub id: ImageId, 
    pub query: Option<ImageSearchQuery>,
    pub section: Mutable<Section>,
    pub loader: AsyncLoader,
    pub loaded: Mutable<bool>,
    pub delete_modal: Mutable<bool>,
    pub file_input: RefCell<Option<HtmlInputElement>>,
}

impl State {
    pub fn new(id: ImageId, query: Option<ImageSearchQuery>) -> Self {

        let section = {
            if query.is_some() 
                { Section::Three }
            else
                { Section:: Three}
        };

        Self {
            id,
            query,
            section: Mutable::new(section),
            loader: AsyncLoader::new(),
            loaded: Mutable::new(false),
            delete_modal: Mutable::new(false),
            file_input: RefCell::new(None),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Section {
    One,
    Two,
    Three
}

#[derive(Clone)]
pub struct MutableImage {
    pub id: Mutable<ImageId>,
    pub name: Mutable<String>,
    pub description: Mutable<String>,
    pub is_premium: Mutable<bool>,
    pub styles: Mutable<HashSet<StyleId>>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub categories: Mutable<HashSet<CategoryId>>,
}

impl From<Image> for MutableImage {
    fn from(image:Image) -> Self {
        Self {
            id: Mutable::new(image.id),
            name: Mutable::new(image.name),
            description: Mutable::new(image.description),
            is_premium: Mutable::new(image.is_premium),
            styles: {
                let mut styles = HashSet::with_capacity(image.styles.len());
                for id in image.styles.into_iter() {
                    styles.insert(id);
                }
                Mutable::new(styles)
            },
            age_ranges: {
                let mut age_ranges = HashSet::with_capacity(image.age_ranges.len());
                for id in image.age_ranges.into_iter() {
                    age_ranges.insert(id);
                }
                Mutable::new(age_ranges)
            },
            affiliations: {
                let mut affiliations = HashSet::with_capacity(image.affiliations.len());
                for id in image.affiliations.into_iter() {
                    affiliations.insert(id);
                }
                Mutable::new(affiliations)
            },
            categories: {
                let mut categories = HashSet::with_capacity(image.categories.len());
                for id in image.categories.into_iter() {
                    categories.insert(id);
                }
                Mutable::new(categories)
            },
        }
    }
}
