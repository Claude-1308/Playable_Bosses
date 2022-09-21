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

pub unsafe fn lioleusboss_air_howling_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("air_howling"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_howling_air"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_air_howling_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if AttackModule::is_attack(module_accessor,0,false) {
        AttackModule::set_power(module_accessor,0,10.0,false);
        AttackModule::set_power(module_accessor,1,10.0,false);
    }
    if MotionModule::is_end(module_accessor) {
        let owner = BossModule::get_owner(module_accessor);
        WorkModule::set_int(owner,*SITUATION_KIND_AIR,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_nail_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["lioleusboss_nail_coroutine"].get_ptr());
    original(item);
    if MotionModule::is_end(module_accessor) {
        WorkModule::set_int(owner,*SITUATION_KIND_AIR,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR,false);
        return L2CValue::I32(1)
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_nail_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["lioleusboss_nail_status"].get_ptr());
    original(item);
    if AttackModule::is_attack(module_accessor,0,false) {
        AttackModule::set_power(module_accessor,0,10.0,false);
        AttackModule::set_poison_param(module_accessor,0,600,40,2.0,false);
        AttackModule::set_power(module_accessor,1,10.0,false);
        AttackModule::set_poison_param(module_accessor,1,600,40,2.0,false);
    }
    if MotionModule::frame(module_accessor) < 67.0
    || MotionModule::frame(module_accessor) > 110.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    else {
        MotionModule::set_rate(module_accessor,1.0);
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_low_fireball_shot_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["lioleusboss_low_fireball_shot_coroutine"].get_ptr());
    original(item);
    if MotionModule::frame(module_accessor) < 84.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    else {
        MotionModule::set_rate(module_accessor,1.0);
    }
    if MotionModule::is_end(module_accessor) {
        WorkModule::set_int(owner,*SITUATION_KIND_AIR,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR,false);
        return L2CValue::I32(1)
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_aerials(item: &mut L2CAgentBase) {
    let lioleusboss_air_howling_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_air_howling_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_air_howling_coroutine_func);
    let lioleusboss_air_howling_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_air_howling_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_air_howling_status_func);

    let owner = BossModule::get_owner(&mut *item.module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    fighter.global_table["lioleusboss_low_fireball_shot_coroutine"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL_AIR),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE)));

    let lioleusboss_low_fireball_shot_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_low_fireball_shot_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_low_fireball_shot_coroutine_func);

    fighter.global_table["lioleusboss_nail_coroutine"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_NAIL_AIR),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE)));

    let lioleusboss_nail_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_nail_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_NAIL_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_nail_coroutine_func);

    fighter.global_table["lioleusboss_nail_status"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_NAIL_AIR),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS)));

    let lioleusboss_nail_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_nail_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_NAIL_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_nail_status_func);
}