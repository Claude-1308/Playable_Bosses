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
use std::arch::asm;

use crate::common::*;
use crate::common::{modules::*,params::*};

pub unsafe fn kiila_summon_fighter_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("summon_light_fighter"),0.0,1.5,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_summon_light_fighter"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_summon_fighter_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        set_visibility_whole_force(lua_state,false);
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_SUMMON_FIGHTER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub static mut KIILA_SUMMON_FIX: usize = 0x47099c;

#[skyline::hook(replace = KIILA_SUMMON_FIX, inline)]
pub unsafe fn kiila_summon_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    let module_accessor = agent_base.module_accessor;
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let owner = BossModule::get_owner(&mut *module_accessor);
    WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON);
    *ctx.registers[0].w.as_mut() = *ITEM_KIILA_STATUS_KIND_TELEPORT as u32;
}

pub static mut KIILA_MISSILE_STOP_FRAME_FIX: usize = 0x46e3f0;

#[skyline::hook(replace = KIILA_MISSILE_STOP_FRAME_FIX, inline)]
pub unsafe fn kiila_missile_stop_frame_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 60u32;
}

pub static mut KIILA_MISSILE_FIX: usize = 0x46e8b8;

#[skyline::hook(replace = KIILA_MISSILE_FIX, inline)]
pub unsafe fn kiila_missile_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = *ITEM_KIILA_STATUS_KIND_MANAGER_WAIT as u32;
}

pub static mut KIILA_LASER_TIME_NUM_FIX: usize = 0x476140;

#[skyline::hook(replace = KIILA_LASER_TIME_NUM_FIX, inline)]
pub unsafe fn kiila_laser_time_num_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 2u32;
}

pub static mut KIILA_LASER_NUM_FIX: usize = 0x476754;

#[skyline::hook(replace = KIILA_LASER_NUM_FIX, inline)]
pub unsafe fn kiila_laser_num_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 20u32;
}

pub static mut KIILA_LASER_WAIT_TIME_FIX: usize = 0x476b84;

#[skyline::hook(replace = KIILA_LASER_WAIT_TIME_FIX, inline)]
pub unsafe fn kiila_laser_wait_time_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 45u32;
}

pub unsafe fn kiila_laser_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("laser_end"),0.0,1.5,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_laser_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_laser_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_MANAGER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn kiila_threat_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("threat_start"),0.0,1.2,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_threat_start"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_threat_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_THREAT_LOOP,false);
    }
    return L2CValue::I32(0)
}

pub static mut KIILA_THREAT_NUM_FIX: usize = 0x46c338;

#[skyline::hook(replace = KIILA_THREAT_NUM_FIX, inline)]
pub unsafe fn kiila_threat_num_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 3u32;
}

pub unsafe fn kiila_threat_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("threat_end"),0.0,1.2,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_threat_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_threat_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_MANAGER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_specials(item: &mut L2CAgentBase) {
    let kiila_summon_fighter_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_summon_fighter_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_SUMMON_FIGHTER),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_summon_fighter_coroutine_func);
    let kiila_summon_fighter_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_summon_fighter_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_SUMMON_FIGHTER),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_summon_fighter_status_func);

    let kiila_laser_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_laser_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_LASER_RUSH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_laser_end_coroutine_func);
    let kiila_laser_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_laser_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_LASER_RUSH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_laser_end_status_func);

    let kiila_threat_start_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_threat_start_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_THREAT_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_threat_start_coroutine_func);
    let kiila_threat_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_threat_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_THREAT_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_threat_start_status_func);

    let kiila_threat_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_threat_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_THREAT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_threat_end_coroutine_func);
    let kiila_threat_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_threat_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_THREAT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_threat_end_status_func);
}