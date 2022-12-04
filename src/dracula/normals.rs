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
use crate::dracula::*;

pub unsafe fn dracula_fill_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("attack_fill_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA),Hash40::new("energy_param_fill_end"),0.0);
    boss_private::clear_sub1_energy(lua_state);
    boss_private::unable_sub1_energy(lua_state);
    return L2CValue::I32(0)
}

pub unsafe fn dracula_fill_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn dracula_rush_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("attack_rush_end"),0.0,1.0,false,0.0,false,false);
    PostureModule::reverse_lr(module_accessor);
    if MotionModule::is_flip_resource(module_accessor) {
        if PostureModule::lr(module_accessor) > 0.0 {
            MotionModule::set_flip(module_accessor,false,true,false);
        }
        else {
            MotionModule::set_flip(module_accessor,true,true,false);
        }
    }
    PostureModule::update_rot_y_lr(module_accessor);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_DRACULA),Hash40::new("energy_param_rush_end"),0.0);
    boss_private::clear_sub1_energy(lua_state);
    boss_private::unable_sub1_energy(lua_state);
    WorkModule::on_flag(module_accessor,*ITEM_DRACULA_INSTANCE_WORK_FLAG_UNVISIBLE);
    HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_NORMAL),0);
    VisibilityModule::set_whole(module_accessor,true);
    return L2CValue::I32(0)
}

pub unsafe fn dracula_rush_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_DRACULA_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let dracula_fill_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_fill_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_FILL_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula_fill_end_coroutine_func);
    let dracula_fill_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_fill_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_FILL_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula_fill_end_status_func);

    let dracula_rush_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_rush_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_RUSH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),dracula_rush_end_coroutine_func);
    let dracula_rush_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(dracula_rush_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_DRACULA_STATUS_KIND_ATTACK_RUSH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),dracula_rush_end_status_func);
}