use std::cell::RefCell;
use std::rc::Rc;
use crate::steps::state::Base;
use shared::domain::jig::module::body::tapping_board::Next;
pub const DEFAULT_SELECT_AMOUNT:usize = 3;

pub struct State {
    pub base: Rc<Base>,
    pub some_amount: RefCell<usize>
}

impl State {
    pub fn new(base:Rc<Base>) -> Self {
        let some_amount = RefCell::new(
            match base.play_settings.next.get_cloned() {
                Next::SelectSome(value) => value,
                _ => DEFAULT_SELECT_AMOUNT
            }
        );

        Self {
            base,
            some_amount
        }
    }
}