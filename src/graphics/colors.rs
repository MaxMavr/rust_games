#[warn(unused_assignments)]
pub const RED: u32 = 0xFF0000;
pub const GREEN: u32 = 0x008000;
pub const BLUE: u32 = 0x0000FF;
pub const YELLOW: u32 = 0xFFFF00;
pub const CYAN: u32 = 0x00FFFF;
pub const MAGENTA: u32 = 0xFF00FF;

pub const WHITE: u32 = 0xFFFFFF;
pub const SILVER: u32 = 0xC0C0C0;
pub const GRAY: u32 = 0x808080;
pub const BLACK: u32 = 0x000000;

pub const MAROON: u32 = 0x800000;
pub const DARK_RED: u32 = 0x8B0000;
pub const CRIMSON: u32 = 0xDC143C;
pub const FIREBRICK: u32 = 0xB22222;
pub const SALMON: u32 = 0xFA8072;
pub const TOMATO: u32 = 0xFF6347;
pub const ORANGE_RED: u32 = 0xFF4500;
pub const PINK: u32 = 0xFFC0CB;
pub const HOT_PINK: u32 = 0xFF69B4;
pub const DEEP_PINK: u32 = 0xFF1493;

pub const ORANGE: u32 = 0xFFA500;
pub const GOLD: u32 = 0xFFD700;
pub const KHAKI: u32 = 0xF0E68C;
pub const LIGHT_YELLOW: u32 = 0xFFFFE0;
pub const LEMON_CHIFFON: u32 = 0xFFFACD;

pub const LIME: u32 = 0x00FF00;
pub const LIME_GREEN: u32 = 0x32CD32;
pub const FOREST_GREEN: u32 = 0x228B22;
pub const SEA_GREEN: u32 = 0x2E8B57;
pub const OLIVE: u32 = 0x808000;
pub const TEAL: u32 = 0x008080;
pub const AQUAMARINE: u32 = 0x7FFFD4;

pub const NAVY: u32 = 0x000080;
pub const DARK_BLUE: u32 = 0x00008B;
pub const MEDIUM_BLUE: u32 = 0x0000CD;
pub const ROYAL_BLUE: u32 = 0x4169E1;
pub const STEEL_BLUE: u32 = 0x4682B4;
pub const SKY_BLUE: u32 = 0x87CEEB;
pub const LIGHT_BLUE: u32 = 0xADD8E6;
pub const POWDER_BLUE: u32 = 0xB0E0E6;
pub const CORNFLOWER_BLUE: u32 = 0x6495ED;
pub const DODGER_BLUE: u32 = 0x1E90FF;

pub const PURPLE: u32 = 0x800080;
pub const INDIGO: u32 = 0x4B0082;
pub const VIOLET: u32 = 0xEE82EE;
pub const PLUM: u32 = 0xDDA0DD;
pub const ORCHID: u32 = 0xDA70D6;
pub const MEDIUM_PURPLE: u32 = 0x9370DB;
pub const SLATE_BLUE: u32 = 0x6A5ACD;

pub const BROWN: u32 = 0xA52A2A;
pub const SADDLE_BROWN: u32 = 0x8B4513;
pub const CHOCOLATE: u32 = 0xD2691E;
pub const PERU: u32 = 0xCD853F;
pub const SANDY_BROWN: u32 = 0xF4A460;
pub const BEIGE: u32 = 0xF5F5DC;

pub trait ToColor {
    fn to_color(&self) -> u32;
}

impl ToColor for u32 {
    fn to_color(&self) -> u32 {
        *self
    }
}
