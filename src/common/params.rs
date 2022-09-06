pub struct Common;

impl Common {
    pub const rebirth_timer: i32 = 600;
    pub const ai_level: f32 = 10.0;
    pub const base_dmg_mul: f32 = 0.5;
    pub const min_stick: f32 = 0.3;
    pub const dead_slow: u8 = 10;
    pub const cutscene_timer: i32 = 6;
    pub const non_last_stock_mt_rate: f32 = 2.0;
}

pub struct Masterhand;

impl Masterhand {
    pub const finish_frame: i32 = 260;
    pub const move_mul: f32 = 2.0;
}

pub struct Crazyhand;

impl Crazyhand {
    pub const finish_frame: i32 = 260;
    pub const move_mul: f32 = 2.0;
}

pub struct Ganonboss;

impl Ganonboss {
    pub const finish_frame: i32 = 280;
    pub const playable_fs: bool = false;
    pub const fs_timer: i32 = 600;
}