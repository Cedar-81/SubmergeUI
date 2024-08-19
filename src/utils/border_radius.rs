use bevy::ui::Val;

use super::font_size::BASE_FONT_SIZE;

#[derive(Debug)]
pub enum SubmergeBR {
    NONE { radius: f32 },
    SM { radius: f32 },
    DEFAULT { radius: f32 },
    MD { radius: f32 },
    LG { radius: f32 },
    XL { radius: f32 },
    XL2 { radius: f32 },
    XL3 { radius: f32 },
    FULL { radius: f32 },
}

#[macro_export]
macro_rules! define_border_radius {
    ($($name:ident => $radius:expr),*) => {
        impl SubmergeBR {
            $(
                pub const $name: SubmergeBR = SubmergeBR::$name { radius: $radius };
            )*
        }
    };
}

define_border_radius! {
    NONE => 0.0 * BASE_FONT_SIZE,
    SM => 0.125 * BASE_FONT_SIZE,
    DEFAULT => 0.25 * BASE_FONT_SIZE,
    MD => 0.375 * BASE_FONT_SIZE,
    LG => 0.5 * BASE_FONT_SIZE,
    XL => 0.75 * BASE_FONT_SIZE,
    XL2 => 1.0 * BASE_FONT_SIZE,
    XL3 => 1.5 * BASE_FONT_SIZE,
    FULL => 9999.0
}

impl SubmergeBR {
    pub const fn get_radius(&self) -> f32 {
        match self {
            SubmergeBR::NONE { radius } => *radius,
            SubmergeBR::SM { radius } => *radius,
            SubmergeBR::DEFAULT { radius } => *radius,
            SubmergeBR::MD { radius } => *radius,
            SubmergeBR::LG { radius } => *radius,
            SubmergeBR::XL { radius } => *radius,
            SubmergeBR::XL2 { radius } => *radius,
            SubmergeBR::XL3 { radius } => *radius,
            SubmergeBR::FULL { radius } => *radius,
        }
    }

    pub const fn radius(self) -> Val {
        Val::Px(self.get_radius())
    }
}
