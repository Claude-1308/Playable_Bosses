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

pub unsafe fn teleport_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON) {
        let pos = Vector2f{x: 0.0, y: 45.0};
        PostureModule::set_pos_2d(module_accessor,&pos);
        MotionModule::change_motion(module_accessor,Hash40::new("teleport_end"),0.0,1.0,false,0.0,false,false);
        VisibilityModule::set_whole(module_accessor,false);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("teleport_start"),0.0,1.0,false,0.0,false,false);
        HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_OFF),0);
        JostleModule::set_status(module_accessor,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn teleport_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON) {
        if MotionModule::is_end(module_accessor) {
            WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON);
            HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_NORMAL),0);
            JostleModule::set_status(module_accessor,true);
            VisibilityModule::set_whole(module_accessor,true);
            StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_MANAGER_WAIT,false);
        }
    }
    else {
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
            StatusModule::change_status_request(module_accessor,*ITEM_KIILA_STATUS_KIND_MANAGER_WAIT,false);
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_teleport(item: &mut L2CAgentBase) {
    let teleport_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(teleport_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_TELEPORT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),teleport_coroutine_func);
    let teleport_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(teleport_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_KIILA_STATUS_KIND_TELEPORT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),teleport_status_func);
}
