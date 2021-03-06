use std::{collections::{HashMap, HashSet}, rc::Rc};

use futures::join;
use futures_signals::signal::Mutable;
use shared::{
    api::{
        ApiEndpoint,
        endpoints::{category, meta}
    },
    domain::{
        category::{Category, CategoryId, CategoryResponse, CategoryTreeScope, GetCategoryRequest},
        jig::JigSearchQuery,
        meta::{Affiliation, AffiliationId, AgeRange, AgeRangeId, Goal, GoalId, MetadataResponse}
    },
    error::EmptyError
};
use utils::prelude::*;


const STR_LANGUAGE_ENGLISH: &'static str = "English";
const STR_LANGUAGE_HEBREW: &'static str = "Hebrew";


pub type Language = (&'static str, &'static str);


#[derive(Debug)]
pub struct SearchSelected {
    pub goals: Mutable<HashSet<GoalId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub language: Mutable<Option<String>>,
    pub query: Mutable<String>,
}

impl SearchSelected {
    pub fn new() -> Self {
        Self {
            goals: Mutable::new(HashSet::new()),
            affiliations: Mutable::new(HashSet::new()),
            categories: Mutable::new(HashSet::new()),
            age_ranges: Mutable::new(HashSet::new()),
            language: Mutable::new(None),
            query: Mutable::new(String::new()),
        }
    }

    pub fn to_search_request(&self) -> JigSearchQuery {
        log::info!("{:?}", self);
        JigSearchQuery {
            q: self.query.lock_ref().to_owned(),
            age_ranges: self.age_ranges.lock_ref().to_owned().into_iter().collect(),
            affiliations: self.affiliations.lock_ref().to_owned().into_iter().collect(),
            categories: self.categories.lock_ref().to_owned().into_iter().collect(),
            goals: self.goals.lock_ref().to_owned().into_iter().collect(),
            page: Some(0),
            ..Default::default()
        }
    }
}

pub struct SearchOptions {
    pub age_ranges: Mutable<Vec<AgeRange>>,
    pub goals: Mutable<Vec<Goal>>,
    pub affiliations: Mutable<Vec<Affiliation>>,
    pub categories: Mutable<Vec<Category>>,
    pub category_label_lookup: Mutable<HashMap<CategoryId, String>>,
    pub languages: Rc<Vec<Language>>,
}

impl SearchOptions {
    pub fn new() -> Self {
        Self {
            age_ranges: Mutable::new(vec![]),
            goals: Mutable::new(vec![]),
            affiliations: Mutable::new(vec![]),
            categories: Mutable::new(vec![]),
            category_label_lookup: Mutable::new(HashMap::new()),
            languages: Rc::new(vec![
                ("en", STR_LANGUAGE_ENGLISH),
                ("he", STR_LANGUAGE_HEBREW),
            ]),
        }
    }

    pub async fn populate_options(&self) {
        let _ = join!(
            self.load_metadata(),
            self.load_categories()
        );
    }
    
    async fn load_metadata(&self) -> Result<(), EmptyError> {
        match api_with_auth::<MetadataResponse, EmptyError, ()>(meta::Get::PATH, meta::Get::METHOD, None).await {
            Err(e) => Err(e),
            Ok(res) => {
                self.affiliations.set(res.affiliations);
                self.age_ranges.set(res.age_ranges);
                self.goals.set(res.goals);
                Ok(())
            },
        }
    }

    async fn load_categories(&self) -> Result<(), EmptyError> {
        let req = GetCategoryRequest {
            ids: Vec::new(), 
            scope: Some(CategoryTreeScope::Decendants)
        };

        match api_with_auth::<CategoryResponse, EmptyError, GetCategoryRequest>(category::Get::PATH, category::Get::METHOD, Some(req)).await {
            Err(e) => Err(e),
            Ok(res) => {
                let mut category_label_lookup = HashMap::new();
                Self::get_categories_labels(&res.categories, &mut category_label_lookup);
                self.category_label_lookup.set(category_label_lookup);
                self.categories.set(res.categories);
                Ok(())
            },
        }
    }

    fn get_categories_labels(categories: &Vec<Category>, lookup: &mut HashMap<CategoryId, String>) {
        for category in categories {
            lookup.insert(category.id.clone(), category.name.clone());
            Self::get_categories_labels(&category.children, lookup);
        }
    }
}
