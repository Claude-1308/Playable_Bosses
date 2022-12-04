use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use crate::FIGHTER_MANAGER;
use crate::ITEM_MANAGER;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::hooks::InlineCtx;

use crate::common::*;
use crate::common::modules::*;

#[status_script(agent = "koopag", status = FIGHTER_STATUS_KIND_DEAD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn dead(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    fighter.status_DeadSub();
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        MotionModule::change_motion(module_accessor,Hash40::new("death_air"),0.0,1.0,false,0.0,false,false);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("death"),0.0,1.0,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(dead_main_loop as *const () as _))
}

unsafe fn dead_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[0x17].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,Hash40::new("death"),-1.0,1.0,0.0);
    }
    if fighter.global_table[0x17].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,Hash40::new("death_air"),-1.0,1.0,0.0);
    }
    if MotionModule::is_end(module_accessor) {
        WorkModule::off_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES);
        fighter.change_status(FIGHTER_STATUS_KIND_STANDBY.into(),false.into());
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "koopag", status = FIGHTER_STATUS_KIND_ESCAPE_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn escape_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Wait()
}