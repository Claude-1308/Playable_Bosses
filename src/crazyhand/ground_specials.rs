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

pub static mut CH_NIGIRU_LOOP_PRE : usize = 0x366bc0;

#[skyline::hook(replace=CH_NIGIRU_LOOP_PRE)]
pub unsafe fn ch_nigiru_loop_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        MotionModule::change_motion(module_accessor,Hash40::new("nigiru_loop"),0.0,1.0,false,0.0,false,false);
        return L2CValue::I32(0)
    }
    else {
        original!()(item)
    }
}

pub static mut CH_NIGIRU_LOOP_STATUS : usize = 0x366ca0;

#[skyline::hook(replace=CH_NIGIRU_LOOP_STATUS)]
pub unsafe fn ch_nigiru_loop_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        let owner = BossModule::get_owner(module_accessor);
        let min_stick = Common::min_stick;
        if ControlModule::check_button_trigger(owner,*CONTROL_PAD_BUTTON_SPECIAL)
        && ControlModule::get_stick_x(owner).abs() < min_stick
        && MotionModule::motion_kind(module_accessor) == hash40("nigiru_loop") {
            MotionModule::change_motion(module_accessor,Hash40::new("nigiru"),60.0,1.0,false,0.0,false,false);
        }
        if MotionModule::motion_kind(module_accessor) == hash40("nigiru") {
            let power_mul = 1.5;
            AttackModule::set_power_mul(module_accessor,power_mul);
            if MotionModule::frame(module_accessor) >= 70.0 {
                MotionModule::set_rate(module_accessor,2.0);
            }
            if MotionModule::frame(module_accessor) >= 80.0 {
                MotionModule::change_motion(module_accessor,Hash40::new("nigiru_loop"),0.0,1.0,false,0.0,false,false);
            }
        }
        if MotionModule::motion_kind(module_accessor) == hash40("nigiru_loop") {
            AttackModule::set_power_mul(module_accessor,1.0);
            if ControlModule::get_stick_x(owner).abs() > min_stick {
                let reaction_mul = 1.3;
                AttackModule::set_reaction_mul(module_accessor,reaction_mul);
                StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_NIGIRU_THROW_END_2,false);
            }
            if ControlModule::get_stick_y(owner).abs() > min_stick {
                StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_NIGIRU_THROW_END_3,false);
            }
        }
        return L2CValue::I32(0)
    }
    else {
        original!()(item)
    }
}

pub unsafe fn ch_kumo_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("kumo"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_CRAZYHAND),Hash40::new("energy_param_kumo"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ch_kumo_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) >= 110.0 {
        if MotionModule::frame(module_accessor) >= 210.0 {
            MotionModule::set_rate(module_accessor,2.0);
        }
        else {
            MotionModule::set_rate(module_accessor,1.0);
        }
    }
    for i in 0..2 {
        if AttackModule::is_attack(module_accessor,i,false) {
            AttackModule::set_power(module_accessor,i,12.0,false);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_notautsu_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("notautsu"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_CRAZYHAND),Hash40::new("energy_param_notautsu"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ch_notautsu_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) < 276.0 {
        for i in 0..8 {
            if AttackModule::is_attack(module_accessor,i,false) {
                AttackModule::set_power(module_accessor,i,1.5,false);
            }
        }
    }
    if MotionModule::frame(module_accessor) > 310.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    if WorkModule::is_flag(module_accessor,*ITEM_CRAZYHAND_INSTANCE_WORK_FLAG_NOTAUTSU_JOSTLE_OFF) {
        JostleModule::set_status(module_accessor,false);
        WorkModule::off_flag(module_accessor,*ITEM_CRAZYHAND_INSTANCE_WORK_FLAG_NOTAUTSU_JOSTLE_OFF);
    }
    if WorkModule::is_flag(module_accessor,*ITEM_CRAZYHAND_INSTANCE_WORK_FLAG_NOTAUTSU_JOSTLE_ON) {
        JostleModule::set_status(module_accessor,true);
        WorkModule::off_flag(module_accessor,*ITEM_CRAZYHAND_INSTANCE_WORK_FLAG_NOTAUTSU_JOSTLE_ON);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_dig_loop_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    MotionModule::change_motion(module_accessor,Hash40::new("dig_loop"),0.0,1.0,false,0.0,false,false);
    let hold_timer = 300;
    WorkModule::set_int(owner,hold_timer,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    return L2CValue::I32(0)
}

pub unsafe fn ch_dig_loop_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER) <= 0
    || ControlModule::check_button_off(owner,*CONTROL_PAD_BUTTON_SPECIAL) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_DIG_END,false);
    }
    WorkModule::dec_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    return L2CValue::I32(0)
}

pub unsafe fn ch_dig_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("dig_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_CRAZYHAND),Hash40::new("energy_param_dig_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ch_dig_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,*ITEM_CRAZYHAND_INSTANCE_WORK_FLAG_SHOCK_WAVE_CREATE) {
        boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_CRAZYHANDSHOCKWAVE),PostureModule::pos_x(module_accessor),0.0,0.0,PostureModule::lr(module_accessor));
        WorkModule::off_flag(module_accessor,*ITEM_CRAZYHAND_INSTANCE_WORK_FLAG_SHOCK_WAVE_CREATE);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_ground_specials(item: &mut L2CAgentBase) {
    let ch_kumo_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_kumo_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_KUMO),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),ch_kumo_pre_func);
    let ch_kumo_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_kumo_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_KUMO),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_kumo_status_func);

    let ch_notautsu_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_notautsu_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_NOTAUTSU),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),ch_notautsu_pre_func);
    let ch_notautsu_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_notautsu_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_NOTAUTSU),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_notautsu_status_func);

    let ch_dig_loop_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_dig_loop_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_DIG_LOOP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ch_dig_loop_coroutine_func);
    let ch_dig_loop_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_dig_loop_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_DIG_LOOP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_dig_loop_status_func);

    let ch_dig_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_dig_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_DIG_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ch_dig_end_coroutine_func);
    let ch_dig_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_dig_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_DIG_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_dig_end_status_func);
}