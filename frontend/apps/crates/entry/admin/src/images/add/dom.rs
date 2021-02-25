use dominator::{html, Dom, clone};
use web_sys::HtmlInputElement;
use std::rc::Rc;
use super::{actions, state::*};
use utils::events;

pub struct ImageAddPage {
}

impl ImageAddPage {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        html!("image-add-page", {
            .child(html!("button-add", {
                .property("slot", "button")
                .event(clone!(state => move |evt:events::Click| {
                    if let Some(elem) = state.file_input.borrow().as_ref() {
                        elem.click();
                    }
                }))
            }))
            .child(html!("input" => HtmlInputElement, {
                .property("type", "file")
                .property("accept", "image/*")
                .style("display", "none")
                .after_inserted(clone!(state => move |elem| {
                    *state.file_input.borrow_mut() = Some(elem);
                }))
                .event(clone!(state => move |_evt:events::Change| {
                    let file =
                        state.file_input.borrow().as_ref()
                            .and_then(|input| input.files())
                            .and_then(|files| files.get(0));

                    if let Some(file) = file {
                        actions::on_file(state.clone(), file);
                    }
                }))
            }))
            .child(
                html!("window-loader-block", {
                    .property_signal("visible", state.loader.is_loading())
                })
            )
            .event(clone!(state => move |evt:events::CustomChange| {
                actions::on_change(state.clone(), evt.value());
            }))
        })
    }
}

