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

pub unsafe fn kiila_cross_bomb_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    item__set_camera_range_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("cross_bomb_camera_subject"));
    MotionModule::change_motion(module_accessor,Hash40::new("cross_bomb"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_cross_bomb"),0.0);
    WorkModule::off_flag(module_accessor,*ITEM_KIILA_INSTANCE_WORK_FLAG_CROSS_BOMB_START);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_cross_bomb_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let mut rnd_vec = vec![0,1,2,3,4,5,6,7];
    if WorkModule::is_flag(module_accessor,*ITEM_KIILA_INSTANCE_WORK_FLAG_CROSS_BOMB_START) {
        for _ in 0..6 {
            let size = rnd_vec.len() as i32;
            let rng = sv_math::rand(hash40("item"),size) as usize;
            let x = item__self_struct_array_param_float(ItemKind(*ITEM_KIND_KIILA),Hash40::new("cross_bomb_pos_param"),rnd_vec[rng],Hash40::new("x"));
            let y = item__self_struct_array_param_float(ItemKind(*ITEM_KIND_KIILA),Hash40::new("cross_bomb_pos_param"),rnd_vec[rng],Hash40::new("y"));
            let bomb = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_KIILACROSSBOMB),x,y,0.0,1.0) as *mut BattleObjectModuleAccessor;
            if bomb.is_null() == false {
                WorkModule::set_int(bomb,40,*ITEM_KIILABOMB_INSTANCE_WORK_INT_DELAY);
            }
            if let Some(index) = rnd_vec.iter().position(|value| *value == rnd_vec[rng] as u64) {
                rnd_vec.remove(index);
            }
        }
        WorkModule::off_flag(module_accessor,*ITEM_KIILA_INSTANCE_WORK_FLAG_CROSS_BOMB_START);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_MANAGER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn kiila_energy_bomb_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("energy_bomb_start"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_energy_bomb_start"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_energy_bomb_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_ENERGY_SMART_BOMB_HOLD,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn kiila_energy_bomb_hold_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("energy_bomb_hold"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_energy_bomb_hold"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_energy_bomb_hold_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_ENERGY_SMART_BOMB,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn kiila_energy_bomb_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("energy_bomb_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_energy_bomb_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_energy_bomb_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_MANAGER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn kiila_explode_bomb_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("explode_bomb_start"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_explode_bomb_start"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_explode_bomb_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_EXPLODE_SHOT_LOOP,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn kiila_explode_bomb_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("explode_bomb_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_explode_bomb_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_explode_bomb_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_MANAGER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub static mut KIILA_CRUSH_DOWN_NUM_FIX: usize = 0x468988;

#[skyline::hook(replace = KIILA_CRUSH_DOWN_NUM_FIX, inline)]
pub unsafe fn kiila_crush_down_num_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 6u32;
}

pub static mut KIILA_CRUSH_DOWN_WAVE_NUM_FIX: usize = 0x4683bc;

#[skyline::hook(replace = KIILA_CRUSH_DOWN_WAVE_NUM_FIX, inline)]
pub unsafe fn kiila_crush_down_wave_num_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 2u32;
}

pub static mut KIILA_CRUSH_DOWN_WAIT_TIME_FIX: usize = 0x468db4;

#[skyline::hook(replace = KIILA_CRUSH_DOWN_WAIT_TIME_FIX, inline)]
pub unsafe fn kiila_crush_down_wait_time_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 15u32;
}

pub unsafe fn kiila_strike_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("strike_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_KIILA),Hash40::new("energy_param_strike_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn kiila_strike_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_MANAGER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let kiila_cross_bomb_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_cross_bomb_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_CROSS_BOMB),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_cross_bomb_coroutine_func);
    let kiila_cross_bomb_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_cross_bomb_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_CROSS_BOMB),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_cross_bomb_status_func);

    let kiila_energy_bomb_start_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_energy_bomb_start_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_ENERGY_SMART_BOMB_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_energy_bomb_start_coroutine_func);
    let kiila_energy_bomb_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_energy_bomb_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_ENERGY_SMART_BOMB_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_energy_bomb_start_status_func);

    let kiila_energy_bomb_hold_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_energy_bomb_hold_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_ENERGY_SMART_BOMB_HOLD),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_energy_bomb_hold_coroutine_func);
    let kiila_energy_bomb_hold_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_energy_bomb_hold_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_ENERGY_SMART_BOMB_HOLD),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_energy_bomb_hold_status_func);

    let kiila_energy_bomb_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_energy_bomb_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_ENERGY_SMART_BOMB_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_energy_bomb_end_coroutine_func);
    let kiila_energy_bomb_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_energy_bomb_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_ENERGY_SMART_BOMB_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_energy_bomb_end_status_func);

    let kiila_explode_bomb_start_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_explode_bomb_start_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_EXPLODE_SHOT_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_explode_bomb_start_coroutine_func);
    let kiila_explode_bomb_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_explode_bomb_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_EXPLODE_SHOT_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_explode_bomb_start_status_func);

    let kiila_explode_bomb_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_explode_bomb_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_EXPLODE_SHOT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_explode_bomb_end_coroutine_func);
    let kiila_explode_bomb_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_explode_bomb_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_EXPLODE_SHOT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_explode_bomb_end_status_func);

    let kiila_strike_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_strike_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_CRUSH_DOWN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),kiila_strike_end_coroutine_func);
    let kiila_strike_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(kiila_strike_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_CRUSH_DOWN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),kiila_strike_end_status_func);
}