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

pub unsafe fn teleport_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("teleport_start"),0.0,1.0,false,0.0,false,false);
    HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_OFF),0);
    JostleModule::set_status(module_accessor,false);
    return L2CValue::I32(0)
}

pub unsafe fn teleport_coroutine(_item: &mut L2CAgentBase) -> L2CValue {
    return L2CValue::I32(0)
}

pub unsafe fn teleport_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if MotionModule::is_end(module_accessor) && MotionModule::motion_kind(module_accessor) == hash40("teleport_start") {
        let dir_mul = 100.0;
        let pos = Vector2f{x: ControlModule::get_stick_x(owner) * dir_mul, y: ControlModule::get_stick_y(owner) * dir_mul};
        PostureModule::add_pos_2d(module_accessor,&pos);
        MotionModule::change_motion(module_accessor,Hash40::new("teleport_end"),0.0,1.0,false,0.0,false,false);
        VisibilityModule::set_whole(module_accessor,true);
    }
    if MotionModule::is_end(module_accessor) && MotionModule::motion_kind(module_accessor) == hash40("teleport_end") {
        HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_NORMAL),0);
        JostleModule::set_status(module_accessor,true);
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn turn_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if PostureModule::lr(module_accessor) < 0.0 {
        MotionModule::change_motion(module_accessor,Hash40::new("turn"),0.0,2.0,false,0.0,false,false);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("turn_reverse"),0.0,2.0,false,0.0,false,false);
    }
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_turn"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn turn_coroutine(_item: &mut L2CAgentBase) -> L2CValue {
    return L2CValue::I32(0)
}

pub unsafe fn turn_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        PostureModule::update_rot_y_lr(module_accessor);
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_teleport_turn(item: &mut L2CAgentBase) {
    let teleport_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(teleport_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_WAIT_TELEPORT),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),teleport_pre_func);
    let teleport_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(teleport_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_WAIT_TELEPORT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),teleport_coroutine_func);
    let teleport_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(teleport_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_WAIT_TELEPORT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),teleport_status_func);
    
    let turn_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(turn_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_TURN),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),turn_pre_func);
    let turn_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(turn_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_TURN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),turn_coroutine_func);
    let turn_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(turn_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_TURN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),turn_status_func);
}