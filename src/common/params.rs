pub struct Common;

impl Common {
    pub const rebirth_timer: i32 = 600;
    pub const ai_level: f32 = 10.0;
    pub const min_stick: f32 = 0.3;
    pub const dead_slow: u8 = 10;
    pub const cutscene_timer: i32 = 6;
    pub const non_last_stock_mt_rate: f32 = 2.0;
}

pub struct Masterhand;

impl Masterhand {
    pub const finish_frame: i32 = 260;
    pub const move_mul: f32 = 2.0;
    pub const health: f32 = 300.0;
}

pub struct Crazyhand;

impl Crazyhand {
    pub const finish_frame: i32 = 260;
    pub const move_mul: f32 = 2.0;
    pub const health: f32 = 300.0;
}

pub struct Ganonboss;

impl Ganonboss {
    pub const finish_frame: i32 = 280;
    pub const playable_fs: bool = false;
    pub const fs_timer: i32 = 600;
    pub const health: f32 = 300.0;
}

pub struct Lioleusboss;

impl Lioleusboss {
    pub const health: f32 = 600.0;
    pub const extra_health_mul: f32 = 2.0;
    pub const finish_frame: i32 = 230;
    pub const move_mul_x: f32 = 2.0;
    pub const move_mul_y: f32 = 1.0;
}

pub struct Galleom;

impl Galleom {
    pub const finish_frame: i32 = 170;
    pub const health: f32 = 900.0;
    pub const extra_health_mul: f32 = 3.0;
}

pub struct Marx;

impl Marx {
    pub const finish_frame: i32 = 120;
    pub const move_mul: f32 = 2.0;
    pub const health: f32 = 300.0;
}