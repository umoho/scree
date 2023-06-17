pub(crate) trait Color {}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Rgba {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

impl Color for Rgba {}

impl From<u32> for Rgba {
    fn from(value: u32) -> Self {
        Self {
            r: (value << 24) as u8,
            g: (value << 16) as u8,
            b: (value << 8) as u8,
            a: value as u8,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Rgba {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
            a: value.3,
        }
    }
}

impl Into<(u8, u8, u8, u8)> for Rgba {
    fn into(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Tricolor {
    Dark,
    Light,
    Gray,
}

impl Into<u32> for Tricolor {
    fn into(self) -> u32 {
        match self {
            Self::Dark => 0,
            Self::Light => 1,
            Self::Gray => 2,
        }
    }
}

impl Color for Tricolor {}
