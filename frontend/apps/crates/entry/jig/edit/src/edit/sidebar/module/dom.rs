use dominator::{html, Dom, clone};
use shared::domain::jig::Jig;
use std::rc::Rc;
use crate::edit::sidebar::state::State as SidebarState;
use super::{
    state::*,
    menu::dom::MenuDom,
    actions
};
use futures_signals::signal::SignalExt;
use utils::events;
use shared::domain::jig::{ModuleKind, LiteModule};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

pub struct ModuleDom {
}

impl ModuleDom {
    pub fn render(sidebar_state: Rc<SidebarState>, index: usize, drag_target_index: Option<usize>, total_len:usize, module: Rc<Module>) -> Dom {
        let state = Rc::new(State::new(sidebar_state, index, total_len, module));

        let is_filler = Some(index) == drag_target_index;



        html!("empty-fragment", {
            .property("slot", if index == 0 { "cover-module" } else { "modules" })
            .child(html!("jig-edit-sidebar-filler", {
                .style("display", {
                    if is_filler { "block" } else {"none"}
                })
            }))
            .child( html!("jig-edit-sidebar-module", {
                .future(State::drag_overlap_signal(state.clone()).for_each(clone!(state => move |overlap| {
                    if overlap {
                        state.sidebar.drag_target_index.set(Some(state.index));
                    }
                        //Doing this here instead of immediately on mousemove
                        //gives us a nice separation of concerns
                        //e.g. to throttle
                        //actions::update_index(state.clone(), pos.x, pos.y);
                    async {}
                })))
                .style("display", {
                    if is_filler { "none" } else {"block"}
                })
                .property_signal("module", state.kind_str_signal())
                .property("index", index as u32)
                .property("lastBottomDecoration", index == total_len-1)
                .event(clone!(state => move |evt:events::MouseDown| {
                    actions::mouse_down(state.clone(), evt.x(), evt.y());
                }))
                .child(html!("jig-edit-sidebar-module-window", {
                    .property("slot", "window")
                    .property_signal("state", state.window_state_signal())
                    .event_preventable(clone!(state => move |evt:events::DragOver| {
                        if let Some(data_transfer) = evt.data_transfer() {
                            if data_transfer.types().index_of(&JsValue::from_str("module_kind"), 0) != -1 {
                                if state.module.kind.get().is_none() {
                                    evt.prevent_default();
                                } 
                            }                     }

                    }))
                    .event(clone!(state => move |evt:events::Drop| {
                        if let Some(data_transfer) = evt.data_transfer() {
                            if let Some(module_kind) = data_transfer.get_data("module_kind").ok() { 
                                let kind:ModuleKind = ModuleKind::from_str(&module_kind).unwrap_throw();
                                actions::assign_kind(state.clone(), kind);
                            }
                        }
                    }))
                }))
                .after_inserted(clone!(state => move |dom| {
                    *state.elem.borrow_mut() = Some(dom);
                    //*state.sidebar.drag_targets.borrow_mut().
                }))
                .after_removed(clone!(state => move |dom| {
                    *state.elem.borrow_mut() = None; 
                }))
                .child(MenuDom::render(state.clone()))
                .apply_if(index < total_len-1, |dom| {
                    dom.child(html!("button-icon", {
                        .property("icon", "gears")
                        .property("slot", "add")
                        .event(clone!(state => move |evt:events::Click| {
                            actions::add_module_after(state.clone())
                        }))
                    }))
                })
            }))
        })
    }
}
