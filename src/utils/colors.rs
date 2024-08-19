use bevy::{color::Color, prelude::Component};

#[derive(Debug, Component)]
pub enum SubmergeColors {
    BLACK { r: f32, g: f32, b: f32 },
    WHITE { r: f32, g: f32, b: f32 },
    SLATE50 { r: f32, g: f32, b: f32 },
    SLATE100 { r: f32, g: f32, b: f32 },
    SLATE200 { r: f32, g: f32, b: f32 },
    SLATE300 { r: f32, g: f32, b: f32 },
    SLATE400 { r: f32, g: f32, b: f32 },
    SLATE500 { r: f32, g: f32, b: f32 },
    SLATE600 { r: f32, g: f32, b: f32 },
    SLATE700 { r: f32, g: f32, b: f32 },
    SLATE800 { r: f32, g: f32, b: f32 },
    SLATE900 { r: f32, g: f32, b: f32 },
    GRAY50 { r: f32, g: f32, b: f32 },
    GRAY100 { r: f32, g: f32, b: f32 },
    GRAY200 { r: f32, g: f32, b: f32 },
    GRAY300 { r: f32, g: f32, b: f32 },
    GRAY400 { r: f32, g: f32, b: f32 },
    GRAY500 { r: f32, g: f32, b: f32 },
    GRAY600 { r: f32, g: f32, b: f32 },
    GRAY700 { r: f32, g: f32, b: f32 },
    GRAY800 { r: f32, g: f32, b: f32 },
    GRAY900 { r: f32, g: f32, b: f32 },
    ZINC50 { r: f32, g: f32, b: f32 },
    ZINC100 { r: f32, g: f32, b: f32 },
    ZINC200 { r: f32, g: f32, b: f32 },
    ZINC300 { r: f32, g: f32, b: f32 },
    ZINC400 { r: f32, g: f32, b: f32 },
    ZINC500 { r: f32, g: f32, b: f32 },
    ZINC600 { r: f32, g: f32, b: f32 },
    ZINC700 { r: f32, g: f32, b: f32 },
    ZINC800 { r: f32, g: f32, b: f32 },
    ZINC900 { r: f32, g: f32, b: f32 },
    NEUTRAL50 { r: f32, g: f32, b: f32 },
    NEUTRAL100 { r: f32, g: f32, b: f32 },
    NEUTRAL200 { r: f32, g: f32, b: f32 },
    NEUTRAL300 { r: f32, g: f32, b: f32 },
    NEUTRAL400 { r: f32, g: f32, b: f32 },
    NEUTRAL500 { r: f32, g: f32, b: f32 },
    NEUTRAL600 { r: f32, g: f32, b: f32 },
    NEUTRAL700 { r: f32, g: f32, b: f32 },
    NEUTRAL800 { r: f32, g: f32, b: f32 },
    NEUTRAL900 { r: f32, g: f32, b: f32 },
    STONE50 { r: f32, g: f32, b: f32 },
    STONE100 { r: f32, g: f32, b: f32 },
    STONE200 { r: f32, g: f32, b: f32 },
    STONE300 { r: f32, g: f32, b: f32 },
    STONE400 { r: f32, g: f32, b: f32 },
    STONE500 { r: f32, g: f32, b: f32 },
    STONE600 { r: f32, g: f32, b: f32 },
    STONE700 { r: f32, g: f32, b: f32 },
    STONE800 { r: f32, g: f32, b: f32 },
    STONE900 { r: f32, g: f32, b: f32 },
    RED50 { r: f32, g: f32, b: f32 },
    RED100 { r: f32, g: f32, b: f32 },
    RED200 { r: f32, g: f32, b: f32 },
    RED300 { r: f32, g: f32, b: f32 },
    RED400 { r: f32, g: f32, b: f32 },
    RED500 { r: f32, g: f32, b: f32 },
    RED600 { r: f32, g: f32, b: f32 },
    RED700 { r: f32, g: f32, b: f32 },
    RED800 { r: f32, g: f32, b: f32 },
    RED900 { r: f32, g: f32, b: f32 },
    ORANGE50 { r: f32, g: f32, b: f32 },
    ORANGE100 { r: f32, g: f32, b: f32 },
    ORANGE200 { r: f32, g: f32, b: f32 },
    ORANGE300 { r: f32, g: f32, b: f32 },
    ORANGE400 { r: f32, g: f32, b: f32 },
    ORANGE500 { r: f32, g: f32, b: f32 },
    ORANGE600 { r: f32, g: f32, b: f32 },
    ORANGE700 { r: f32, g: f32, b: f32 },
    ORANGE800 { r: f32, g: f32, b: f32 },
    ORANGE900 { r: f32, g: f32, b: f32 },
    AMBER50 { r: f32, g: f32, b: f32 },
    AMBER100 { r: f32, g: f32, b: f32 },
    AMBER200 { r: f32, g: f32, b: f32 },
    AMBER300 { r: f32, g: f32, b: f32 },
    AMBER400 { r: f32, g: f32, b: f32 },
    AMBER500 { r: f32, g: f32, b: f32 },
    AMBER600 { r: f32, g: f32, b: f32 },
    AMBER700 { r: f32, g: f32, b: f32 },
    AMBER800 { r: f32, g: f32, b: f32 },
    AMBER900 { r: f32, g: f32, b: f32 },
    YELLOW50 { r: f32, g: f32, b: f32 },
    YELLOW100 { r: f32, g: f32, b: f32 },
    YELLOW200 { r: f32, g: f32, b: f32 },
    YELLOW300 { r: f32, g: f32, b: f32 },
    YELLOW400 { r: f32, g: f32, b: f32 },
    YELLOW500 { r: f32, g: f32, b: f32 },
    YELLOW600 { r: f32, g: f32, b: f32 },
    YELLOW700 { r: f32, g: f32, b: f32 },
    YELLOW800 { r: f32, g: f32, b: f32 },
    YELLOW900 { r: f32, g: f32, b: f32 },
    LIME50 { r: f32, g: f32, b: f32 },
    LIME100 { r: f32, g: f32, b: f32 },
    LIME200 { r: f32, g: f32, b: f32 },
    LIME300 { r: f32, g: f32, b: f32 },
    LIME400 { r: f32, g: f32, b: f32 },
    LIME500 { r: f32, g: f32, b: f32 },
    LIME600 { r: f32, g: f32, b: f32 },
    LIME700 { r: f32, g: f32, b: f32 },
    LIME800 { r: f32, g: f32, b: f32 },
    LIME900 { r: f32, g: f32, b: f32 },
    GREEN50 { r: f32, g: f32, b: f32 },
    GREEN100 { r: f32, g: f32, b: f32 },
    GREEN200 { r: f32, g: f32, b: f32 },
    GREEN300 { r: f32, g: f32, b: f32 },
    GREEN400 { r: f32, g: f32, b: f32 },
    GREEN500 { r: f32, g: f32, b: f32 },
    GREEN600 { r: f32, g: f32, b: f32 },
    GREEN700 { r: f32, g: f32, b: f32 },
    GREEN800 { r: f32, g: f32, b: f32 },
    GREEN900 { r: f32, g: f32, b: f32 },
    EMERALD50 { r: f32, g: f32, b: f32 },
    EMERALD100 { r: f32, g: f32, b: f32 },
    EMERALD200 { r: f32, g: f32, b: f32 },
    EMERALD300 { r: f32, g: f32, b: f32 },
    EMERALD400 { r: f32, g: f32, b: f32 },
    EMERALD500 { r: f32, g: f32, b: f32 },
    EMERALD600 { r: f32, g: f32, b: f32 },
    EMERALD700 { r: f32, g: f32, b: f32 },
    EMERALD800 { r: f32, g: f32, b: f32 },
    EMERALD900 { r: f32, g: f32, b: f32 },
    TEAL50 { r: f32, g: f32, b: f32 },
    TEAL100 { r: f32, g: f32, b: f32 },
    TEAL200 { r: f32, g: f32, b: f32 },
    TEAL300 { r: f32, g: f32, b: f32 },
    TEAL400 { r: f32, g: f32, b: f32 },
    TEAL500 { r: f32, g: f32, b: f32 },
    TEAL600 { r: f32, g: f32, b: f32 },
    TEAL700 { r: f32, g: f32, b: f32 },
    TEAL800 { r: f32, g: f32, b: f32 },
    TEAL900 { r: f32, g: f32, b: f32 },
    CYAN50 { r: f32, g: f32, b: f32 },
    CYAN100 { r: f32, g: f32, b: f32 },
    CYAN200 { r: f32, g: f32, b: f32 },
    CYAN300 { r: f32, g: f32, b: f32 },
    CYAN400 { r: f32, g: f32, b: f32 },
    CYAN500 { r: f32, g: f32, b: f32 },
    CYAN600 { r: f32, g: f32, b: f32 },
    CYAN700 { r: f32, g: f32, b: f32 },
    CYAN800 { r: f32, g: f32, b: f32 },
    CYAN900 { r: f32, g: f32, b: f32 },
    SKY50 { r: f32, g: f32, b: f32 },
    SKY100 { r: f32, g: f32, b: f32 },
    SKY200 { r: f32, g: f32, b: f32 },
    SKY300 { r: f32, g: f32, b: f32 },
    SKY400 { r: f32, g: f32, b: f32 },
    SKY500 { r: f32, g: f32, b: f32 },
    SKY600 { r: f32, g: f32, b: f32 },
    SKY700 { r: f32, g: f32, b: f32 },
    SKY800 { r: f32, g: f32, b: f32 },
    SKY900 { r: f32, g: f32, b: f32 },
    BLUE50 { r: f32, g: f32, b: f32 },
    BLUE100 { r: f32, g: f32, b: f32 },
    BLUE200 { r: f32, g: f32, b: f32 },
    BLUE300 { r: f32, g: f32, b: f32 },
    BLUE400 { r: f32, g: f32, b: f32 },
    BLUE500 { r: f32, g: f32, b: f32 },
    BLUE600 { r: f32, g: f32, b: f32 },
    BLUE700 { r: f32, g: f32, b: f32 },
    BLUE800 { r: f32, g: f32, b: f32 },
    BLUE900 { r: f32, g: f32, b: f32 },
    INDIGO50 { r: f32, g: f32, b: f32 },
    INDIGO100 { r: f32, g: f32, b: f32 },
    INDIGO200 { r: f32, g: f32, b: f32 },
    INDIGO300 { r: f32, g: f32, b: f32 },
    INDIGO400 { r: f32, g: f32, b: f32 },
    INDIGO500 { r: f32, g: f32, b: f32 },
    INDIGO600 { r: f32, g: f32, b: f32 },
    INDIGO700 { r: f32, g: f32, b: f32 },
    INDIGO800 { r: f32, g: f32, b: f32 },
    INDIGO900 { r: f32, g: f32, b: f32 },
    VIOLET50 { r: f32, g: f32, b: f32 },
    VIOLET100 { r: f32, g: f32, b: f32 },
    VIOLET200 { r: f32, g: f32, b: f32 },
    VIOLET300 { r: f32, g: f32, b: f32 },
    VIOLET400 { r: f32, g: f32, b: f32 },
    VIOLET500 { r: f32, g: f32, b: f32 },
    VIOLET600 { r: f32, g: f32, b: f32 },
    VIOLET700 { r: f32, g: f32, b: f32 },
    VIOLET800 { r: f32, g: f32, b: f32 },
    VIOLET900 { r: f32, g: f32, b: f32 },
    PURPLE50 { r: f32, g: f32, b: f32 },
    PURPLE100 { r: f32, g: f32, b: f32 },
    PURPLE200 { r: f32, g: f32, b: f32 },
    PURPLE300 { r: f32, g: f32, b: f32 },
    PURPLE400 { r: f32, g: f32, b: f32 },
    PURPLE500 { r: f32, g: f32, b: f32 },
    PURPLE600 { r: f32, g: f32, b: f32 },
    PURPLE700 { r: f32, g: f32, b: f32 },
    PURPLE800 { r: f32, g: f32, b: f32 },
    PURPLE900 { r: f32, g: f32, b: f32 },
    FUCHSIA50 { r: f32, g: f32, b: f32 },
    FUCHSIA100 { r: f32, g: f32, b: f32 },
    FUCHSIA200 { r: f32, g: f32, b: f32 },
    FUCHSIA300 { r: f32, g: f32, b: f32 },
    FUCHSIA400 { r: f32, g: f32, b: f32 },
    FUCHSIA500 { r: f32, g: f32, b: f32 },
    FUCHSIA600 { r: f32, g: f32, b: f32 },
    FUCHSIA700 { r: f32, g: f32, b: f32 },
    FUCHSIA800 { r: f32, g: f32, b: f32 },
    FUCHSIA900 { r: f32, g: f32, b: f32 },
    PINK50 { r: f32, g: f32, b: f32 },
    PINK100 { r: f32, g: f32, b: f32 },
    PINK200 { r: f32, g: f32, b: f32 },
    PINK300 { r: f32, g: f32, b: f32 },
    PINK400 { r: f32, g: f32, b: f32 },
    PINK500 { r: f32, g: f32, b: f32 },
    PINK600 { r: f32, g: f32, b: f32 },
    PINK700 { r: f32, g: f32, b: f32 },
    PINK800 { r: f32, g: f32, b: f32 },
    PINK900 { r: f32, g: f32, b: f32 },
    ROSE50 { r: f32, g: f32, b: f32 },
    ROSE100 { r: f32, g: f32, b: f32 },
    ROSE200 { r: f32, g: f32, b: f32 },
    ROSE300 { r: f32, g: f32, b: f32 },
    ROSE400 { r: f32, g: f32, b: f32 },
    ROSE500 { r: f32, g: f32, b: f32 },
    ROSE600 { r: f32, g: f32, b: f32 },
    ROSE700 { r: f32, g: f32, b: f32 },
    ROSE800 { r: f32, g: f32, b: f32 },
    ROSE900 { r: f32, g: f32, b: f32 },
}

