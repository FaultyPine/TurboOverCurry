use smash::phx::*;

pub static mut GLOBAL_FRAME_COUNT: [i32;8] = [0;8];
pub static mut AQUIRED_TURBO_FRAME: [i32;8] = [0;8];

pub const TURBO_DURATION: i32 = 1200; // length of turbo mode (in frames) -- must be greater than "gekikara_frame" in item.prc

pub const TURBO_ACTIVE_EFFECT_STR: &str = "sys_revenge_aura";
pub const TURBO_DEACTIVATE_EFFECT_STR: &str = "sys_sp_flash";

pub const TURBO_ACTIVATE_EFFECT_OFFSET_FROM_TOP: Vector3f = Vector3f {x: 0.0, y: 7.0, z: 0.0};
pub const TURBO_DEACTIVATE_EFFECT_OFFSET_FROM_TOP: Vector3f = Vector3f {x: 7.0, y: 18.0, z: 0.0};
pub const DEFAULT_VEC: Vector3f = Vector3f {x: 0.0, y: 0.0, z: 0.0};