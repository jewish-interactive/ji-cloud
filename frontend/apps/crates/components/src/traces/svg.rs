use dominator::{DomBuilder, html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Signal, Mutable, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use web_sys::HtmlCanvasElement;
use awsm_web::canvas::get_2d_context;
use once_cell::sync::Lazy;
use std::fmt::Write;
use shared::domain::jig::module::body::Transform;

static SVG_CLASS: Lazy<String> = Lazy::new(|| class! {
    .style("position", "absolute")
    .style("top", "0")
});
static BG_CLASS: Lazy<String> = Lazy::new(|| class! {
    .style("fill", "black")
    .style("fill-opacity", "0.5")
});

static MASK_RECT_CLASS: Lazy<String> = Lazy::new(|| class! {
    .style("fill", "white")
});
static SHAPE_MASK_CLASS: Lazy<String> = Lazy::new(|| class! {
    .style("fill", "black")
    .style("cursor", "pointer")
});

static SHAPE_SHADOW_CLASS: Lazy<String> = Lazy::new(|| class! {
    .style("fill", "blue")
    .style("cursor", "pointer")
});

static SHAPE_TRANSPARENT_CLASS: Lazy<String> = Lazy::new(|| class! {
    .style("fill-opacity", "0")
    .style("cursor", "pointer")
});

pub struct ShapeStyle {
    pub base: ShapeStyleBase
}

pub enum ShapeStyleBase {
    Mask,
    Shadow,
    Transparent,
}

impl ShapeStyle {
    pub fn new(base: ShapeStyleBase) -> Self {
        Self {
            base
        }
    }

    pub fn class(&self) -> &'static str {
        match self.base {
            ShapeStyleBase::Mask => &SHAPE_MASK_CLASS,
            ShapeStyleBase::Shadow => &SHAPE_SHADOW_CLASS,
            ShapeStyleBase::Transparent => &SHAPE_TRANSPARENT_CLASS,
        }
    }
}

pub fn render_masks<ChildrenMaskSignal, ChildrenDrawSignal, OnMouseDownFn, OnMouseUpFn, OnMouseMoveFn>(
    children_mask: ChildrenMaskSignal,
    children_draw: ChildrenDrawSignal,
    on_mouse_down:OnMouseDownFn,
    on_mouse_up:OnMouseUpFn,
    on_mouse_move:OnMouseMoveFn,
) -> Dom 
where 
    ChildrenMaskSignal: SignalVec<Item = Dom> + 'static,
    ChildrenDrawSignal: SignalVec<Item = Dom> + 'static,
    OnMouseDownFn: Fn(i32, i32) + Clone + 'static,
    OnMouseUpFn: Fn(i32, i32) + Clone + 'static,
    OnMouseMoveFn: Fn(i32, i32) + Clone + 'static,

{
    svg!("svg", {
        .class(&*SVG_CLASS)
        .attribute_signal("width", resize_info_signal().map(|info| {
            format!("{}px", info.width)
        }))
        .attribute_signal("height", resize_info_signal().map(|info| {
            format!("{}px", info.height)
        }))
        .child(svg!("rect", {
            .attribute("mask", "url(#maskPath)")
            .attribute("x", "0")
            .attribute("y", "0")
            .attribute_signal("width", resize_info_signal().map(|info| {
                format!("{}px", info.width)
            }))
            .attribute_signal("height", resize_info_signal().map(|info| {
                format!("{}px", info.height)
            }))
            .class(&*BG_CLASS)
            .event(clone!(on_mouse_down => move |evt:events::MouseDown| {
                on_mouse_down(evt.x() as i32, evt.y() as i32);
            }))
        }))
        .child(svg!("defs", {
            .child(svg!("mask", {
                .attribute("id", "maskPath")
                .child(svg!("rect", {
                    .attribute("x", "0")
                    .attribute("y", "0")
                    .attribute_signal("width", resize_info_signal().map(|info| {
                        format!("{}px", info.width)
                    }))
                    .attribute_signal("height", resize_info_signal().map(|info| {
                        format!("{}px", info.height)
                    }))
                    .class(&*MASK_RECT_CLASS)
                }))
                .children_signal_vec(children_mask)
            }))
        }))
        .children_signal_vec(children_draw)

        .global_event_preventable(clone!(on_mouse_up => move |evt:events::MouseUp| {
            on_mouse_up(evt.x() as i32, evt.y() as i32);
        }))
        .global_event_preventable(clone!(on_mouse_move => move |evt:events::MouseMove| {
            on_mouse_move(evt.x() as i32, evt.y() as i32);
        }))
    })
}

