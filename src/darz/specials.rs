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

pub unsafe fn darz_summon_fighter_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("summon_fighter"),0.0,1.5,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DARZ),Hash40::new("energy_param_summon_fighter"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn darz_summon_fighter_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        set_visibility_whole_force(lua_state,false);
        StatusModule::change_status_request(module_accessor,*ITEM_DARZ_STATUS_KIND_SUMMON_FIGHTER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub static mut DARZ_SUMMON_FIX: usize = 0x3b574c;

#[skyline::hook(replace = DARZ_SUMMON_FIX, inline)]
pub unsafe fn darz_summon_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    let module_accessor = agent_base.module_accessor;
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let owner = BossModule::get_owner(&mut *module_accessor);
    WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON);
    *ctx.registers[0].w.as_mut() = *ITEM_DARZ_STATUS_KIND_TELEPORT as u32;
}

pub static mut DARZ_PIERCE_INTERVAL1_FIX: usize = 0x3b2164;

#[skyline::hook(replace = DARZ_PIERCE_INTERVAL1_FIX, inline)]
pub unsafe fn darz_pierce_interval1_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 60u32;
}

pub static mut DARZ_PIERCE_INTERVAL_FIX: usize = 0x3b2308;

#[skyline::hook(replace = DARZ_PIERCE_INTERVAL_FIX, inline)]
pub unsafe fn darz_pierce_interval_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 45u32;
}

pub static mut DARZ_PIERCE_FIX: usize = 0x3b24c8;

#[skyline::hook(replace = DARZ_PIERCE_FIX, inline)]
pub unsafe fn darz_pierce_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let owner = BossModule::get_owner(&mut *agent_base.module_accessor);
    WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON);
    *ctx.registers[0].w.as_mut() = *ITEM_DARZ_STATUS_KIND_TELEPORT as u32;
}

pub unsafe fn darz_pierce_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("teleport_start"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DARZ),Hash40::new("energy_param_teleport_start"),0.0);
    JostleModule::set_status(module_accessor,false);
    HitModule::set_status_all(module_accessor,HitStatus(*HIT_STATUS_XLU),0);
    return L2CValue::I32(0)
}

pub unsafe fn darz_pierce_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        HitModule::set_status_all(module_accessor,HitStatus(*HIT_STATUS_NORMAL),0);
        LinkModule::send_event_parents(module_accessor,*ITEM_LINK_NO_CREATEOWNER,Hash40::new_raw(0x14f1921eacu64));
        StatusModule::change_status_request(module_accessor,*ITEM_DARZ_STATUS_KIND_PIERCE_LOOP,false);
    }
    return L2CValue::I32(0)
}

pub static mut DARZ_TEAR_FIX: usize = 0x3adb48;

#[skyline::hook(replace = DARZ_TEAR_FIX, inline)]
pub unsafe fn darz_tear_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = *ITEM_DARZ_STATUS_KIND_MANAGER_WAIT as u32;
}

pub static mut DARZ_GATLING_FRAME_FIX: usize = 0x3ab100;

#[skyline::hook(replace = DARZ_GATLING_FRAME_FIX, inline)]
pub unsafe fn darz_gatling_frame_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 240u32;
}

pub static mut DARZ_GATLING_FIX: usize = 0x3ab248;

#[skyline::hook(replace = DARZ_GATLING_FIX, inline)]
pub unsafe fn darz_gatling_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = *ITEM_DARZ_STATUS_KIND_GATLING_ANGER as u32;
}

pub unsafe fn darz_gatling_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("gatling_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DARZ),Hash40::new("energy_param_gatling_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn darz_gatling_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_DARZ_STATUS_KIND_MANAGER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_specials(item: &mut L2CAgentBase) {
    let darz_summon_fighter_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_summon_fighter_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_SUMMON_FIGHTER),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),darz_summon_fighter_coroutine_func);
    let darz_summon_fighter_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_summon_fighter_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_SUMMON_FIGHTER),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),darz_summon_fighter_status_func);

    let darz_pierce_start_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_pierce_start_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_PIERCE_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),darz_pierce_start_coroutine_func);
    let darz_pierce_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_pierce_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_PIERCE_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),darz_pierce_start_status_func);

    let darz_gatling_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_gatling_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_GATLING_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),darz_gatling_end_coroutine_func);
    let darz_gatling_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_gatling_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_GATLING_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),darz_gatling_end_status_func);
}