#[macro_export]
macro_rules! define_colors {
    ($($name:ident => $r:expr, $g:expr, $b:expr),*) => {
        impl SubmergeColors {
            $(
                pub const $name: SubmergeColors = SubmergeColors::$name { r: $r, g: $g, b: $b };
            )*
        }
    };
}

const COLOR_SCALE: f32 = 255.0;

define_colors! {
    BLACK => 0.0, 0.0, 0.0,
    WHITE => 255.0 / COLOR_SCALE, 255.0 / COLOR_SCALE, 255.0 / COLOR_SCALE,
    SLATE50=> 248.0 / COLOR_SCALE, 250.0 / COLOR_SCALE, 252.0 / COLOR_SCALE,
    SLATE100 => 241.0 / COLOR_SCALE, 245.0 / COLOR_SCALE, 249.0 / COLOR_SCALE,
    SLATE200 => 226.0 / COLOR_SCALE, 232.0 / COLOR_SCALE, 240.0 / COLOR_SCALE,
    SLATE300 => 203.0 / COLOR_SCALE, 213.0 / COLOR_SCALE, 225.0 / COLOR_SCALE,
    SLATE400 => 148.0 / COLOR_SCALE, 163.0 / COLOR_SCALE, 184.0 / COLOR_SCALE,
    SLATE500 => 100.0 / COLOR_SCALE, 116.0 / COLOR_SCALE, 139.0 / COLOR_SCALE,
    SLATE600 => 71.0 / COLOR_SCALE, 85.0 / COLOR_SCALE, 105.0 / COLOR_SCALE,
    SLATE700 => 51.0 / COLOR_SCALE, 65.0 / COLOR_SCALE, 85.0 / COLOR_SCALE,
    SLATE800 => 30.0 / COLOR_SCALE, 41.0 / COLOR_SCALE, 59.0 / COLOR_SCALE,
    SLATE900 => 15.0 / COLOR_SCALE, 23.0 / COLOR_SCALE, 42.0 / COLOR_SCALE,
    GRAY50=> 249.0 / COLOR_SCALE, 250.0 / COLOR_SCALE, 251.0 / COLOR_SCALE,
    GRAY100 => 243.0 / COLOR_SCALE, 244.0 / COLOR_SCALE, 246.0 / COLOR_SCALE,
    GRAY200 => 229.0 / COLOR_SCALE, 231.0 / COLOR_SCALE, 235.0 / COLOR_SCALE,
    GRAY300 => 209.0 / COLOR_SCALE, 213.0 / COLOR_SCALE, 219.0 / COLOR_SCALE,
    GRAY400 => 156.0 / COLOR_SCALE, 163.0 / COLOR_SCALE, 175.0 / COLOR_SCALE,
    GRAY500 => 107.0 / COLOR_SCALE, 114.0 / COLOR_SCALE, 128.0 / COLOR_SCALE,
    GRAY600 => 75.0 / COLOR_SCALE, 85.0 / COLOR_SCALE, 99.0 / COLOR_SCALE,
    GRAY700 => 55.0 / COLOR_SCALE, 65.0 / COLOR_SCALE, 81.0 / COLOR_SCALE,
    GRAY800 => 31.0 / COLOR_SCALE, 41.0 / COLOR_SCALE, 55.0 / COLOR_SCALE,
    GRAY900 => 17.0 / COLOR_SCALE, 24.0 / COLOR_SCALE, 39.0 / COLOR_SCALE,
    ZINC50=> 250.0 / COLOR_SCALE, 250.0 / COLOR_SCALE, 250.0 / COLOR_SCALE,
    ZINC100 => 244.0 / COLOR_SCALE, 244.0 / COLOR_SCALE, 245.0 / COLOR_SCALE,
    ZINC200 => 228.0 / COLOR_SCALE, 228.0 / COLOR_SCALE, 231.0 / COLOR_SCALE,
    ZINC300 => 212.0 / COLOR_SCALE, 212.0 / COLOR_SCALE, 216.0 / COLOR_SCALE,
    ZINC400 => 161.0 / COLOR_SCALE, 161.0 / COLOR_SCALE, 170.0 / COLOR_SCALE,
    ZINC500 => 113.0 / COLOR_SCALE, 113.0 / COLOR_SCALE, 122.0 / COLOR_SCALE,
    ZINC600 => 82.0 / COLOR_SCALE, 82.0 / COLOR_SCALE, 91.0 / COLOR_SCALE,
    ZINC700 => 63.0 / COLOR_SCALE, 63.0 / COLOR_SCALE, 70.0 / COLOR_SCALE,
    ZINC800 => 39.0 / COLOR_SCALE, 39.0 / COLOR_SCALE, 42.0 / COLOR_SCALE,
    ZINC900 => 24.0 / COLOR_SCALE, 24.0 / COLOR_SCALE, 27.0 / COLOR_SCALE,
    NEUTRAL50=> 250.0 / COLOR_SCALE, 250.0 / COLOR_SCALE, 250.0 / COLOR_SCALE,
    NEUTRAL100 => 245.0 / COLOR_SCALE, 245.0 / COLOR_SCALE, 245.0 / COLOR_SCALE,
    NEUTRAL200 => 229.0 / COLOR_SCALE, 229.0 / COLOR_SCALE, 229.0 / COLOR_SCALE,
    NEUTRAL300 => 212.0 / COLOR_SCALE, 212.0 / COLOR_SCALE, 212.0 / COLOR_SCALE,
    NEUTRAL400 => 163.0 / COLOR_SCALE, 163.0 / COLOR_SCALE, 163.0 / COLOR_SCALE,
    NEUTRAL500 => 115.0 / COLOR_SCALE, 115.0 / COLOR_SCALE, 115.0 / COLOR_SCALE,
    NEUTRAL600 => 82.0 / COLOR_SCALE, 82.0 / COLOR_SCALE, 82.0 / COLOR_SCALE,
    NEUTRAL700 => 64.0 / COLOR_SCALE, 64.0 / COLOR_SCALE, 64.0 / COLOR_SCALE,
    NEUTRAL800 => 38.0 / COLOR_SCALE, 38.0 / COLOR_SCALE, 38.0 / COLOR_SCALE,
    NEUTRAL900 => 23.0 / COLOR_SCALE, 23.0 / COLOR_SCALE, 23.0 / COLOR_SCALE,
    STONE50=> 250.0 / COLOR_SCALE, 250.0 / COLOR_SCALE, 249.0 / COLOR_SCALE,
    STONE100 => 245.0 / COLOR_SCALE, 245.0 / COLOR_SCALE, 244.0 / COLOR_SCALE,
    STONE200 => 231.0 / COLOR_SCALE, 229.0 / COLOR_SCALE, 228.0 / COLOR_SCALE,
    STONE300 => 214.0 / COLOR_SCALE, 211.0 / COLOR_SCALE, 209.0 / COLOR_SCALE,
    STONE400 => 168.0 / COLOR_SCALE, 162.0 / COLOR_SCALE, 158.0 / COLOR_SCALE,
    STONE500 => 120.0 / COLOR_SCALE, 113.0 / COLOR_SCALE, 108.0 / COLOR_SCALE,
    STONE600 => 87.0 / COLOR_SCALE, 83.0 / COLOR_SCALE, 78.0 / COLOR_SCALE,
    STONE700 => 68.0 / COLOR_SCALE, 64.0 / COLOR_SCALE, 60.0 / COLOR_SCALE,
    STONE800 => 41.0 / COLOR_SCALE, 37.0 / COLOR_SCALE, 36.0 / COLOR_SCALE,
    STONE900 => 28.0 / COLOR_SCALE, 25.0 / COLOR_SCALE, 23.0 / COLOR_SCALE,
    RED50=> 254.0 / COLOR_SCALE, 242.0 / COLOR_SCALE, 242.0 / COLOR_SCALE,
    RED100 => 254.0 / COLOR_SCALE, 226.0 / COLOR_SCALE, 226.0 / COLOR_SCALE,
    RED200 => 254.0 / COLOR_SCALE, 202.0 / COLOR_SCALE, 202.0 / COLOR_SCALE,
    RED300 => 252.0 / COLOR_SCALE, 165.0 / COLOR_SCALE, 165.0 / COLOR_SCALE,
    RED400 => 248.0 / COLOR_SCALE, 113.0 / COLOR_SCALE, 113.0 / COLOR_SCALE,
    RED500 => 239.0 / COLOR_SCALE, 68.0 / COLOR_SCALE, 68.0 / COLOR_SCALE,
    RED600 => 220.0 / COLOR_SCALE, 38.0 / COLOR_SCALE, 38.0 / COLOR_SCALE,
    RED700 => 185.0 / COLOR_SCALE, 28.0 / COLOR_SCALE, 28.0 / COLOR_SCALE,
    RED800 => 153.0 / COLOR_SCALE, 27.0 / COLOR_SCALE, 27.0 / COLOR_SCALE,
    RED900 => 127.0 / COLOR_SCALE, 29.0 / COLOR_SCALE, 29.0 / COLOR_SCALE,
    ORANGE50=>COLOR_SCALE / COLOR_SCALE, 247.0 / COLOR_SCALE, 237.0 / COLOR_SCALE,
    ORANGE100 =>COLOR_SCALE / COLOR_SCALE, 237.0 / COLOR_SCALE, 213.0 / COLOR_SCALE,
    ORANGE200 => 254.0 / COLOR_SCALE, 215.0 / COLOR_SCALE, 170.0 / COLOR_SCALE,
    ORANGE300 => 253.0 / COLOR_SCALE, 186.0 / COLOR_SCALE, 116.0 / COLOR_SCALE,
    ORANGE400 => 251.0 / COLOR_SCALE, 146.0 / COLOR_SCALE, 60.0 / COLOR_SCALE,
    ORANGE500 => 249.0 / COLOR_SCALE, 115.0 / COLOR_SCALE, 22.0 / COLOR_SCALE,
    ORANGE600 => 234.0 / COLOR_SCALE, 88.0 / COLOR_SCALE, 12.0 / COLOR_SCALE,
    ORANGE700 => 194.0 / COLOR_SCALE, 65.0 / COLOR_SCALE, 12.0 / COLOR_SCALE,
    ORANGE800 => 154.0 / COLOR_SCALE, 52.0 / COLOR_SCALE, 18.0 / COLOR_SCALE,
    ORANGE900 => 124.0 / COLOR_SCALE, 45.0 / COLOR_SCALE, 18.0 / COLOR_SCALE,
    AMBER50=>COLOR_SCALE / COLOR_SCALE, 251.0 / COLOR_SCALE, 235.0 / COLOR_SCALE,
    AMBER100 => 254.0 / COLOR_SCALE, 243.0 / COLOR_SCALE, 199.0 / COLOR_SCALE,
    AMBER200 => 253.0 / COLOR_SCALE, 230.0 / COLOR_SCALE, 138.0 / COLOR_SCALE,
    AMBER300 => 252.0 / COLOR_SCALE, 211.0 / COLOR_SCALE, 77.0 / COLOR_SCALE,
    AMBER400 => 251.0 / COLOR_SCALE, 191.0 / COLOR_SCALE, 36.0 / COLOR_SCALE,
    AMBER500 => 245.0 / COLOR_SCALE, 158.0 / COLOR_SCALE, 11.0 / COLOR_SCALE,
    AMBER600 => 217.0 / COLOR_SCALE, 119.0 / COLOR_SCALE, 6.0 / COLOR_SCALE,
    AMBER700 => 180.0 / COLOR_SCALE, 83.0 / COLOR_SCALE, 9.0 / COLOR_SCALE,
    AMBER800 => 146.0 / COLOR_SCALE, 64.0 / COLOR_SCALE, 14.0 / COLOR_SCALE,
    AMBER900 => 120.0 / COLOR_SCALE, 53.0 / COLOR_SCALE, 15.0 / COLOR_SCALE,
    YELLOW50=> 254.0 / COLOR_SCALE, 252.0 / COLOR_SCALE, 232.0 / COLOR_SCALE,
    YELLOW100 => 254.0 / COLOR_SCALE, 249.0 / COLOR_SCALE, 195.0 / COLOR_SCALE,
    YELLOW200 => 254.0 / COLOR_SCALE, 240.0 / COLOR_SCALE, 138.0 / COLOR_SCALE,
    YELLOW300 => 253.0 / COLOR_SCALE, 224.0 / COLOR_SCALE, 71.0 / COLOR_SCALE,
    YELLOW400 => 250.0 / COLOR_SCALE, 204.0 / COLOR_SCALE, 21.0 / COLOR_SCALE,
    YELLOW500 => 234.0 / COLOR_SCALE, 179.0 / COLOR_SCALE, 8.0 / COLOR_SCALE,
    YELLOW600 => 202.0 / COLOR_SCALE, 138.0 / COLOR_SCALE, 4.0 / COLOR_SCALE,
    YELLOW700 => 161.0 / COLOR_SCALE, 98.0 / COLOR_SCALE, 7.0 / COLOR_SCALE,
    YELLOW800 => 133.0 / COLOR_SCALE, 77.0 / COLOR_SCALE, 14.0 / COLOR_SCALE,
    YELLOW900 => 113.0 / COLOR_SCALE, 63.0 / COLOR_SCALE, 18.0 / COLOR_SCALE,
    LIME50=> 247.0 / COLOR_SCALE, 254.0 / COLOR_SCALE, 231.0 / COLOR_SCALE,
    LIME100 => 236.0 / COLOR_SCALE, 252.0 / COLOR_SCALE, 203.0 / COLOR_SCALE,
    LIME200 => 217.0 / COLOR_SCALE, 249.0 / COLOR_SCALE, 157.0 / COLOR_SCALE,
    LIME300 => 190.0 / COLOR_SCALE, 242.0 / COLOR_SCALE, 100.0 / COLOR_SCALE,
    LIME400 => 163.0 / COLOR_SCALE, 230.0 / COLOR_SCALE, 53.0 / COLOR_SCALE,
    LIME500 => 132.0 / COLOR_SCALE, 204.0 / COLOR_SCALE, 22.0 / COLOR_SCALE,
    LIME600 => 101.0 / COLOR_SCALE, 163.0 / COLOR_SCALE, 13.0 / COLOR_SCALE,
    LIME700 => 77.0 / COLOR_SCALE, 124.0 / COLOR_SCALE, 15.0 / COLOR_SCALE,
    LIME800 => 63.0 / COLOR_SCALE, 98.0 / COLOR_SCALE, 18.0 / COLOR_SCALE,
    LIME900 => 54.0 / COLOR_SCALE, 83.0 / COLOR_SCALE, 20.0 / COLOR_SCALE,
    GREEN50=> 240.0 / COLOR_SCALE, 253.0 / COLOR_SCALE, 244.0 / COLOR_SCALE,
    GREEN100 => 220.0 / COLOR_SCALE, 252.0 / COLOR_SCALE, 231.0 / COLOR_SCALE,
    GREEN200 => 187.0 / COLOR_SCALE, 247.0 / COLOR_SCALE, 208.0 / COLOR_SCALE,
    GREEN300 => 134.0 / COLOR_SCALE, 239.0 / COLOR_SCALE, 172.0 / COLOR_SCALE,
    GREEN400 => 74.0 / COLOR_SCALE, 222.0 / COLOR_SCALE, 128.0 / COLOR_SCALE,
    GREEN500 => 34.0 / COLOR_SCALE, 197.0 / COLOR_SCALE, 94.0 / COLOR_SCALE,
    GREEN600 => 22.0 / COLOR_SCALE, 163.0 / COLOR_SCALE, 74.0 / COLOR_SCALE,
    GREEN700 => 21.0 / COLOR_SCALE, 128.0 / COLOR_SCALE, 61.0 / COLOR_SCALE,
    GREEN800 => 22.0 / COLOR_SCALE, 101.0 / COLOR_SCALE, 52.0 / COLOR_SCALE,
    GREEN900 => 20.0 / COLOR_SCALE, 83.0 / COLOR_SCALE, 45.0 / COLOR_SCALE,
    EMERALD50=> 236.0 / COLOR_SCALE, 253.0 / COLOR_SCALE, 245.0 / COLOR_SCALE,
    EMERALD100 => 209.0 / COLOR_SCALE, 250.0 / COLOR_SCALE, 229.0 / COLOR_SCALE,
    EMERALD200 => 167.0 / COLOR_SCALE, 243.0 / COLOR_SCALE, 208.0 / COLOR_SCALE,
    EMERALD300 => 110.0 / COLOR_SCALE, 231.0 / COLOR_SCALE, 183.0 / COLOR_SCALE,
    EMERALD400 => 52.0 / COLOR_SCALE, 211.0 / COLOR_SCALE, 153.0 / COLOR_SCALE,
    EMERALD500 => 16.0 / COLOR_SCALE, 185.0 / COLOR_SCALE, 129.0 / COLOR_SCALE,
    EMERALD600 => 5.0 / COLOR_SCALE, 150.0 / COLOR_SCALE, 105.0 / COLOR_SCALE,
    EMERALD700 => 4.0 / COLOR_SCALE, 120.0 / COLOR_SCALE, 87.0 / COLOR_SCALE,
    EMERALD800 => 6.0 / COLOR_SCALE, 95.0 / COLOR_SCALE, 70.0 / COLOR_SCALE,
    EMERALD900 => 6.0 / COLOR_SCALE, 78.0 / COLOR_SCALE, 59.0 / COLOR_SCALE,
    TEAL50=> 240.0 / COLOR_SCALE, 253.0 / COLOR_SCALE, 250.0 / COLOR_SCALE,
    TEAL100 => 204.0 / COLOR_SCALE, 251.0 / COLOR_SCALE, 241.0 / COLOR_SCALE,
    TEAL200 => 153.0 / COLOR_SCALE, 246.0 / COLOR_SCALE, 228.0 / COLOR_SCALE,
    TEAL300 => 94.0 / COLOR_SCALE, 234.0 / COLOR_SCALE, 212.0 / COLOR_SCALE,
    TEAL400 => 45.0 / COLOR_SCALE, 212.0 / COLOR_SCALE, 191.0 / COLOR_SCALE,
    TEAL500 => 20.0 / COLOR_SCALE, 184.0 / COLOR_SCALE, 166.0 / COLOR_SCALE,
    TEAL600 => 13.0 / COLOR_SCALE, 148.0 / COLOR_SCALE, 136.0 / COLOR_SCALE,
    TEAL700 => 15.0 / COLOR_SCALE, 118.0 / COLOR_SCALE, 110.0 / COLOR_SCALE,
    TEAL800 => 17.0 / COLOR_SCALE, 94.0 / COLOR_SCALE, 89.0 / COLOR_SCALE,
    TEAL900 => 19.0 / COLOR_SCALE, 78.0 / COLOR_SCALE, 74.0 / COLOR_SCALE,
    CYAN50=> 236.0 / COLOR_SCALE, 254.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    CYAN100 => 207.0 / COLOR_SCALE, 250.0 / COLOR_SCALE, 254.0 / COLOR_SCALE,
    CYAN200 => 165.0 / COLOR_SCALE, 243.0 / COLOR_SCALE, 252.0 / COLOR_SCALE,
    CYAN300 => 103.0 / COLOR_SCALE, 232.0 / COLOR_SCALE, 249.0 / COLOR_SCALE,
    CYAN400 => 34.0 / COLOR_SCALE, 211.0 / COLOR_SCALE, 238.0 / COLOR_SCALE,
    CYAN500 => 6.0 / COLOR_SCALE, 182.0 / COLOR_SCALE, 212.0 / COLOR_SCALE,
    CYAN600 => 8.0 / COLOR_SCALE, 145.0 / COLOR_SCALE, 178.0 / COLOR_SCALE,
    CYAN700 => 14.0 / COLOR_SCALE, 116.0 / COLOR_SCALE, 144.0 / COLOR_SCALE,
    CYAN800 => 21.0 / COLOR_SCALE, 94.0 / COLOR_SCALE, 117.0 / COLOR_SCALE,
    CYAN900 => 22.0 / COLOR_SCALE, 78.0 / COLOR_SCALE, 99.0 / COLOR_SCALE,
    SKY50=> 240.0 / COLOR_SCALE, 249.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    SKY100 => 224.0 / COLOR_SCALE, 242.0 / COLOR_SCALE, 254.0 / COLOR_SCALE,
    SKY200 => 186.0 / COLOR_SCALE, 230.0 / COLOR_SCALE, 253.0 / COLOR_SCALE,
    SKY300 => 125.0 / COLOR_SCALE, 211.0 / COLOR_SCALE, 252.0 / COLOR_SCALE,
    SKY400 => 56.0 / COLOR_SCALE, 189.0 / COLOR_SCALE, 248.0 / COLOR_SCALE,
    SKY500 => 14.0 / COLOR_SCALE, 165.0 / COLOR_SCALE, 233.0 / COLOR_SCALE,
    SKY600 => 2.0 / COLOR_SCALE, 132.0 / COLOR_SCALE, 199.0 / COLOR_SCALE,
    SKY700 => 3.0 / COLOR_SCALE, 105.0 / COLOR_SCALE, 161.0 / COLOR_SCALE,
    SKY800 => 7.0 / COLOR_SCALE, 89.0 / COLOR_SCALE, 133.0 / COLOR_SCALE,
    SKY900 => 12.0 / COLOR_SCALE, 74.0 / COLOR_SCALE, 110.0 / COLOR_SCALE,
    BLUE50=> 239.0 / COLOR_SCALE, 246.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    BLUE100 => 219.0 / COLOR_SCALE, 234.0 / COLOR_SCALE, 254.0 / COLOR_SCALE,
    BLUE200 => 191.0 / COLOR_SCALE, 219.0 / COLOR_SCALE, 254.0 / COLOR_SCALE,
    BLUE300 => 147.0 / COLOR_SCALE, 197.0 / COLOR_SCALE, 253.0 / COLOR_SCALE,
    BLUE400 => 96.0 / COLOR_SCALE, 165.0 / COLOR_SCALE, 250.0 / COLOR_SCALE,
    BLUE500 => 59.0 / COLOR_SCALE, 130.0 / COLOR_SCALE, 246.0 / COLOR_SCALE,
    BLUE600 => 37.0 / COLOR_SCALE, 99.0 / COLOR_SCALE, 235.0 / COLOR_SCALE,
    BLUE700 => 29.0 / COLOR_SCALE, 78.0 / COLOR_SCALE, 216.0 / COLOR_SCALE,
    BLUE800 => 30.0 / COLOR_SCALE, 64.0 / COLOR_SCALE, 175.0 / COLOR_SCALE,
    BLUE900 => 30.0 / COLOR_SCALE, 58.0 / COLOR_SCALE, 138.0 / COLOR_SCALE,
    INDIGO50=> 238.0 / COLOR_SCALE, 242.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    INDIGO100 => 224.0 / COLOR_SCALE, 231.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    INDIGO200 => 199.0 / COLOR_SCALE, 210.0 / COLOR_SCALE, 254.0 / COLOR_SCALE,
    INDIGO300 => 165.0 / COLOR_SCALE, 180.0 / COLOR_SCALE, 252.0 / COLOR_SCALE,
    INDIGO400 => 129.0 / COLOR_SCALE, 140.0 / COLOR_SCALE, 248.0 / COLOR_SCALE,
    INDIGO500 => 99.0 / COLOR_SCALE, 102.0 / COLOR_SCALE, 241.0 / COLOR_SCALE,
    INDIGO600 => 79.0 / COLOR_SCALE, 70.0 / COLOR_SCALE, 229.0 / COLOR_SCALE,
    INDIGO700 => 67.0 / COLOR_SCALE, 56.0 / COLOR_SCALE, 202.0 / COLOR_SCALE,
    INDIGO800 => 55.0 / COLOR_SCALE, 48.0 / COLOR_SCALE, 163.0 / COLOR_SCALE,
    INDIGO900 => 49.0 / COLOR_SCALE, 46.0 / COLOR_SCALE, 129.0 / COLOR_SCALE,
    VIOLET50=> 245.0 / COLOR_SCALE, 243.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    VIOLET100 => 237.0 / COLOR_SCALE, 233.0 / COLOR_SCALE, 254.0 / COLOR_SCALE,
    VIOLET200 => 221.0 / COLOR_SCALE, 214.0 / COLOR_SCALE, 254.0 / COLOR_SCALE,
    VIOLET300 => 196.0 / COLOR_SCALE, 181.0 / COLOR_SCALE, 253.0 / COLOR_SCALE,
    VIOLET400 => 167.0 / COLOR_SCALE, 139.0 / COLOR_SCALE, 250.0 / COLOR_SCALE,
    VIOLET500 => 139.0 / COLOR_SCALE, 92.0 / COLOR_SCALE, 246.0 / COLOR_SCALE,
    VIOLET600 => 124.0 / COLOR_SCALE, 58.0 / COLOR_SCALE, 237.0 / COLOR_SCALE,
    VIOLET700 => 109.0 / COLOR_SCALE, 40.0 / COLOR_SCALE, 217.0 / COLOR_SCALE,
    VIOLET800 => 91.0 / COLOR_SCALE, 33.0 / COLOR_SCALE, 182.0 / COLOR_SCALE,
    VIOLET900 => 76.0 / COLOR_SCALE, 29.0 / COLOR_SCALE, 149.0 / COLOR_SCALE,
    PURPLE50=> 250.0 / COLOR_SCALE, 245.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    PURPLE100 => 243.0 / COLOR_SCALE, 232.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    PURPLE200 => 233.0 / COLOR_SCALE, 213.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    PURPLE300 => 216.0 / COLOR_SCALE, 180.0 / COLOR_SCALE, 254.0 / COLOR_SCALE,
    PURPLE400 => 192.0 / COLOR_SCALE, 132.0 / COLOR_SCALE, 252.0 / COLOR_SCALE,
    PURPLE500 => 168.0 / COLOR_SCALE, 85.0 / COLOR_SCALE, 247.0 / COLOR_SCALE,
    PURPLE600 => 147.0 / COLOR_SCALE, 51.0 / COLOR_SCALE, 234.0 / COLOR_SCALE,
    PURPLE700 => 126.0 / COLOR_SCALE, 34.0 / COLOR_SCALE, 206.0 / COLOR_SCALE,
    PURPLE800 => 107.0 / COLOR_SCALE, 33.0 / COLOR_SCALE, 168.0 / COLOR_SCALE,
    PURPLE900 => 88.0 / COLOR_SCALE, 28.0 / COLOR_SCALE, 135.0 / COLOR_SCALE,
    FUCHSIA50=> 253.0 / COLOR_SCALE, 244.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    FUCHSIA100 => 250.0 / COLOR_SCALE, 232.0 / COLOR_SCALE,COLOR_SCALE / COLOR_SCALE,
    FUCHSIA200 => 245.0 / COLOR_SCALE, 208.0 / COLOR_SCALE, 254.0 / COLOR_SCALE,
    FUCHSIA300 => 240.0 / COLOR_SCALE, 171.0 / COLOR_SCALE, 252.0 / COLOR_SCALE,
    FUCHSIA400 => 232.0 / COLOR_SCALE, 121.0 / COLOR_SCALE, 249.0 / COLOR_SCALE,
    FUCHSIA500 => 217.0 / COLOR_SCALE, 70.0 / COLOR_SCALE, 239.0 / COLOR_SCALE,
    FUCHSIA600 => 192.0 / COLOR_SCALE, 38.0 / COLOR_SCALE, 211.0 / COLOR_SCALE,
    FUCHSIA700 => 162.0 / COLOR_SCALE, 28.0 / COLOR_SCALE, 175.0 / COLOR_SCALE,
    FUCHSIA800 => 134.0 / COLOR_SCALE, 25.0 / COLOR_SCALE, 143.0 / COLOR_SCALE,
    FUCHSIA900 => 112.0 / COLOR_SCALE, 26.0 / COLOR_SCALE, 117.0 / COLOR_SCALE,
    PINK50=> 253.0 / COLOR_SCALE, 242.0 / COLOR_SCALE, 248.0 / COLOR_SCALE,
    PINK100 => 252.0 / COLOR_SCALE, 231.0 / COLOR_SCALE, 243.0 / COLOR_SCALE,
    PINK200 => 251.0 / COLOR_SCALE, 207.0 / COLOR_SCALE, 232.0 / COLOR_SCALE,
    PINK300 => 249.0 / COLOR_SCALE, 168.0 / COLOR_SCALE, 212.0 / COLOR_SCALE,
    PINK400 => 244.0 / COLOR_SCALE, 114.0 / COLOR_SCALE, 182.0 / COLOR_SCALE,
    PINK500 => 236.0 / COLOR_SCALE, 72.0 / COLOR_SCALE, 153.0 / COLOR_SCALE,
    PINK600 => 219.0 / COLOR_SCALE, 39.0 / COLOR_SCALE, 119.0 / COLOR_SCALE,
    PINK700 => 190.0 / COLOR_SCALE, 24.0 / COLOR_SCALE, 93.0 / COLOR_SCALE,
    PINK800 => 157.0 / COLOR_SCALE, 23.0 / COLOR_SCALE, 77.0 / COLOR_SCALE,
    PINK900 => 131.0 / COLOR_SCALE, 24.0 / COLOR_SCALE, 67.0 / COLOR_SCALE,
    ROSE50=>COLOR_SCALE / COLOR_SCALE, 241.0 / COLOR_SCALE, 242.0 / COLOR_SCALE,
    ROSE100 =>COLOR_SCALE / COLOR_SCALE, 228.0 / COLOR_SCALE, 230.0 / COLOR_SCALE,
    ROSE200 => 254.0 / COLOR_SCALE, 205.0 / COLOR_SCALE, 211.0 / COLOR_SCALE,
    ROSE300 => 253.0 / COLOR_SCALE, 164.0 / COLOR_SCALE, 175.0 / COLOR_SCALE,
    ROSE400 => 251.0 / COLOR_SCALE, 113.0 / COLOR_SCALE, 133.0 / COLOR_SCALE,
    ROSE500 => 244.0 / COLOR_SCALE, 63.0 / COLOR_SCALE, 94.0 / COLOR_SCALE,
    ROSE600 => 225.0 / COLOR_SCALE, 29.0 / COLOR_SCALE, 72.0 / COLOR_SCALE,
    ROSE700 => 190.0 / COLOR_SCALE, 18.0 / COLOR_SCALE, 60.0 / COLOR_SCALE,
    ROSE800 => 159.0 / COLOR_SCALE, 18.0 / COLOR_SCALE, 57.0 / COLOR_SCALE,
    ROSE900 => 136.0 / COLOR_SCALE, 19.0 / COLOR_SCALE, 55.0 / COLOR_SCALE
}

