use smash::phx::*;

pub static mut IS_TURBO: [bool;8] = [false;8];

pub const TURBO_ACTIVE_EFFECT_STR: &str = "sys_revenge_aura";
pub const TURBO_DEACTIVATE_EFFECT_STR: &str = "sys_sp_flash";

pub const TURBO_ACTIVATE_EFFECT_OFFSET_FROM_TOP: Vector3f = Vector3f {x: 0.0, y: 7.0, z: 0.0};
pub const TURBO_DEACTIVATE_EFFECT_OFFSET_FROM_TOP: Vector3f = Vector3f {x: 7.0, y: 18.0, z: 0.0};
pub const DEFAULT_VEC: Vector3f = Vector3f {x: 0.0, y: 0.0, z: 0.0};