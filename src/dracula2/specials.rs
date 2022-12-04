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
use crate::dracula2::move_input::*;

pub static mut DRACULA2_FIRE_SHOT_FIX: usize = 0x402258;

#[skyline::hook(replace = DRACULA2_FIRE_SHOT_FIX, inline)]
pub unsafe fn dracula2_fire_shot_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = *ITEM_DRACULA2_STATUS_KIND_FIRE_SHOT_END as u32;
}

pub static mut DRACULA2_SHOCKWAVE_FIX: usize = 0x40384c;

#[skyline::hook(replace = DRACULA2_SHOCKWAVE_FIX, inline)]
pub unsafe fn dracula2_shockwave_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = *ITEM_DRACULA2_STATUS_KIND_WAIT as u32;
}

pub static mut DRACULA2_TURN_SHOCKWAVE_FIX: usize = 0x403b9c;

#[skyline::hook(replace = DRACULA2_TURN_SHOCKWAVE_FIX, inline)]
pub unsafe fn dracula2_turn_shockwave_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = *ITEM_DRACULA2_STATUS_KIND_WAIT as u32;
}

pub unsafe fn dracula2_fire_shot_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("fire_shot_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_fire_shot_end"),0.0);
    boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_common_brake"));
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_fire_shot_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA2_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_homing_shot_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("homing_shot_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_homing_shot_end"),0.0);
    boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_common_brake"));
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_homing_shot_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA2_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_step_strike_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("step_strike"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA2),Hash40::new("energy_param_step_strike"),0.0);
    AttackModule::set_power_mul(module_accessor,1.5);
    AttackModule::set_reaction_mul(module_accessor,1.5);
    return L2CValue::I32(0)
}

pub unsafe fn dracula2_step_strike_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA2_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_specials(item: &mut L2CAgentBase) {
    let dracula2_fire_shot_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_fire_shot_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_FIRE_SHOT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula2_fire_shot_end_coroutine_func);
    let dracula2_fire_shot_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_fire_shot_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_FIRE_SHOT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula2_fire_shot_end_status_func);

    let dracula2_homing_shot_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_homing_shot_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_HOMING_SHOT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula2_homing_shot_end_coroutine_func);
    let dracula2_homing_shot_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_homing_shot_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_HOMING_SHOT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula2_homing_shot_end_status_func);

    let dracula2_step_strike_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_step_strike_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_STEP_STRIKE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula2_step_strike_coroutine_func);
    let dracula2_step_strike_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula2_step_strike_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA2_STATUS_KIND_STEP_STRIKE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula2_step_strike_status_func);
}