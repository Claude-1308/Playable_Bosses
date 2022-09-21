use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smashline::*;
use crate::FIGHTER_MANAGER;
use crate::ITEM_MANAGER;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::hooks::InlineCtx;

use crate::common::*;
use crate::common::{modules::*,params::*};

pub unsafe fn galleom_missile_fix(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if StatusModule::status_kind(module_accessor) == *ITEM_GALLEOM_STATUS_KIND_MISSILE {
        if MotionModule::rate(module_accessor) <= 1.0 {
            MotionModule::set_rate(module_accessor,2.0);
        }
    }
}

pub unsafe fn galleom_uppercut_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("uppercut"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_double_arm"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_uppercut_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 57.0
    && MotionModule::frame(module_accessor) < 68.0 {
        MotionModule::set_rate(module_accessor,1.0);
    }
    else {
        MotionModule::set_rate(module_accessor,2.0);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_hammer_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("hammer_knuckle"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_hammer_knuckle"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_hammer_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 85.0
    && MotionModule::frame(module_accessor) < 101.0 {
        MotionModule::set_rate(module_accessor,1.0);
    }
    else {
        MotionModule::set_rate(module_accessor,2.0);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_foot_crush_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("foot_crush"),0.0,1.5,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_foot_crush"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_foot_crush_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 49.0
    && MotionModule::frame(module_accessor) < 55.0 {
        MotionModule::set_rate(module_accessor,1.0);
    }
    else {
        MotionModule::set_rate(module_accessor,1.5);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_specials(item: &mut L2CAgentBase) {
    let lioleusboss_fireball_shot3_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_fireball_shot3_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_fireball_shot3_end_coroutine_func);
    let lioleusboss_fireball_shot3_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_fireball_shot3_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_fireball_shot3_end_status_func);

    let lioleusboss_tackle_loop_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_loop_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_tackle_loop_status_func);

    let lioleusboss_tackle_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_tackle_end_coroutine_func);
    let lioleusboss_tackle_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_tackle_end_status_func);

    let lioleusboss_tackle_turn_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_turn_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_TURN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_tackle_turn_coroutine_func);
    let lioleusboss_tackle_turn_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_turn_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_TURN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_tackle_turn_status_func);
}