/*
pub fn render_simple<ChildrenSignal>(
    children: ChildrenSignal,
) -> Dom 
where 
    ChildrenSignal: SignalVec<Item = Dom> + 'static,

{
    svg!("svg", {
        .class(&*SVG_CLASS)
        .attribute_signal("width", resize_info_signal().map(|info| {
            format!("{}px", info.width)
        }))
        .attribute_signal("height", resize_info_signal().map(|info| {
            format!("{}px", info.height)
        }))
        .children_signal_vec(children)
    })
}
*/

fn apply_transform<A: AsRef<web_sys::Element>>(dom:DomBuilder<A>, resize_info: &ResizeInfo, transform_size: Option<(&Transform, (f64, f64))>) -> DomBuilder<A> {
    dom.apply_if(transform_size.is_some(), |dom| {
        let (transform, size) = transform_size.unwrap_ji();
        let (width, height) = resize_info.get_size_px(size.0, size.1);
        let style = format!("transform: {}; transform-origin: {}px {}px;width: {}px; height: {}px;", transform.denormalize_matrix_string(&resize_info), width/2.0, height/2.0, width, height);

        dom.attribute("style", &style)
    })
}
pub fn render_path_signal(shape_style: &ShapeStyle, resize_info: ResizeInfo, transform_size: Option<(&Transform, (f64, f64))>, points: &Mutable<Vec<(f64, f64)>>) -> Dom {
    let path_string = 
        points.signal_ref(clone!(resize_info => move |points| {
            if points.len() < 2 {
                String::from("M 0 0")
            } else {
                path_to_string(
                    points 
                        .iter()
                        .map(|(x, y)| {
                            resize_info.get_pos_denormalized(*x, *y)
                        })
                )
            }
        }));

    svg!("path", {
        .class(shape_style.class())
        .attribute_signal("d", path_string)
        .apply(|dom| apply_transform(dom, &resize_info, transform_size))
    })
}
pub fn render_path(shape_style: &ShapeStyle, resize_info: &ResizeInfo, transform_size: Option<(&Transform, (f64, f64))>, points: &[(f64, f64)], on_select: Option<impl Fn() + Clone + 'static>) -> Dom {
    let path_string = {
        if points.len() < 2 {
            String::from("M 0 0")
        } else {
            path_to_string(
                points 
                    .iter()
                    .map(|(x, y)| {
                        resize_info.get_pos_denormalized(*x, *y)
                    })
            )
        }
    };

    svg!("path", {
        .class(shape_style.class())
        .attribute("d", &path_string)
        .apply(|dom| apply_transform(dom, resize_info, transform_size))
        .event(clone!(on_select => move |evt:events::Click| {
            if let Some(on_select) = &on_select {
                (on_select)();
            }
        }))
    })
}


pub fn render_rect(shape_style: &ShapeStyle, resize_info: &ResizeInfo, transform_size: Option<(&Transform, (f64, f64))>, width: f64, height: f64, on_select: Option<impl Fn() + Clone + 'static>) -> Dom {

    let (width, height) = resize_info.get_pos_denormalized(width, height);

    svg!("rect", {
        .class(shape_style.class())
        .attribute("width", &format!("{}px", width))
        .attribute("height", &format!("{}px", height))
        .apply(|dom| apply_transform(dom, resize_info, transform_size))
        .event(clone!(on_select => move |evt:events::Click| {
            if let Some(on_select) = &on_select {
                (on_select)();
            }
        }))
    })

}

pub fn render_ellipse(shape_style: &ShapeStyle, resize_info: &ResizeInfo, transform_size: Option<(&Transform, (f64, f64))>, radius_x: f64, radius_y: f64, on_select: Option<impl Fn() + Clone + 'static>) -> Dom {

    let (radius_x, radius_y) = resize_info.get_pos_denormalized(radius_x, radius_y);

    svg!("ellipse", {
        .class(shape_style.class())
        .attribute("cx", &format!("{}px", radius_x))
        .attribute("cy", &format!("{}px", radius_y))
        .attribute("rx", &format!("{}px", radius_x))
        .attribute("ry", &format!("{}px", radius_y))
        .apply(|dom| apply_transform(dom, resize_info, transform_size))
        .event(clone!(on_select => move |evt:events::Click| {
            if let Some(on_select) = &on_select {
                (on_select)();
            }
        }))
    })

}
fn path_to_string(path:impl Iterator<Item = (f64, f64)>) -> String {
    let mut output = String::from("M");
    for (index, (x, y)) in path.enumerate() {
        write!(&mut output, " {} {}", x, y).unwrap_ji();
    }

    output.push_str(" Z");

    output
}