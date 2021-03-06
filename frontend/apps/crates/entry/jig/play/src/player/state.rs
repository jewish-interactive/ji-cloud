use std::{cell::RefCell, rc::Rc};

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::{Jig, JigId, module::ModuleId};
use serde::{Serialize, Deserialize};
use web_sys::HtmlIFrameElement;

use super::timer::Timer;

pub struct State {
    pub is_teacher: bool,
    pub jig_id: JigId,
    pub jig: Mutable<Option<Jig>>,
    pub loader: AsyncLoader,
    pub active_module: Mutable<usize>,
    pub module_id: Mutable<Option<ModuleId>>, // needed?
    pub timer: Mutable<Option<Timer>>,
    pub points: Mutable<u32>,
    pub iframe: Rc<RefCell<Option<HtmlIFrameElement>>>,

}

impl State {
    pub fn new(jig_id: JigId, module_id: Option<ModuleId>) -> Self {
        Self {
            is_teacher: true,
            jig_id,
            jig: Mutable::new(None),
            loader: AsyncLoader::new(),
            active_module: Mutable::new(0),
            module_id: Mutable::new(None),
            timer: Mutable::new(None),
            points: Mutable::new(0),
            // background_music: Mutable::new(None),
            iframe: Rc::new(RefCell::new(None)),
        }
    }
}
