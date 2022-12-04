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

pub unsafe fn darz_cross_bomb_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    item__set_camera_range_from_param(lua_state,ItemKind(*ITEM_KIND_DARZ),Hash40::new("cross_bomb_camera_subject"));
    MotionModule::change_motion(module_accessor,Hash40::new("cross_bomb"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DARZ),Hash40::new("energy_param_cross_bomb"),0.0);
    WorkModule::off_flag(module_accessor,*ITEM_DARZ_INSTANCE_WORK_FLAG_SPAWN_SHOT);
    return L2CValue::I32(0)
}

pub unsafe fn darz_cross_bomb_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let mut rnd_vec = vec![0,1,2,3,4,5,6,7];
    if WorkModule::is_flag(module_accessor,*ITEM_DARZ_INSTANCE_WORK_FLAG_SPAWN_SHOT) {
        for _ in 0..6 {
            let size = rnd_vec.len() as i32;
            let rng = sv_math::rand(hash40("item"),size) as usize;
            let x = item__self_struct_array_param_float(ItemKind(*ITEM_KIND_DARZ),Hash40::new("cross_bomb_pos_param"),rnd_vec[rng],Hash40::new("x"));
            let y = item__self_struct_array_param_float(ItemKind(*ITEM_KIND_DARZ),Hash40::new("cross_bomb_pos_param"),rnd_vec[rng],Hash40::new("y"));
            let bomb = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_DARZCROSSBOMB),x,y,0.0,1.0) as *mut BattleObjectModuleAccessor;
            if bomb.is_null() == false {
                WorkModule::set_int(bomb,40,*ITEM_DARZBOMB_INSTANCE_WORK_INT_DELAY);
            }
            if let Some(index) = rnd_vec.iter().position(|value| *value == rnd_vec[rng] as u64) {
                rnd_vec.remove(index);
            }
        }
        WorkModule::off_flag(module_accessor,*ITEM_DARZ_INSTANCE_WORK_FLAG_SPAWN_SHOT);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_DARZ_STATUS_KIND_MANAGER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub static mut DARZ_SPACE_RUSH_PARAM_FIX: usize = 0x3ae93c;

#[skyline::hook(replace = DARZ_SPACE_RUSH_PARAM_FIX, inline)]
pub unsafe fn darz_space_rush_param_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let value: u64 = hash40("space_rush_pos_param3");
    asm!("mov x0, {}", in(reg) value);
}

pub static mut DARZ_SPACE_RUSH_INTERVAL_FIX: usize = 0x3aeab0;

#[skyline::hook(replace = DARZ_SPACE_RUSH_INTERVAL_FIX, inline)]
pub unsafe fn darz_space_rush_interval_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let value: f32 = 50.0;
    asm!("fmov s0, w8", in("w8") value);
}

pub static mut DARZ_SPACE_RUSH_NUM_FIX: usize = 0x3aebf8;

#[skyline::hook(replace = DARZ_SPACE_RUSH_NUM_FIX, inline)]
pub unsafe fn darz_space_rush_num_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 5u32;
}

pub unsafe fn darz_space_rush_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("space_rush_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DARZ),Hash40::new("energy_param_space_rush_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn darz_space_rush_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_DARZ_STATUS_KIND_MANAGER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub static mut DARZ_PILLAR_FRAME_FIX: usize = 0x3b36e8;

#[skyline::hook(replace = DARZ_PILLAR_FRAME_FIX, inline)]
pub unsafe fn darz_pillar_frame_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[3].x.as_mut() = 10u64;
}

pub unsafe fn darz_dark_pillar_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("dark_pillar_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DARZ),Hash40::new("energy_param_dark_pillar_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn darz_dark_pillar_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_DARZ_STATUS_KIND_MANAGER_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub static mut DARZ_CENTIPEDE_LENGTH_FIX: usize = 0x3b0250;

#[skyline::hook(replace = DARZ_CENTIPEDE_LENGTH_FIX, inline)]
pub unsafe fn darz_centipede_length_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 9u32;
}

pub static mut DARZ_CENTIPEDE_FIX: usize = 0x3b0da4;

#[skyline::hook(replace = DARZ_CENTIPEDE_FIX, inline)]
pub unsafe fn darz_centipede_fix(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let owner = BossModule::get_owner(&mut *agent_base.module_accessor);
    WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON);
    *ctx.registers[0].w.as_mut() = *ITEM_DARZ_STATUS_KIND_TELEPORT as u32;
}

pub unsafe fn darz_centipede_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("teleport_start"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DARZ),Hash40::new("energy_param_teleport_start"),0.0);
    JostleModule::set_status(module_accessor,false);
    HitModule::set_status_all(module_accessor,HitStatus(*HIT_STATUS_XLU),0);
    return L2CValue::I32(0)
}

pub unsafe fn darz_centipede_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        HitModule::set_status_all(module_accessor,HitStatus(*HIT_STATUS_NORMAL),0);
        LinkModule::send_event_parents(module_accessor,*ITEM_LINK_NO_CREATEOWNER,Hash40::new_raw(0x14f1921eacu64));
        StatusModule::change_status_request(module_accessor,*ITEM_DARZ_STATUS_KIND_CENTIPEDE_LOOP,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let darz_cross_bomb_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_cross_bomb_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_CROSS_BOMB),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),darz_cross_bomb_coroutine_func);
    let darz_cross_bomb_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_cross_bomb_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_CROSS_BOMB),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),darz_cross_bomb_status_func);

    let darz_space_rush_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_space_rush_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_SPACE_RUSH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),darz_space_rush_end_coroutine_func);
    let darz_space_rush_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_space_rush_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_SPACE_RUSH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),darz_space_rush_end_status_func);

    let darz_dark_pillar_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_dark_pillar_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_DARK_PILLAR_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),darz_dark_pillar_end_coroutine_func);
    let darz_dark_pillar_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_dark_pillar_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_DARK_PILLAR_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),darz_dark_pillar_end_status_func);

    let darz_centipede_start_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_centipede_start_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_CENTIPEDE_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),darz_centipede_start_coroutine_func);
    let darz_centipede_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(darz_centipede_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DARZ_STATUS_KIND_CENTIPEDE_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),darz_centipede_start_status_func);
}