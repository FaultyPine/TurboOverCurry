use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use crate::utils;
use crate::vars::*;
use smash::phx::*;
use smash::hash40;

/*  TODO
- get better effect(s) for activate/deactivate/active

- model/animate turbo item
- change ingame text
*/

//called once per frame while turbo mode is on
unsafe fn turbo_mode(boma: &mut app::BattleObjectModuleAccessor, lua_state: u64) {
    //actual turbo logic... allows cancels if you've hit someone, or someone's shield
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        CancelModule::enable_cancel(boma);
    }
    app::sv_animcmd::FT_DISABLE_CURRY_FACE(lua_state); //forces normal expression instead of curry surprised face
}

//called once when turbo should be "turned off"
unsafe fn reset_turbo_mode(boma: &mut app::BattleObjectModuleAccessor, entry_id: usize) {
    if AQUIRED_TURBO_FRAME[entry_id] != 0 {
        AQUIRED_TURBO_FRAME[entry_id] = 0;
        EffectModule::req_on_joint(boma, Hash40::new(TURBO_DEACTIVATE_EFFECT_STR), Hash40::new("top"),
            &TURBO_DEACTIVATE_EFFECT_OFFSET_FROM_TOP, &DEFAULT_VEC, 1.5, &DEFAULT_VEC, &DEFAULT_VEC,
            false, 0, 0, 0);
        EffectModule::kill_kind(boma, Hash40::new(TURBO_ACTIVE_EFFECT_STR), true, true);
    }
}

//called once when turbo should be "turned on"
pub unsafe fn turbo_activate(boma: &mut app::BattleObjectModuleAccessor, entry_id: usize) {
    AQUIRED_TURBO_FRAME[entry_id] = GLOBAL_FRAME_COUNT[entry_id];
    let eff_size = WorkModule::get_param_float(boma, hash40("shield_radius"), 0) / 7.8;
    EffectModule::req_follow(boma, Hash40::new(TURBO_ACTIVE_EFFECT_STR), Hash40::new("top"), &TURBO_ACTIVATE_EFFECT_OFFSET_FROM_TOP, &DEFAULT_VEC, eff_size, false, 0, 0, 0, 0, 0, false, false);
}


fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if utils::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let entry_id = utils::get_player_number(boma);
            GLOBAL_FRAME_COUNT[entry_id] += 1;

            //if current player has turbo
            if AQUIRED_TURBO_FRAME[entry_id] != 0 { 
                turbo_mode(boma, fighter.lua_state_agent);
            }

            //if turbo time limit is up, or if you die
            if (GLOBAL_FRAME_COUNT[entry_id] - AQUIRED_TURBO_FRAME[entry_id] >= TURBO_DURATION && AQUIRED_TURBO_FRAME[entry_id] != 0) || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DEAD {
                reset_turbo_mode(boma, entry_id);
            }

            // yeah i know this is a stupid way to do it but its easy and it works 😛
            //prevent all curry vfx
            EffectModule::kill_kind(boma, Hash40::new("sys_curry_shot"), false, true);
            EffectModule::kill_kind(boma, Hash40::new("sys_curry_fire"), false, true);
            EffectModule::kill_kind(boma, Hash40::new("sys_curry_steam"), false, true);
            //prevent all curry sfx
            SoundModule::stop_se(boma, Hash40::new("se_item_curry_fire"), 0);
            SoundModule::stop_se(boma, Hash40::new("se_item_curry_fire_sp"), 0);
            SoundModule::stop_se(boma, Hash40::new("se_item_curry_shot"), 0);
            SoundModule::stop_se(boma, Hash40::new("se_item_curry_shot_b"), 0);

            //handles resets like going into different matches/games, and training mode reset
            handle_game_resets(boma, entry_id);
        
        }
    }
}


unsafe fn handle_game_resets(boma: &mut app::BattleObjectModuleAccessor, entry_id: usize) {
    static mut LAST_READY_GO: bool = true;
    static mut IS_READY_GO: bool = false;
    IS_READY_GO = utils::is_ready_go();

    //THIS BLOCK RUNS WHEN A GAME/MATCH STARTS UP (I.E. match start, training mode start, training mode reset, etc...)
    if !IS_READY_GO && LAST_READY_GO {
        reset_turbo_mode(boma, entry_id);
    }
    LAST_READY_GO = IS_READY_GO;
}

//prevents curry idle anim where they do that little spicy dance thing
#[skyline::hook(replace = MotionModule::change_motion)]
unsafe fn change_motion_hook(boma: &mut app::BattleObjectModuleAccessor, motion: Hash40, arg3: f32, arg4: f32, arg5: bool, arg6: f32, arg7: bool, arg8: bool) -> u64 {
    let mut new_hash = motion.hash;
    if motion.hash == 81108382147 { //curry idle motion
        new_hash = hash40("wait");
    }
    original!()(boma, Hash40::new_raw(new_hash), arg3, arg4, arg5, arg6, arg7, arg8)
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    skyline::install_hook!(change_motion_hook);
}