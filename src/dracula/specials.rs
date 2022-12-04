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
use crate::common::modules::*;
use crate::dracula::move_input::*;

pub unsafe fn dracula_straight_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("attack_straight_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA),Hash40::new("energy_param_straight_end"),0.0);
    boss_private::clear_sub1_energy(lua_state);
    boss_private::unable_sub1_energy(lua_state);
    return L2CValue::I32(0)
}

pub unsafe fn dracula_straight_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula_3way_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("attack_3way_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA),Hash40::new("energy_param_3way_end"),0.0);
    boss_private::clear_sub1_energy(lua_state);
    boss_private::unable_sub1_energy(lua_state);
    return L2CValue::I32(0)
}

pub unsafe fn dracula_3way_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula_turn_3way_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("attack_turn_3way_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA),Hash40::new("energy_param_turn_3way_end"),0.0);
    boss_private::clear_sub1_energy(lua_state);
    boss_private::unable_sub1_energy(lua_state);
    return L2CValue::I32(0)
}

pub unsafe fn dracula_turn_3way_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula_pillar_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("attack_pillar_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA),Hash40::new("energy_param_pillar_end"),0.0);
    boss_private::clear_sub1_energy(lua_state);
    boss_private::unable_sub1_energy(lua_state);
    return L2CValue::I32(0)
}

pub unsafe fn dracula_pillar_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula_change_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_PLAY_ENTRY) == false {
        MotionModule::change_motion(module_accessor,Hash40::new("change_start"),35.0,1.0,false,0.0,false,false);
        boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA),Hash40::new("energy_param_change_start"),0.0);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("dead"),0.0,1.0,false,0.0,false,false);
    }
    WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
    return L2CValue::I32(0)
}

pub unsafe fn dracula_change_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_OFF),0);
    if MotionModule::is_end(module_accessor)
    && WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_PLAY_ENTRY) == false {
        WorkModule::set_int(owner,BossKind::DRACULA2,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_KIND);
        BossModule::summon_boss(&mut *owner);
        StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_LOST,false);
    }
    return L2CValue::I32(0)
}

pub static mut DRACULA_STRAIGHT_SHOT_FIX: usize = 0x3ed8e8;

#[skyline::hook(replace = DRACULA_STRAIGHT_SHOT_FIX, inline)]
pub unsafe fn dracula_straight_shot_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[28].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let num: f32 = sv_math::rand(hash40("fighter"),3) as f32 + 4.0;
    asm!("fmov s0, w8", in("w8") num);
}

pub static mut DRACULA_PILLAR_NUM_FIX: usize = 0x3f79e0;

#[skyline::hook(replace = DRACULA_PILLAR_NUM_FIX, inline)]
pub unsafe fn dracula_pillar_num_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let num: f32 = sv_math::rand(hash40("fighter"),4) as f32 + 4.0;
    asm!("fmov s0, w8", in("w8") num);
}

pub static mut DRACULA_PILLAR_FRAME_FIX: usize = 0x3f7958;

#[skyline::hook(replace = DRACULA_PILLAR_FRAME_FIX, inline)]
pub unsafe fn dracula_pillar_frame_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let num: f32 = 20.0;
    asm!("fmov s0, w8", in("w8") num);
}

pub unsafe fn install_specials(item: &mut L2CAgentBase) {
    let dracula_straight_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_straight_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_STRAIGHT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula_straight_end_coroutine_func);
    let dracula_straight_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_straight_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_STRAIGHT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula_straight_end_status_func);

    let dracula_3way_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_3way_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_3WAY_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula_3way_end_coroutine_func);
    let dracula_3way_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_3way_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_3WAY_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula_3way_end_status_func);

    let dracula_turn_3way_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_turn_3way_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_TURN_3WAY_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula_turn_3way_end_coroutine_func);
    let dracula_turn_3way_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_turn_3way_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_TURN_3WAY_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula_turn_3way_end_status_func);

    let dracula_pillar_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_pillar_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_PILLAR_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula_pillar_end_coroutine_func);
    let dracula_pillar_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_pillar_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_PILLAR_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula_pillar_end_status_func);
}