use dominator::{html, Dom, DomBuilder, clone};
use crate::data::state::*;
use std::{borrow::BorrowMut, rc::Rc};
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{self, Signal, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::jig::module::body::{Sprite, Transform};
use super::state::*;
use components::{
    transform::{
        dom::TransformDom,
        events::Move as TransformMove,
        state::{TransformState, Action as TransformAction},
    },
    text_editor::dom::render_wysiwyg,
    renderables::text::*,
};
use web_sys::HtmlElement;

//For text, we need to be able to click into the text while the transform is active
//therefore it's a child of the transform
//the transform box itself is only rotated, everything else is done by internal math
//however the text shouldn't really scale either, so we just take the width/height
pub struct TextDom {}
impl TextDom {
    pub fn render(state:Rc<State>, index: ReadOnlyMutable<Option<usize>>, text: Rc<Text>) -> Dom {


        let get_active_signal = || { state.renderables.selected_signal(index.clone()) };
        text.transform.size.set(Some((300.0, 300.0)));
        if *text.is_new.borrow() {
            text.transform.set_to_center();
        }
        
        TransformDom::render_child(
            text.transform.clone(),
            clone!(state, index, text => move || super::menu::dom::render(state.clone(), index.clone(), text.clone())),
            get_active_signal,
            get_active_signal().map(clone!(state, text, index => move |active| {

                *text.transform.hide_on_dbl_click.borrow_mut() = active;


                fn apply_transform<A: AsRef<HtmlElement>>(dom:DomBuilder<A>, transform: &TransformState) -> DomBuilder<A> {
                    dom
                            .style("position", "absolute")
                            .style_signal("width", transform.width_px_signal().map(|x| format!("{}px", x)))
                            .style_signal("height", transform.height_px_signal().map(|x| format!("{}px", x)))
                }

                if active {
                    Some(html!("div", {
                        .style("display", "block")
                        .style("border", "green dashed 1px")
                        .style("box-sizing", "border-box")
                        .style("align-self", "baseline")
                        .apply(|dom| apply_transform(dom, &text.transform))

                        //TODO - set text.rect_hidden to false when wysiwyg is blured
                        .child(render_wysiwyg(state.text_editor.clone()))
                    }))
                } else {
                        if crate::debug::settings().text_mock_box {
                            Some(html!("div", {
                                .text("Hello World!!!")
                                .style("display", "block")
                                .style("background-color", "red")
                                .style("text-align", "center")
                                .apply(|dom| apply_transform(dom, &text.transform))
                            }))
                        } else {
                            Some(html!("wysiwyg-output-renderer", {
                                .property_signal("valueAsString", text.value.signal_cloned())
                                .style("cursor", "pointer") //TODO: move to element
                                .apply(|dom| apply_transform(dom, &text.transform))
                                .event(clone!(index, state, text => move |evt:events::Click| {
                                    if let Some(index) = index.get_cloned() {
                                        let value = text.value.get_cloned();

                                        state.text_editor.set_value(if value.is_empty() { None } else { Some(value) });
                                        state.select_renderable(index);
                                    }
                                }))
                            }))
                        }
                }
        })))
    }
}