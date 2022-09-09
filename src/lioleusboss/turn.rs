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
use crate::common::modules::*;

pub unsafe fn turn_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("tail"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_turn"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn turn_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        PostureModule::update_rot_y_lr(module_accessor);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn turn_air_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("turn_air"),0.0,0.6,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_fly_turn"),0.0);
    boss_private::sub1_energy_from_param_inherit(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_fly_brake"));
    return L2CValue::I32(0)
}

pub unsafe fn turn_air_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        PostureModule::update_rot_y_lr(module_accessor);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_step_air_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["lioleusboss_step_air_coroutine"].get_ptr());
    original(item);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_turn(item: &mut L2CAgentBase) {
    let turn_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(turn_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TAIL),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),turn_coroutine_func);
    let turn_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(turn_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TAIL),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),turn_status_func);

    let turn_air_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(turn_air_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_TURN_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),turn_air_coroutine_func);
    let turn_air_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(turn_air_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_TURN_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),turn_air_status_func);

    let owner = BossModule::get_owner(&mut *item.module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    fighter.global_table["lioleusboss_step_air_coroutine"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_STEP_AIR),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE)));

    let lioleusboss_step_air_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_step_air_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_STEP_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_step_air_coroutine_func);
}