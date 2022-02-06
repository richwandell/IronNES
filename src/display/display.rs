use std::cell::RefCell;
use std::rc::Rc;
use glutin_window::GlutinWindow as Window;
use graphics::{Transformed};
use graphics::Context;
use opengl_graphics::{GlGraphics};
use crate::{Cpu, Ppu, State};
use crate::display::{EMU_HEIGHT, EMU_WIDTH};

pub(crate) fn get_scaled_context(c: Context) -> Context {
    let size = c.get_view_size();
    let x_scaler = size[0] / EMU_WIDTH as f64;
    let y_scaler = size[1] / EMU_HEIGHT as f64;
    c.scale(x_scaler, y_scaler)
}

pub struct NesSystem {
    pub(crate) window: Window,
    pub(crate) gl: GlGraphics,
    pub(crate) state: Rc<RefCell<State>>,
    pub(crate) cpu: Rc<RefCell<Cpu>>,
    pub(crate) ppu: Rc<RefCell<Ppu>>
}

pub trait Game {
    fn start(&mut self);
}




