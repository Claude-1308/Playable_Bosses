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

pub unsafe fn low_fireball_shot3_start_rate_fix(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if MotionModule::motion_kind(boss_boma) == hash40("low_fireball_shot3_start") {
        MotionModule::set_rate(module_accessor,2.0);
    }
}

pub unsafe fn lioleusboss_low_fireball_shot3_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("low_fireball_shot3_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_fireball3_air_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_low_fireball_shot3_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        let owner = BossModule::get_owner(module_accessor);
        WorkModule::set_int(owner,*SITUATION_KIND_AIR,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn glide_correct(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if MotionModule::motion_kind(boss_boma) == hash40("sky_slip_loop") {
        if AttackModule::is_attack(boss_boma,0,false) {
            AttackModule::set_power(boss_boma,0,15.0,false);
            AttackModule::set_power(boss_boma,1,15.0,false);
        }
        if ControlModule::get_stick_x(module_accessor) * PostureModule::lr(boss_boma) < 0.0 {
            StatusModule::change_status_request(boss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_GLIDE_END2,false);
        }
        else if ControlModule::get_stick_x(module_accessor).abs() < Common::min_stick
        || ControlModule::check_button_off(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
            StatusModule::change_status_request(boss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_GLIDE_END,false);
        }
    }
    if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_GLIDE_END {
        if AttackModule::is_attack(boss_boma,1,false) {
            AttackModule::set_power(boss_boma,1,10.0,false);
        }
        WorkModule::set_int(boss_boma,*SITUATION_KIND_GROUND,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
    }
    if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_GLIDE_END2 {
        MotionModule::set_rate(boss_boma,1.5);
        WorkModule::set_int(boss_boma,*SITUATION_KIND_AIR,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
    }
}

pub static mut LIOLEUSBOSS_SKY_SLIP_ENDING : usize = 0x4bcc28;

#[skyline::hook(replace = LIOLEUSBOSS_SKY_SLIP_ENDING, inline)]
pub unsafe fn lioleusboss_sky_slip_ending(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT as u32;
}

pub static mut LIOLEUSBOSS_SKY_SLIP_ENDING2 : usize = 0x4bd380;

#[skyline::hook(replace = LIOLEUSBOSS_SKY_SLIP_ENDING2, inline)]
pub unsafe fn lioleusboss_sky_slip_ending2(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR as u32;
}

pub unsafe fn landing_correct(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_CHANGE_MODE_GROUND {
        if MotionModule::frame(boss_boma) > 50.0 {
            MotionModule::set_rate(boss_boma,1.5);
        }
        WorkModule::set_int(boss_boma,*SITUATION_KIND_GROUND,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
    }
}

pub static mut LIOLEUSBOSS_LANDING_ENDING : usize = 0x4bdd54;

#[skyline::hook(replace = LIOLEUSBOSS_LANDING_ENDING, inline)]
pub unsafe fn lioleusboss_landing_ending(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR as u32;
}

pub unsafe fn install_air_specials(item: &mut L2CAgentBase) {
    let lioleusboss_low_fireball_shot3_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_low_fireball_shot3_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_AIR_END),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),lioleusboss_low_fireball_shot3_end_coroutine_func);
    let lioleusboss_low_fireball_shot3_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_low_fireball_shot3_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_AIR_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_low_fireball_shot3_end_status_func);
}