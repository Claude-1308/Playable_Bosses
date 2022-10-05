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
use crate::ganonboss::move_input::*;

pub unsafe fn ganonboss_body_attack_hold_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let rate = 1.5;
    MotionModule::change_motion(module_accessor,Hash40::new("body_attack_hold"),0.0,rate,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_attack_body_attack_hold"),0.0);
    WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_body_attack_hold_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_BODY_ATTACK_START,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_spin_slash_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let rate = 3.0;
    MotionModule::change_motion(module_accessor,Hash40::new("spin_slash_start"),0.0,rate,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_attack_spin_slash_start"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_spin_slash_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SPIN_SLASH_LOOP,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_spin_slash_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if ControlModule::get_stick_x(owner) * PostureModule::lr(module_accessor) < 0.0 {
        MotionModule::change_motion(module_accessor,Hash40::new("spin_slash_end_reverse"),0.0,1.0,false,0.0,false,false);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("spin_slash_end"),0.0,1.0,false,0.0,false,false);
    }
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_attack_spin_slash_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_spin_slash_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let power = 15.0;
    for i in 0..10 {
        if AttackModule::is_attack(module_accessor,i,false) {
            AttackModule::set_power(module_accessor,i,power,false);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_body_attack_loop_fix(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if StatusModule::status_kind(boss_boma) == *ITEM_GANONBOSS_STATUS_KIND_ATTACK_BODY_ATTACK_LOOP {
        if PostureModule::pos_x(boss_boma).abs() > dead_range(lua_state).x.abs()
        && fighter.global_table["fs_ganon_ending"].get_bool() == false {
            StatusModule::change_status_request(boss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_BODY_ATTACK_END,false);
        }
        let power = 25.0;
        for i in 0..2 {
            if AttackModule::is_attack(boss_boma,i,false) {
                AttackModule::set_power(boss_boma,i,power,false);
            }
        }
    }
    if StatusModule::status_kind(boss_boma) == *ITEM_GANONBOSS_STATUS_KIND_ATTACK_BODY_ATTACK_END {
        if MotionModule::frame(boss_boma) >= 45.0 {
            let rate = 2.0;
            MotionModule::set_rate(boss_boma,rate);
        }
    }
}

pub unsafe fn ganonboss_thunder_slash_fix(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if MotionModule::frame(boss_boma) >= 112.0
    && StatusModule::status_kind(boss_boma) == *ITEM_GANONBOSS_STATUS_KIND_ATTACK_THUNDER_SLASH_EXEC {
        StatusModule::change_status_request(boss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_THUNDER_SLASH_RETURN,false);
    }
    if MotionModule::frame(boss_boma) >= 50.0
    && StatusModule::status_kind(boss_boma) == *ITEM_GANONBOSS_STATUS_KIND_ATTACK_THUNDER_SLASH_RETURN {
        let rate = 2.0;
        MotionModule::set_rate(module_accessor,rate);
    }
}

pub unsafe fn install_specials(item: &mut L2CAgentBase) {
    let ganonboss_body_attack_hold_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_body_attack_hold_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_BODY_ATTACK_HOLD),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganonboss_body_attack_hold_coroutine_func);
    let ganonboss_body_attack_hold_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_body_attack_hold_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_BODY_ATTACK_HOLD),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganonboss_body_attack_hold_status_func);
    
    let ganonboss_spin_slash_start_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_spin_slash_start_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SPIN_SLASH_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganonboss_spin_slash_start_coroutine_func);
    let ganonboss_spin_slash_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_spin_slash_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SPIN_SLASH_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganonboss_spin_slash_start_status_func);

    let ganonboss_spin_slash_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_spin_slash_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SPIN_SLASH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganonboss_spin_slash_end_coroutine_func);
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SPIN_SLASH_END_REVERSE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganonboss_spin_slash_end_coroutine_func);
    let ganonboss_spin_slash_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_spin_slash_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SPIN_SLASH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganonboss_spin_slash_end_status_func);
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SPIN_SLASH_END_REVERSE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganonboss_spin_slash_end_status_func);
}