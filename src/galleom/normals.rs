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

pub unsafe fn galleom_double_arm_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("double_arm"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_double_arm"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_double_arm_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 63.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    if AttackModule::is_attack(module_accessor,0,false) {
        for i in 0..6 {
            AttackModule::set_power(module_accessor,i,14.0,false);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_uppercut_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("uppercut"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_double_arm"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_uppercut_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 67.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    for i in 0..3 {
        if AttackModule::is_attack(module_accessor,i,false) {
            AttackModule::set_power(module_accessor,i,17.0,false);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_hammer_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("hammer_knuckle"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_hammer_knuckle"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_hammer_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,*ITEM_GALLEOM_INSTANCE_WORK_FLAG_SHOCKWAVE) {
        let mut pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        let global_pos = ModelModule::joint_global_position(module_accessor,Hash40::new("havel"),&mut pos,true);
        let shockwave = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_GALLEOM),global_pos.x,global_pos.y,global_pos.z,PostureModule::lr(module_accessor)) as *mut BattleObjectModuleAccessor;
        WorkModule::off_flag(module_accessor,*ITEM_GALLEOM_INSTANCE_WORK_FLAG_SHOCKWAVE);
    }
    if MotionModule::frame(module_accessor) > 100.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    for i in 0..5 {
        if AttackModule::is_attack(module_accessor,i,false) {
            AttackModule::set_power(module_accessor,i,17.0,false);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_foot_crush_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("foot_crush"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_foot_crush"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_foot_crush_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 55.0 {
        MotionModule::set_rate(module_accessor,1.5);
    }
    for i in 0..7 {
        if AttackModule::is_attack(module_accessor,i,false) {
            AttackModule::set_power(module_accessor,i,15.0,false);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let galleom_double_arm_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_double_arm_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_DOUBLE_ARM_PRESS),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_double_arm_coroutine_func);
    let galleom_double_arm_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_double_arm_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_DOUBLE_ARM_PRESS),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_double_arm_status_func);

    let galleom_uppercut_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_uppercut_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_UPPERCUT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_uppercut_coroutine_func);
    let galleom_uppercut_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_uppercut_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_UPPERCUT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_uppercut_status_func);

    let galleom_hammer_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_hammer_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_HAMMER_KNUCKLE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_hammer_coroutine_func);
    let galleom_hammer_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_hammer_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_HAMMER_KNUCKLE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_hammer_status_func);

    let galleom_foot_crush_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_foot_crush_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_STEP_CRUSH),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_foot_crush_coroutine_func);
    let galleom_foot_crush_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_foot_crush_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_STEP_CRUSH),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_foot_crush_status_func);
}