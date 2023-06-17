use crate::{color::Color, frame::Frame};

#[derive(Debug)]
pub(crate) struct Drawer<T: Color> {
    pub(crate) frame: Frame<T>,
}

impl<T: Color> Drawer<T> {
    pub(crate) fn new(frame: Frame<T>) -> Self {
        Self { frame }
    }

    pub(crate) fn draw_pixel(&mut self, pos: (u32, u32), color: impl Into<T>) {
        let color = color.into();

        let (_h, w) = self.frame.res;
        let (x, y) = pos;

        let step = y * w + x;

        self.frame.ctx[step as usize] = color;
    }
}