impl SubmergeColors {
    pub const fn get_rgb(&self) -> (f32, f32, f32) {
        match self {
            SubmergeColors::SLATE50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SLATE100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SLATE200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SLATE300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SLATE400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SLATE500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SLATE600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SLATE700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SLATE800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SLATE900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GRAY50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GRAY100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GRAY200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GRAY300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GRAY400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GRAY500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GRAY600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GRAY700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GRAY800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GRAY900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ZINC50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ZINC100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ZINC200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ZINC300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ZINC400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ZINC500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ZINC600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ZINC700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ZINC800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ZINC900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::NEUTRAL50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::NEUTRAL100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::NEUTRAL200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::NEUTRAL300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::NEUTRAL400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::NEUTRAL500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::NEUTRAL600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::NEUTRAL700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::NEUTRAL800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::NEUTRAL900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::STONE50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::STONE100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::STONE200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::STONE300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::STONE400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::STONE500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::STONE600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::STONE700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::STONE800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::STONE900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::RED50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::RED100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::RED200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::RED300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::RED400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::RED500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::RED600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::RED700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::RED800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::RED900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ORANGE50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ORANGE100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ORANGE200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ORANGE300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ORANGE400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ORANGE500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ORANGE600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ORANGE700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ORANGE800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ORANGE900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::AMBER50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::AMBER100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::AMBER200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::AMBER300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::AMBER400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::AMBER500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::AMBER600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::AMBER700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::AMBER800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::AMBER900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::YELLOW50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::YELLOW100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::YELLOW200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::YELLOW300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::YELLOW400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::YELLOW500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::YELLOW600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::YELLOW700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::YELLOW800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::YELLOW900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::LIME50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::LIME100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::LIME200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::LIME300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::LIME400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::LIME500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::LIME600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::LIME700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::LIME800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::LIME900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GREEN50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GREEN100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GREEN200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GREEN300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GREEN400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GREEN500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GREEN600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GREEN700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GREEN800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::GREEN900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::EMERALD50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::EMERALD100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::EMERALD200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::EMERALD300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::EMERALD400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::EMERALD500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::EMERALD600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::EMERALD700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::EMERALD800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::EMERALD900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::TEAL50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::TEAL100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::TEAL200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::TEAL300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::TEAL400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::TEAL500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::TEAL600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::TEAL700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::TEAL800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::TEAL900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::CYAN50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::CYAN100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::CYAN200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::CYAN300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::CYAN400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::CYAN500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::CYAN600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::CYAN700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::CYAN800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::CYAN900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SKY50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SKY100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SKY200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SKY300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SKY400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SKY500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SKY600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SKY700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SKY800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::SKY900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLUE50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLUE100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLUE200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLUE300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLUE400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLUE500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLUE600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLUE700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLUE800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLUE900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::INDIGO50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::INDIGO100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::INDIGO200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::INDIGO300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::INDIGO400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::INDIGO500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::INDIGO600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::INDIGO700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::INDIGO800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::INDIGO900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::VIOLET50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::VIOLET100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::VIOLET200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::VIOLET300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::VIOLET400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::VIOLET500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::VIOLET600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::VIOLET700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::VIOLET800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::VIOLET900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PURPLE50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PURPLE100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PURPLE200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PURPLE300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PURPLE400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PURPLE500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PURPLE600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PURPLE700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PURPLE800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PURPLE900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::FUCHSIA50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::FUCHSIA100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::FUCHSIA200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::FUCHSIA300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::FUCHSIA400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::FUCHSIA500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::FUCHSIA600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::FUCHSIA700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::FUCHSIA800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::FUCHSIA900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PINK50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PINK100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PINK200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PINK300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PINK400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PINK500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PINK600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PINK700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PINK800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::PINK900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ROSE50 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ROSE100 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ROSE200 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ROSE300 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ROSE400 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ROSE500 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ROSE600 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ROSE700 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ROSE800 { r, g, b } => (*r, *g, *b),
            SubmergeColors::ROSE900 { r, g, b } => (*r, *g, *b),
            SubmergeColors::BLACK { r, g, b } => (*r, *g, *b),
            SubmergeColors::WHITE { r, g, b } => (*r, *g, *b),
        }
    }

