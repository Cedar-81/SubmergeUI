#[derive(Debug)]
pub enum SubmergeText {
    XS { size: f32 },
    SM { size: f32 },
    BASE { size: f32 },
    LG { size: f32 },
    XL { size: f32 },
    XL2 { size: f32 },
    XL3 { size: f32 },
    XL4 { size: f32 },
    XL5 { size: f32 },
    XL6 { size: f32 },
    XL7 { size: f32 },
    XL8 { size: f32 },
    XL9 { size: f32 },
}

pub const BASE_FONT_SIZE: f32 = 20.0;

#[macro_export]
macro_rules! define_text_sizes {
    ($($name:ident => $size:expr),*) => {
        impl SubmergeText {
            $(
                pub const $name: SubmergeText = SubmergeText::$name {size: $size};
            )*
        }
    };
}

define_text_sizes! {
    XS => 0.75 * BASE_FONT_SIZE,
    SM => 0.875 * BASE_FONT_SIZE,
    BASE => 1.0 * BASE_FONT_SIZE,
    LG => 1.125 * BASE_FONT_SIZE,
    XL => 1.25 * BASE_FONT_SIZE,
    XL2 => 1.5 * BASE_FONT_SIZE,
    XL3 => 1.875 * BASE_FONT_SIZE,
    XL4 => 2.25 * BASE_FONT_SIZE,
    XL5 => 3.0 * BASE_FONT_SIZE,
    XL6 => 3.75 * BASE_FONT_SIZE,
    XL7 => 4.5 * BASE_FONT_SIZE,
    XL8 => 6.0 * BASE_FONT_SIZE,
    XL9 => 8.0 * BASE_FONT_SIZE
}

impl SubmergeText {
    pub const fn get_size(&self) -> f32 {
        match self {
            SubmergeText::XS { size } => *size,
            SubmergeText::SM { size } => *size,
            SubmergeText::BASE { size } => *size,
            SubmergeText::LG { size } => *size,
            SubmergeText::XL { size } => *size,
            SubmergeText::XL2 { size } => *size,
            SubmergeText::XL3 { size } => *size,
            SubmergeText::XL4 { size } => *size,
            SubmergeText::XL5 { size } => *size,
            SubmergeText::XL6 { size } => *size,
            SubmergeText::XL7 { size } => *size,
            SubmergeText::XL8 { size } => *size,
            SubmergeText::XL9 { size } => *size,
        }
    }

    pub fn get_size_from_text_style(style: &str) -> f32 {
        if style == "text-xs" {
            SubmergeText::get_size(&SubmergeText::XS)
        } else if style == "text-sm" {
            SubmergeText::get_size(&SubmergeText::SM)
        } else if style == "text-base" {
            SubmergeText::get_size(&SubmergeText::BASE)
        } else if style == "text-lg" {
            SubmergeText::get_size(&SubmergeText::LG)
        } else if style == "text-xl" {
            SubmergeText::get_size(&SubmergeText::XL)
        } else if style == "text-2xl" {
            SubmergeText::get_size(&SubmergeText::XL2)
        } else if style == "text-3xl" {
            SubmergeText::get_size(&SubmergeText::XL3)
        } else if style == "text-4xl" {
            SubmergeText::get_size(&SubmergeText::XL4)
        } else if style == "text-5xl" {
            SubmergeText::get_size(&SubmergeText::XL5)
        } else if style == "text-6xl" {
            SubmergeText::get_size(&SubmergeText::XL6)
        } else if style == "text-7xl" {
            SubmergeText::get_size(&SubmergeText::XL7)
        } else if style == "text-8xl" {
            SubmergeText::get_size(&SubmergeText::XL8)
        } else if style == "text-9xl" {
            SubmergeText::get_size(&SubmergeText::XL9)
        } else {
            SubmergeText::get_size(&SubmergeText::BASE)
        }
    }

    pub const fn text(self) -> f32 {
        self.get_size()
    }
}
