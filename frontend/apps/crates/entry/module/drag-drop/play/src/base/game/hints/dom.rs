use super::state::*;
use std::rc::Rc;
use super::state::*;
use components::{
    traces::hints::dom::render_traces_hint,
    stickers::dom::render_stickers_raw
};
use gloo_timers::future::TimeoutFuture;
use dominator::{clone, html, Dom};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
pub fn render(state: Rc<Hints>) -> Dom {

    html!("empty-fragment", {
        .future(clone!(state => async move {
            TimeoutFuture::new(crate::config::HINT_TIME).await;
            state.finish();
        }))
        .child(render_stickers_raw(
            &state.game.base.items
                .iter()
                .map(|item| {
                    item.sticker.clone()
                })
                .collect::<Vec<_>>(),
            state.game.base.theme_id
        ))
        .child(render_traces_hint(
                state.game.base.target_areas
                    .iter()
                    .map(|area| area.trace.clone())
                    .collect()
        ))
    })
}
