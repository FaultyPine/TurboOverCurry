use smash::app;
use smash::lib::{L2CValue, L2CAgent};
use smash::lua2cpp::L2CFighterAnimcmdExpressionCommon;
use crate::vars::*;
use crate::utils;

//uses nro hook
pub fn install() {
    skyline::install_hook!(curr_common_expression_hook);
}

//for some reason this seems to be the only func that actually runs when curry is being used... the others just didn't get called
#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdExpressionCommon_expression_FireCurryRumble)]
unsafe fn curr_common_expression_hook(expression_common: *mut L2CFighterAnimcmdExpressionCommon) -> L2CValue {
    let l2c_agent: L2CAgent = (*expression_common).agent;
    let boma = app::sv_system::battle_object_module_accessor(l2c_agent.lua_state_agent);
    let entry_id = utils::get_player_number(boma);

    //if turbo isn't already on
    if AQUIRED_TURBO_FRAME[entry_id] == 0 {
        crate::once_per_frame::turbo_activate(boma, entry_id);
    }

    return L2CValue::new_void();
}