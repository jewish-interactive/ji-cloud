use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use components::{
    instructions::editor::dom::render as render_instructions,
};

pub fn render(state: Rc<Step3>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), TabKind::Settings),
            render_tab(state.clone(), TabKind::Instructions),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(|tab| {
                    match tab {
                        Tab::Settings(state) => {
                            None
                        },
                        Tab::Instructions(state) => {
                            Some(render_instructions(state.clone()))
                        },
                    }
                }))
            })
        ])
    })
}


fn render_tab(state: Rc<Step3>, tab_kind:TabKind) -> Dom {
    html!("menu-tab-with-title", {
        .property("slot", "tabs")
        .property_signal("active", state.tab.signal_ref(clone!(tab_kind => move |curr| {
            curr.kind() == tab_kind
        })))
        .property("kind", tab_kind.as_str())
        .event(clone!(state, tab_kind => move |evt:events::Click| {
            state.tab.set(Tab::new(state.base.clone(), tab_kind));
        }))
    })
}
