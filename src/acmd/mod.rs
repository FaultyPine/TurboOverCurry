use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterBase;
use acmd::{acmd, acmd_func};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_XXXXX, 
    battle_object_kind = THING_KIND_XXXXX, 
    animation = "",
    animcmd = "")]
pub fn func_here(fighter : &mut L2CFighterBase) {
acmd!({
    
});
}

pub fn install() {
    acmd::add_hooks!(
        
    );
}