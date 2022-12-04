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
use crate::dracula2::*;

pub unsafe fn dracula2_slash_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("slash"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_slash"),0.0);
    AttackModule::set_power_mul(module_accessor,2.0);
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_slash_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA2_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_turn_slash_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("turn_slash"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_turn_slash"),0.0);
    AttackModule::set_power_mul(module_accessor,2.0);
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_turn_slash_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA2_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_step_slash_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("step_slash"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_step_slash"),0.0);
    AttackModule::set_power_mul(module_accessor,2.0);
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_step_slash_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA2_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_strike_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("strike"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_strike"),0.0);
    AttackModule::set_reaction_mul(module_accessor,2.0);
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_strike_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA2_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_squash_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if ControlModule::get_stick_x(owner) * PostureModule::lr(module_accessor) >= 0.0 {
        MotionModule::change_motion(module_accessor,Hash40::new("squash_end"),0.0,1.0,false,0.0,false,false);
        boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_squash_end"),0.0);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("squash_end_turn"),0.0,1.0,false,0.0,false,false);
        boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_squash_end_turn"),0.0);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_squash_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA2_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let dracula2_slash_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_slash_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_SLASH),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula2_slash_coroutine_func);
    let dracula2_slash_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_slash_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_SLASH),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula2_slash_status_func);

    let dracula2_turn_slash_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_turn_slash_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_TURN_SLASH),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula2_turn_slash_coroutine_func);
    let dracula2_turn_slash_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_turn_slash_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_TURN_SLASH),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula2_turn_slash_status_func);

    let dracula2_step_slash_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_step_slash_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_STEP_SLASH),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula2_step_slash_coroutine_func);
    let dracula2_step_slash_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_step_slash_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_STEP_SLASH),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula2_step_slash_status_func);

    let dracula2_strike_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_strike_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_STRIKE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula2_strike_coroutine_func);
    let dracula2_strike_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_strike_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_STRIKE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula2_strike_status_func);

    let dracula2_squash_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_squash_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_SQUASH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula2_squash_end_coroutine_func);
    let dracula2_squash_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_squash_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_SQUASH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula2_squash_end_status_func);
}