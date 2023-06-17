use crate::color::{Color, Rgba, Tricolor};

#[derive(Debug)]
pub(crate) struct Frame<T: Color> {
    pub(crate) res: (u32, u32),
    pub(crate) ctx: Vec<T>,
}

impl Frame<Rgba> {
    #[allow(dead_code)]
    pub(crate) fn new_blank_rgba(res: (u32, u32)) -> Self {
        let mut ctx = Vec::new();
        let (height, weight) = res;
        for _line in 0..height {
            for _column in 0..weight {
                ctx.push((0, 0, 0, 0).into());
            }
        }
        Self { res, ctx }
    }
}

impl Frame<Tricolor> {
    pub(crate) fn new_blank_tricolor(res: (u32, u32)) -> Self {
        let mut ctx = Vec::new();
        let (height, weight) = res;
        for _line in 0..height {
            for _column in 0..weight {
                ctx.push(Tricolor::Dark);
            }
        }
        Self { res, ctx }
    }
}
