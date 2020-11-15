use core::routes::{Route, JigRoute, JigPlayMode};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};
use crate::pages::{
    gallery::GalleryPage,
    edit::EditPage
};

pub struct Router {
}

impl Router {
    pub fn new() -> Self {
        Self { }
    }

    fn signal() -> impl Signal<Item = Route> {
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        Self::signal()
            .map(|route| {
                match route {
                    Route::Jig(route) => {
                        match route {
                            JigRoute::Gallery => Some(GalleryPage::render(GalleryPage::new())),
                            JigRoute::Edit(jig_id, module_id) => Some(EditPage::render(EditPage::new(jig_id, module_id))),
                            _ => None
                        }
                    },
                    _ => None
                }
            })
    }
    
    pub fn render(&self) -> Dom {
        html!("main", { .child_signal(Self::dom_signal()) } )
    }
}
