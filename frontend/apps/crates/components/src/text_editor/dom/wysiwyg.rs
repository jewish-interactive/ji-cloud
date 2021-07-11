use dominator::{Dom, clone, html};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;
use crate::text_editor::font_css_converter::font_from_css;

use super::super::{
    wysiwyg_types::{WysiwygControlsChange, ControlsChange},
    state::State,
};

pub fn render(state: Rc<State>) -> Dom {
    html!("wysiwyg-base", {
        .property_signal("theme", state.theme_id.signal_cloned().map(|theme_id| {
            theme_id.as_str_id()
        }))
        .after_inserted(clone!(state => move |wysiwyg_ref| {
            state.set_wysiwyg_ref(wysiwyg_ref);
        }))
        .event(clone!(state => move |e: WysiwygControlsChange| {
            let value = e.value();
            // log::info!("{:?}", &value);
            match value {
                ControlsChange::Font(font) => {
                    let mut controls = state.controls.lock_mut();
                    controls.font = font_from_css(&font);
                },
                ControlsChange::Element(element_type) => {
                    let mut controls = state.controls.lock_mut();
                    controls.element = element_type;
                },
                ControlsChange::Weight(weight) => {
                    let mut controls = state.controls.lock_mut();
                    controls.weight = weight;
                },
                ControlsChange::Italic(italic) => {
                    let mut controls = state.controls.lock_mut();
                    controls.italic = italic;
                }
                ControlsChange::Underline(underline) => {
                    let mut controls = state.controls.lock_mut();
                    controls.underline = underline;
                }
                ControlsChange::Align(align) => {
                    let mut controls = state.controls.lock_mut();
                    controls.align = align;
                }
                ControlsChange::FontSize(font_size) => {
                    let mut controls = state.controls.lock_mut();
                    controls.font_size = font_size;
                },
                ControlsChange::Color(color) => {
                    let mut controls = state.controls.lock_mut();
                    controls.color = color;
                },
                ControlsChange::HighlightColor(color) => {
                    let mut controls = state.controls.lock_mut();
                    controls.highlight_color = color;
                },
                ControlsChange::BoxColor(color) => {
                    let mut controls = state.controls.lock_mut();
                    controls.box_color = color;
                },
                ControlsChange::IndentCount(indent_count) => {
                    let mut controls = state.controls.lock_mut();
                    controls.indent_count = indent_count;
                },
            }
        }))
        .event(clone!(state => move |e: events::CustomChange| {
            let value = e.value();
            if let Some(on_change) = &state.callbacks.on_change.as_ref() {
                on_change(&value);
            }
        }))
        .event(clone!(state => move |_: events::CustomBlur| {
            if let Some(on_blur) = &state.callbacks.on_blur.as_ref() {
                on_blur();
            }
        }))
    })
}