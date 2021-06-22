use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;
use crate::module::{
    _groups::cards::edit::state::*,
    edit::prelude::*,
};

impl <RawData: RawDataExt, E: ExtraExt> DomRenderable for Header<RawData, E> {
    fn render(state: Rc<Header<RawData, E>>) -> Dom {
        html!("empty-fragment")
    }
}
