use crate::{color::Tricolor, frame::Frame};

pub(crate) struct ConsoleScreen {
    res: (u32, u32),
    display: String,
}

impl ConsoleScreen {
    pub(crate) fn new(res: (u32, u32)) -> Self {
        Self {
            res,
            display: String::new(),
        }
    }

    pub(crate) fn show(&mut self, frame: impl Into<Frame<Tricolor>>) {
        let frame: Frame<Tricolor> = frame.into();
        let (_height, weight) = self.res;

        for i in 0..frame.ctx.len() {
            if i % weight as usize == 0 {
                self.display += "\n";
            }

            let color: Tricolor = frame.ctx[i];

            let pixel = match color.into() {
                0 => "  ",
                1 => "# ",
                _ => ". ",
            };
            self.display += pixel;
        }
        println!("{}", self.display);
    }

    pub(crate) fn refresh(&mut self) {
        self.display.clear();

        let clear_screen = "\x1bc";
        print!("{}", clear_screen);
    }
}