    pub fn get_color_from_style_text(style: &str) -> Color {
        if style.trim() == "slate-50" {
            return SubmergeColors::color(SubmergeColors::SLATE50);
        }
        if style.trim() == "slate-100" {
            return SubmergeColors::color(SubmergeColors::SLATE100);
        }
        if style.trim() == "slate-200" {
            return SubmergeColors::color(SubmergeColors::SLATE200);
        }
        if style.trim() == "slate-300" {
            return SubmergeColors::color(SubmergeColors::SLATE300);
        }
        if style.trim() == "slate-400" {
            return SubmergeColors::color(SubmergeColors::SLATE400);
        }
        if style.trim() == "slate-500" {
            return SubmergeColors::color(SubmergeColors::SLATE500);
        }
        if style.trim() == "slate-600" {
            return SubmergeColors::color(SubmergeColors::SLATE600);
        }
        if style.trim() == "slate-700" {
            return SubmergeColors::color(SubmergeColors::SLATE700);
        }
        if style.trim() == "slate-800" {
            return SubmergeColors::color(SubmergeColors::SLATE800);
        }
        if style.trim() == "slate-900" {
            return SubmergeColors::color(SubmergeColors::SLATE900);
        }
        if style.trim() == "gray-50" {
            return SubmergeColors::color(SubmergeColors::GRAY50);
        }
        if style.trim() == "gray-100" {
            return SubmergeColors::color(SubmergeColors::GRAY100);
        }
        if style.trim() == "gray-200" {
            return SubmergeColors::color(SubmergeColors::GRAY200);
        }
        if style.trim() == "gray-300" {
            return SubmergeColors::color(SubmergeColors::GRAY300);
        }
        if style.trim() == "gray-400" {
            return SubmergeColors::color(SubmergeColors::GRAY400);
        }
        if style.trim() == "gray-500" {
            return SubmergeColors::color(SubmergeColors::GRAY500);
        }
        if style.trim() == "gray-600" {
            return SubmergeColors::color(SubmergeColors::GRAY600);
        }
        if style.trim() == "gray-700" {
            return SubmergeColors::color(SubmergeColors::GRAY700);
        }
        if style.trim() == "gray-800" {
            return SubmergeColors::color(SubmergeColors::GRAY800);
        }
        if style.trim() == "gray-900" {
            return SubmergeColors::color(SubmergeColors::GRAY900);
        }
        if style.trim() == "zinc-50" {
            return SubmergeColors::color(SubmergeColors::ZINC50);
        }
        if style.trim() == "zinc-100" {
            return SubmergeColors::color(SubmergeColors::ZINC100);
        }
        if style.trim() == "zinc-200" {
            return SubmergeColors::color(SubmergeColors::ZINC200);
        }
        if style.trim() == "zinc-300" {
            return SubmergeColors::color(SubmergeColors::ZINC300);
        }
        if style.trim() == "zinc-400" {
            return SubmergeColors::color(SubmergeColors::ZINC400);
        }
        if style.trim() == "zinc-500" {
            return SubmergeColors::color(SubmergeColors::ZINC500);
        }
        if style.trim() == "zinc-600" {
            return SubmergeColors::color(SubmergeColors::ZINC600);
        }
        if style.trim() == "zinc-700" {
            return SubmergeColors::color(SubmergeColors::ZINC700);
        }
        if style.trim() == "zinc-800" {
            return SubmergeColors::color(SubmergeColors::ZINC800);
        }
        if style.trim() == "zinc-900" {
            return SubmergeColors::color(SubmergeColors::ZINC900);
        }
        if style.trim() == "neutral-50" {
            return SubmergeColors::color(SubmergeColors::NEUTRAL50);
        }
        if style.trim() == "neutral-100" {
            return SubmergeColors::color(SubmergeColors::NEUTRAL100);
        }
        if style.trim() == "neutral-200" {
            return SubmergeColors::color(SubmergeColors::NEUTRAL200);
        }
        if style.trim() == "neutral-300" {
            return SubmergeColors::color(SubmergeColors::NEUTRAL300);
        }
        if style.trim() == "neutral-400" {
            return SubmergeColors::color(SubmergeColors::NEUTRAL400);
        }
        if style.trim() == "neutral-500" {
            return SubmergeColors::color(SubmergeColors::NEUTRAL500);
        }
        if style.trim() == "neutral-600" {
            return SubmergeColors::color(SubmergeColors::NEUTRAL600);
        }
        if style.trim() == "neutral-700" {
            return SubmergeColors::color(SubmergeColors::NEUTRAL700);
        }
        if style.trim() == "neutral-800" {
            return SubmergeColors::color(SubmergeColors::NEUTRAL800);
        }
        if style.trim() == "neutral-900" {
            return SubmergeColors::color(SubmergeColors::NEUTRAL900);
        }
        if style.trim() == "stone-50" {
            return SubmergeColors::color(SubmergeColors::STONE50);
        }
        if style.trim() == "stone-100" {
            return SubmergeColors::color(SubmergeColors::STONE100);
        }
        if style.trim() == "stone-200" {
            return SubmergeColors::color(SubmergeColors::STONE200);
        }
        if style.trim() == "stone-300" {
            return SubmergeColors::color(SubmergeColors::STONE300);
        }
        if style.trim() == "stone-400" {
            return SubmergeColors::color(SubmergeColors::STONE400);
        }
        if style.trim() == "stone-500" {
            return SubmergeColors::color(SubmergeColors::STONE500);
        }
        if style.trim() == "stone-600" {
            return SubmergeColors::color(SubmergeColors::STONE600);
        }
        if style.trim() == "stone-700" {
            return SubmergeColors::color(SubmergeColors::STONE700);
        }
        if style.trim() == "stone-800" {
            return SubmergeColors::color(SubmergeColors::STONE800);
        }
        if style.trim() == "stone-900" {
            return SubmergeColors::color(SubmergeColors::STONE900);
        }
        if style.trim() == "red-50" {
            return SubmergeColors::color(SubmergeColors::RED50);
        }
        if style.trim() == "red-100" {
            return SubmergeColors::color(SubmergeColors::RED100);
        }
        if style.trim() == "red-200" {
            return SubmergeColors::color(SubmergeColors::RED200);
        }
        if style.trim() == "red-300" {
            return SubmergeColors::color(SubmergeColors::RED300);
        }
        if style.trim() == "red-400" {
            return SubmergeColors::color(SubmergeColors::RED400);
        }
        if style.trim() == "red-500" {
            return SubmergeColors::color(SubmergeColors::RED500);
        }
        if style.trim() == "red-600" {
            return SubmergeColors::color(SubmergeColors::RED600);
        }
        if style.trim() == "red-700" {
            return SubmergeColors::color(SubmergeColors::RED700);
        }
        if style.trim() == "red-800" {
            return SubmergeColors::color(SubmergeColors::RED800);
        }
        if style.trim() == "red-900" {
            return SubmergeColors::color(SubmergeColors::RED900);
        }
        if style.trim() == "orange-50" {
            return SubmergeColors::color(SubmergeColors::ORANGE50);
        }
        if style.trim() == "orange-100" {
            return SubmergeColors::color(SubmergeColors::ORANGE100);
        }
        if style.trim() == "orange-200" {
            return SubmergeColors::color(SubmergeColors::ORANGE200);
        }
        if style.trim() == "orange-300" {
            return SubmergeColors::color(SubmergeColors::ORANGE300);
        }
        if style.trim() == "orange-400" {
            return SubmergeColors::color(SubmergeColors::ORANGE400);
        }
        if style.trim() == "orange-500" {
            return SubmergeColors::color(SubmergeColors::ORANGE500);
        }
        if style.trim() == "orange-600" {
            return SubmergeColors::color(SubmergeColors::ORANGE600);
        }
        if style.trim() == "orange-700" {
            return SubmergeColors::color(SubmergeColors::ORANGE700);
        }
        if style.trim() == "orange-800" {
            return SubmergeColors::color(SubmergeColors::ORANGE800);
        }
        if style.trim() == "orange-900" {
            return SubmergeColors::color(SubmergeColors::ORANGE900);
        }
        if style.trim() == "amber-50" {
            return SubmergeColors::color(SubmergeColors::AMBER50);
        }
        if style.trim() == "amber-100" {
            return SubmergeColors::color(SubmergeColors::AMBER100);
        }
        if style.trim() == "amber-200" {
            return SubmergeColors::color(SubmergeColors::AMBER200);
        }
        if style.trim() == "amber-300" {
            return SubmergeColors::color(SubmergeColors::AMBER300);
        }
        if style.trim() == "amber-400" {
            return SubmergeColors::color(SubmergeColors::AMBER400);
        }
        if style.trim() == "amber-500" {
            return SubmergeColors::color(SubmergeColors::AMBER500);
        }
        if style.trim() == "amber-600" {
            return SubmergeColors::color(SubmergeColors::AMBER600);
        }
        if style.trim() == "amber-700" {
            return SubmergeColors::color(SubmergeColors::AMBER700);
        }
        if style.trim() == "amber-800" {
            return SubmergeColors::color(SubmergeColors::AMBER800);
        }
        if style.trim() == "amber-900" {
            return SubmergeColors::color(SubmergeColors::AMBER900);
        }
        if style.trim() == "yellow-50" {
            return SubmergeColors::color(SubmergeColors::YELLOW50);
        }
        if style.trim() == "yellow-100" {
            return SubmergeColors::color(SubmergeColors::YELLOW100);
        }
        if style.trim() == "yellow-200" {
            return SubmergeColors::color(SubmergeColors::YELLOW200);
        }
        if style.trim() == "yellow-300" {
            return SubmergeColors::color(SubmergeColors::YELLOW300);
        }
        if style.trim() == "yellow-400" {
            return SubmergeColors::color(SubmergeColors::YELLOW400);
        }
        if style.trim() == "yellow-500" {
            return SubmergeColors::color(SubmergeColors::YELLOW500);
        }
        if style.trim() == "yellow-600" {
            return SubmergeColors::color(SubmergeColors::YELLOW600);
        }
        if style.trim() == "yellow-700" {
            return SubmergeColors::color(SubmergeColors::YELLOW700);
        }
        if style.trim() == "yellow-800" {
            return SubmergeColors::color(SubmergeColors::YELLOW800);
        }
        if style.trim() == "yellow-900" {
            return SubmergeColors::color(SubmergeColors::YELLOW900);
        }
        if style.trim() == "lime-50" {
            return SubmergeColors::color(SubmergeColors::LIME50);
        }
        if style.trim() == "lime-100" {
            return SubmergeColors::color(SubmergeColors::LIME100);
        }
        if style.trim() == "lime-200" {
            return SubmergeColors::color(SubmergeColors::LIME200);
        }
        if style.trim() == "lime-300" {
            return SubmergeColors::color(SubmergeColors::LIME300);
        }
        if style.trim() == "lime-400" {
            return SubmergeColors::color(SubmergeColors::LIME400);
        }
        if style.trim() == "lime-500" {
            return SubmergeColors::color(SubmergeColors::LIME500);
        }
        if style.trim() == "lime-600" {
            return SubmergeColors::color(SubmergeColors::LIME600);
        }
        if style.trim() == "lime-700" {
            return SubmergeColors::color(SubmergeColors::LIME700);
        }
        if style.trim() == "lime-800" {
            return SubmergeColors::color(SubmergeColors::LIME800);
        }
        if style.trim() == "lime-900" {
            return SubmergeColors::color(SubmergeColors::LIME900);
        }
        if style.trim() == "green-50" {
            return SubmergeColors::color(SubmergeColors::GREEN50);
        }
        if style.trim() == "green-100" {
            return SubmergeColors::color(SubmergeColors::GREEN100);
        }
        if style.trim() == "green-200" {
            return SubmergeColors::color(SubmergeColors::GREEN200);
        }
        if style.trim() == "green-300" {
            return SubmergeColors::color(SubmergeColors::GREEN300);
        }
        if style.trim() == "green-400" {
            return SubmergeColors::color(SubmergeColors::GREEN400);
        }
        if style.trim() == "green-500" {
            return SubmergeColors::color(SubmergeColors::GREEN500);
        }
        if style.trim() == "green-600" {
            return SubmergeColors::color(SubmergeColors::GREEN600);
        }
        if style.trim() == "green-700" {
            return SubmergeColors::color(SubmergeColors::GREEN700);
        }
        if style.trim() == "green-800" {
            return SubmergeColors::color(SubmergeColors::GREEN800);
        }
        if style.trim() == "green-900" {
            return SubmergeColors::color(SubmergeColors::GREEN900);
        }
        if style.trim() == "emerald-50" {
            return SubmergeColors::color(SubmergeColors::EMERALD50);
        }
        if style.trim() == "emerald-100" {
            return SubmergeColors::color(SubmergeColors::EMERALD100);
        }
        if style.trim() == "emerald-200" {
            return SubmergeColors::color(SubmergeColors::EMERALD200);
        }
        if style.trim() == "emerald-300" {
            return SubmergeColors::color(SubmergeColors::EMERALD300);
        }
        if style.trim() == "emerald-400" {
            return SubmergeColors::color(SubmergeColors::EMERALD400);
        }
        if style.trim() == "emerald-500" {
            return SubmergeColors::color(SubmergeColors::EMERALD500);
        }
        if style.trim() == "emerald-600" {
            return SubmergeColors::color(SubmergeColors::EMERALD600);
        }
        if style.trim() == "emerald-700" {
            return SubmergeColors::color(SubmergeColors::EMERALD700);
        }
        if style.trim() == "emerald-800" {
            return SubmergeColors::color(SubmergeColors::EMERALD800);
        }
        if style.trim() == "emerald-900" {
            return SubmergeColors::color(SubmergeColors::EMERALD900);
        }
        if style.trim() == "teal-50" {
            return SubmergeColors::color(SubmergeColors::TEAL50);
        }
        if style.trim() == "teal-100" {
            return SubmergeColors::color(SubmergeColors::TEAL100);
        }
        if style.trim() == "teal-200" {
            return SubmergeColors::color(SubmergeColors::TEAL200);
        }
        if style.trim() == "teal-300" {
            return SubmergeColors::color(SubmergeColors::TEAL300);
        }
        if style.trim() == "teal-400" {
            return SubmergeColors::color(SubmergeColors::TEAL400);
        }
        if style.trim() == "teal-500" {
            return SubmergeColors::color(SubmergeColors::TEAL500);
        }
        if style.trim() == "teal-600" {
            return SubmergeColors::color(SubmergeColors::TEAL600);
        }
        if style.trim() == "teal-700" {
            return SubmergeColors::color(SubmergeColors::TEAL700);
        }
        if style.trim() == "teal-800" {
            return SubmergeColors::color(SubmergeColors::TEAL800);
        }
        if style.trim() == "teal-900" {
            return SubmergeColors::color(SubmergeColors::TEAL900);
        }
        if style.trim() == "cyan-50" {
            return SubmergeColors::color(SubmergeColors::CYAN50);
        }
        if style.trim() == "cyan-100" {
            return SubmergeColors::color(SubmergeColors::CYAN100);
        }
        if style.trim() == "cyan-200" {
            return SubmergeColors::color(SubmergeColors::CYAN200);
        }
        if style.trim() == "cyan-300" {
            return SubmergeColors::color(SubmergeColors::CYAN300);
        }
        if style.trim() == "cyan-400" {
            return SubmergeColors::color(SubmergeColors::CYAN400);
        }
        if style.trim() == "cyan-500" {
            return SubmergeColors::color(SubmergeColors::CYAN500);
        }
        if style.trim() == "cyan-600" {
            return SubmergeColors::color(SubmergeColors::CYAN600);
        }
        if style.trim() == "cyan-700" {
            return SubmergeColors::color(SubmergeColors::CYAN700);
        }
        if style.trim() == "cyan-800" {
            return SubmergeColors::color(SubmergeColors::CYAN800);
        }
        if style.trim() == "cyan-900" {
            return SubmergeColors::color(SubmergeColors::CYAN900);
        }
        if style.trim() == "sky-50" {
            return SubmergeColors::color(SubmergeColors::SKY50);
        }
        if style.trim() == "sky-100" {
            return SubmergeColors::color(SubmergeColors::SKY100);
        }
        if style.trim() == "sky-200" {
            return SubmergeColors::color(SubmergeColors::SKY200);
        }
        if style.trim() == "sky-300" {
            return SubmergeColors::color(SubmergeColors::SKY300);
        }
        if style.trim() == "sky-400" {
            return SubmergeColors::color(SubmergeColors::SKY400);
        }
        if style.trim() == "sky-500" {
            return SubmergeColors::color(SubmergeColors::SKY500);
        }
        if style.trim() == "sky-600" {
            return SubmergeColors::color(SubmergeColors::SKY600);
        }
        if style.trim() == "sky-700" {
            return SubmergeColors::color(SubmergeColors::SKY700);
        }
        if style.trim() == "sky-800" {
            return SubmergeColors::color(SubmergeColors::SKY800);
        }
        if style.trim() == "sky-900" {
            return SubmergeColors::color(SubmergeColors::SKY900);
        }
        if style.trim() == "blue-50" {
            return SubmergeColors::color(SubmergeColors::BLUE50);
        }
        if style.trim() == "blue-100" {
            return SubmergeColors::color(SubmergeColors::BLUE100);
        }
        if style.trim() == "blue-200" {
            return SubmergeColors::color(SubmergeColors::BLUE200);
        }
        if style.trim() == "blue-300" {
            return SubmergeColors::color(SubmergeColors::BLUE300);
        }
        if style.trim() == "blue-400" {
            return SubmergeColors::color(SubmergeColors::BLUE400);
        }
        if style.trim() == "blue-500" {
            return SubmergeColors::color(SubmergeColors::BLUE500);
        }
        if style.trim() == "blue-600" {
            return SubmergeColors::color(SubmergeColors::BLUE600);
        }
        if style.trim() == "blue-700" {
            return SubmergeColors::color(SubmergeColors::BLUE700);
        }
        if style.trim() == "blue-800" {
            return SubmergeColors::color(SubmergeColors::BLUE800);
        }
        if style.trim() == "blue-900" {
            return SubmergeColors::color(SubmergeColors::BLUE900);
        }
        if style.trim() == "indigo-50" {
            return SubmergeColors::color(SubmergeColors::INDIGO50);
        }
        if style.trim() == "indigo-100" {
            return SubmergeColors::color(SubmergeColors::INDIGO100);
        }
        if style.trim() == "indigo-200" {
            return SubmergeColors::color(SubmergeColors::INDIGO200);
        }
        if style.trim() == "indigo-300" {
            return SubmergeColors::color(SubmergeColors::INDIGO300);
        }
        if style.trim() == "indigo-400" {
            return SubmergeColors::color(SubmergeColors::INDIGO400);
        }
        if style.trim() == "indigo-500" {
            return SubmergeColors::color(SubmergeColors::INDIGO500);
        }
        if style.trim() == "indigo-600" {
            return SubmergeColors::color(SubmergeColors::INDIGO600);
        }
        if style.trim() == "indigo-700" {
            return SubmergeColors::color(SubmergeColors::INDIGO700);
        }
        if style.trim() == "indigo-800" {
            return SubmergeColors::color(SubmergeColors::INDIGO800);
        }
        if style.trim() == "indigo-900" {
            return SubmergeColors::color(SubmergeColors::INDIGO900);
        }
        if style.trim() == "violet-50" {
            return SubmergeColors::color(SubmergeColors::VIOLET50);
        }
        if style.trim() == "violet-100" {
            return SubmergeColors::color(SubmergeColors::VIOLET100);
        }
        if style.trim() == "violet-200" {
            return SubmergeColors::color(SubmergeColors::VIOLET200);
        }
        if style.trim() == "violet-300" {
            return SubmergeColors::color(SubmergeColors::VIOLET300);
        }
        if style.trim() == "violet-400" {
            return SubmergeColors::color(SubmergeColors::VIOLET400);
        }
        if style.trim() == "violet-500" {
            return SubmergeColors::color(SubmergeColors::VIOLET500);
        }
        if style.trim() == "violet-600" {
            return SubmergeColors::color(SubmergeColors::VIOLET600);
        }
        if style.trim() == "violet-700" {
            return SubmergeColors::color(SubmergeColors::VIOLET700);
        }
        if style.trim() == "violet-800" {
            return SubmergeColors::color(SubmergeColors::VIOLET800);
        }
        if style.trim() == "violet-900" {
            return SubmergeColors::color(SubmergeColors::VIOLET900);
        }
        if style.trim() == "purple-50" {
            return SubmergeColors::color(SubmergeColors::PURPLE50);
        }
        if style.trim() == "purple-100" {
            return SubmergeColors::color(SubmergeColors::PURPLE100);
        }
        if style.trim() == "purple-200" {
            return SubmergeColors::color(SubmergeColors::PURPLE200);
        }
        if style.trim() == "purple-300" {
            return SubmergeColors::color(SubmergeColors::PURPLE300);
        }
        if style.trim() == "purple-400" {
            return SubmergeColors::color(SubmergeColors::PURPLE400);
        }
        if style.trim() == "purple-500" {
            return SubmergeColors::color(SubmergeColors::PURPLE500);
        }
        if style.trim() == "purple-600" {
            return SubmergeColors::color(SubmergeColors::PURPLE600);
        }
        if style.trim() == "purple-700" {
            return SubmergeColors::color(SubmergeColors::PURPLE700);
        }
        if style.trim() == "purple-800" {
            return SubmergeColors::color(SubmergeColors::PURPLE800);
        }
        if style.trim() == "purple-900" {
            return SubmergeColors::color(SubmergeColors::PURPLE900);
        }
        if style.trim() == "fuchsia-50" {
            return SubmergeColors::color(SubmergeColors::FUCHSIA50);
        }
        if style.trim() == "fuchsia-100" {
            return SubmergeColors::color(SubmergeColors::FUCHSIA100);
        }
        if style.trim() == "fuchsia-200" {
            return SubmergeColors::color(SubmergeColors::FUCHSIA200);
        }
        if style.trim() == "fuchsia-300" {
            return SubmergeColors::color(SubmergeColors::FUCHSIA300);
        }
        if style.trim() == "fuchsia-400" {
            return SubmergeColors::color(SubmergeColors::FUCHSIA400);
        }
        if style.trim() == "fuchsia-500" {
            return SubmergeColors::color(SubmergeColors::FUCHSIA500);
        }
        if style.trim() == "fuchsia-600" {
            return SubmergeColors::color(SubmergeColors::FUCHSIA600);
        }
        if style.trim() == "fuchsia-700" {
            return SubmergeColors::color(SubmergeColors::FUCHSIA700);
        }
        if style.trim() == "fuchsia-800" {
            return SubmergeColors::color(SubmergeColors::FUCHSIA800);
        }
        if style.trim() == "fuchsia-900" {
            return SubmergeColors::color(SubmergeColors::FUCHSIA900);
        }
        if style.trim() == "pink-50" {
            return SubmergeColors::color(SubmergeColors::PINK50);
        }
        if style.trim() == "pink-100" {
            return SubmergeColors::color(SubmergeColors::PINK100);
        }
        if style.trim() == "pink-200" {
            return SubmergeColors::color(SubmergeColors::PINK200);
        }
        if style.trim() == "pink-300" {
            return SubmergeColors::color(SubmergeColors::PINK300);
        }
        if style.trim() == "pink-400" {
            return SubmergeColors::color(SubmergeColors::PINK400);
        }
        if style.trim() == "pink-500" {
            return SubmergeColors::color(SubmergeColors::PINK500);
        }
        if style.trim() == "pink-600" {
            return SubmergeColors::color(SubmergeColors::PINK600);
        }
        if style.trim() == "pink-700" {
            return SubmergeColors::color(SubmergeColors::PINK700);
        }
        if style.trim() == "pink-800" {
            return SubmergeColors::color(SubmergeColors::PINK800);
        }
        if style.trim() == "pink-900" {
            return SubmergeColors::color(SubmergeColors::PINK900);
        }
        if style.trim() == "rose-50" {
            return SubmergeColors::color(SubmergeColors::ROSE50);
        }
        if style.trim() == "rose-100" {
            return SubmergeColors::color(SubmergeColors::ROSE100);
        }
        if style.trim() == "rose-200" {
            return SubmergeColors::color(SubmergeColors::ROSE200);
        }
        if style.trim() == "rose-300" {
            return SubmergeColors::color(SubmergeColors::ROSE300);
        }
        if style.trim() == "rose-400" {
            return SubmergeColors::color(SubmergeColors::ROSE400);
        }
        if style.trim() == "rose-500" {
            return SubmergeColors::color(SubmergeColors::ROSE500);
        }
        if style.trim() == "rose-600" {
            return SubmergeColors::color(SubmergeColors::ROSE600);
        }
        if style.trim() == "rose-700" {
            return SubmergeColors::color(SubmergeColors::ROSE700);
        }
        if style.trim() == "rose-800" {
            return SubmergeColors::color(SubmergeColors::ROSE800);
        }
        if style.trim() == "rose-900" {
            return SubmergeColors::color(SubmergeColors::ROSE900);
        }
        if style.trim() == "black" {
            return SubmergeColors::color(SubmergeColors::BLACK);
        }
        if style.trim() == "white" {
            return SubmergeColors::color(SubmergeColors::WHITE);
        }
        return SubmergeColors::color(SubmergeColors::BLACK);
    }

    pub const fn color(self) -> Color {
        let (v1, v2, v3) = self.get_rgb();
        let color = Color::srgb(v1, v2, v3);
        // println!("color: {:?}", color);
        color
        // println!("converted to something else: {:?}, {:?}, {:?}", v1.clone(), v2, v3);
    }
}
