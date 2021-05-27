use dominator::{html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt, ReadOnlyMutable},
    signal_vec::{SignalVec, SignalVecExt},
};
use crate::traces::{
    svg::{self, ShapeStyle, ShapeStyleBase}, 
    edit::state::*
};
use super::trace::state::*;

use shared::domain::jig::module::body::{Trace as RawTrace, Transform, TraceShape};
use web_sys::HtmlCanvasElement;
use awsm_web::canvas::get_2d_context;
use once_cell::sync::Lazy;
use std::fmt::Write;

pub fn render(state:Rc<Edit>) -> Dom { 

    let mask_children = resize_info_signal()
        .switch_signal_vec(clone!(state => move |resize_info| {
            state.list
                .signal_vec_cloned()
                .map(clone!(resize_info, state => move |trace| {
                    let style = ShapeStyle::new(ShapeStyleBase::Mask);
                    render_trace(&style, &resize_info, &trace, None::<fn()>) 
                }))
        }));

    let click_children = resize_info_signal()
        .switch_signal_vec(clone!(state => move |resize_info| {
            state.list
                .signal_vec_cloned()
                .enumerate()
                .map(clone!(resize_info, state => move |(index, trace)| {
                    let style = ShapeStyle::new(ShapeStyleBase::Transparent);
                    render_trace(&style, &resize_info, &trace, Some(clone!(state, index => move || {
                        if let Some(index) = index.get_cloned() {
                            state.select_index(index);
                        }
                    })))
                }))
        }));
    let menu_children = resize_info_signal()
        .switch_signal_vec(clone!(state => move |resize_info| {
            state.list
                .signal_vec_cloned()
                .enumerate()
                .map(clone!(state, resize_info => move |(index, trace)| {
                    super::select_box::dom::render(state.clone(), index, &trace, &resize_info)
                }))
        }));
    html!("empty-fragment", {
        .child(
            svg::render_masks(
                mask_children,
                click_children,
                clone!(state => move |x, y| {
                    //TODO - detect if area selected
                    Edit::start_new_trace(state.clone(), None, Some((x, y)));
                }),
                clone!(state => move |x, y| {
                }),
                clone!(state => move |x, y| {
                }),
            )
        )
        .children_signal_vec(menu_children)
    })
}

pub fn render_trace(style: &ShapeStyle, resize_info:&ResizeInfo, trace:&AllTrace, on_select: Option<impl Fn() + Clone + 'static>) -> Dom {
    let transform_size = Some((&trace.transform, trace.size.clone())); 


    match trace.shape {

        TraceShape::Path(ref path) => {
            svg::render_path(&style, &resize_info, transform_size, &path, on_select)
        },

        TraceShape::Rect(width, height) => {
            svg::render_rect(&style, &resize_info, transform_size, width, height, on_select)
        }
        TraceShape::Ellipse(radius_x, radius_y) => {
            svg::render_ellipse(&style, &resize_info, transform_size, radius_x, radius_y, on_select)
        }
    }
}