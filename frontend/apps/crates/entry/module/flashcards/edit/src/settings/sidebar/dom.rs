use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::{
    state::*,
    settings::state::*
};
use super::state::*;
use utils::prelude::*;
use shared::domain::jig::module::body::flashcards::DisplayMode;

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    html!("flashcards-settings", {
        .child(render_display_mode(state.clone(), DisplayMode::Single))
        .child(render_display_mode(state.clone(), DisplayMode::Double))
    })
}
pub fn render_display_mode(state: Rc<SidebarSettings>, display_mode:DisplayMode) -> Dom {
    html!("flashcards-settings-option", {
        .property("mode", display_mode.as_str_id())
        .property_signal("selected", state.base.extra.settings.display_mode.signal_ref(move |curr| {
            *curr == display_mode
        }))
        .event(clone!(state => move |evt:events::Click| {
            state.set_display_mode(display_mode); 
        }))
    })

}