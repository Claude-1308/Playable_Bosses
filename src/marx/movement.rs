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
use crate::marx::move_input::*;

pub unsafe fn marx_move_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if ControlModule::get_stick_x(owner).abs() < Common::min_stick {
        if ControlModule::get_stick_y(owner) > Common::min_stick {
            MotionModule::change_motion(module_accessor,Hash40::new("move_up_start"),0.0,1.0,false,0.0,false,false);
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("move_down_start"),0.0,1.0,false,0.0,false,false);
        }
    }
    else {
        if ControlModule::get_stick_x(owner) * PostureModule::lr(module_accessor) > 0.0 {
            MotionModule::change_motion(module_accessor,Hash40::new("move_right_start"),0.0,1.0,false,0.0,false,false);
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("move_left_start"),0.0,1.0,false,0.0,false,false);
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn marx_move_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if MotionModule::is_end(module_accessor) {
        if MotionModule::motion_kind(module_accessor) == hash40("move_up_start") {
            MotionModule::change_motion(module_accessor,Hash40::new("move_up_loop"),0.0,1.0,false,0.0,false,false);
        }
        else if MotionModule::motion_kind(module_accessor) == hash40("move_down_start") {
            MotionModule::change_motion(module_accessor,Hash40::new("move_down_loop"),0.0,1.0,false,0.0,false,false);
        }
        else if MotionModule::motion_kind(module_accessor) == hash40("move_right_start") {
            MotionModule::change_motion(module_accessor,Hash40::new("move_right_loop"),0.0,1.0,false,0.0,false,false);
        }
        else if MotionModule::motion_kind(module_accessor) == hash40("move_left_start") {
            MotionModule::change_motion(module_accessor,Hash40::new("move_left_loop"),0.0,1.0,false,0.0,false,false);
        }
        else {
            StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
        }
    }
    if ControlModule::get_stick_x(owner).abs() < Common::min_stick
    && ControlModule::get_stick_y(owner).abs() < Common::min_stick {
        if MotionModule::motion_kind(module_accessor) == hash40("move_up_loop")
        || MotionModule::motion_kind(module_accessor) == hash40("move_down_loop")
        || MotionModule::motion_kind(module_accessor) == hash40("move_right_loop")
        || MotionModule::motion_kind(module_accessor) == hash40("move_left_loop") {
            if MotionModule::motion_kind(module_accessor) == hash40("move_up_loop") {
                MotionModule::change_motion(module_accessor,Hash40::new("move_up_end"),0.0,1.0,false,0.0,false,false);
            }
            else if MotionModule::motion_kind(module_accessor) == hash40("move_down_loop") {
                MotionModule::change_motion(module_accessor,Hash40::new("move_down_end"),0.0,1.0,false,0.0,false,false);
            }
            else if MotionModule::motion_kind(module_accessor) == hash40("move_right_loop") {
                MotionModule::change_motion(module_accessor,Hash40::new("move_right_end"),0.0,1.0,false,0.0,false,false);
            }
            else if MotionModule::motion_kind(module_accessor) == hash40("move_left_loop") {
                MotionModule::change_motion(module_accessor,Hash40::new("move_left_end"),0.0,1.0,false,0.0,false,false);
            }
        }
    }
    if ControlModule::get_stick_x(owner).abs() < Common::min_stick {
        if ControlModule::get_stick_y(owner) > Common::min_stick {
            MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,Hash40::new("move_up_loop"),1.0,-1.0,0.0);
            let pos = Vector2f{x: 0.0, y: ControlModule::get_stick_y(owner) * Marx::move_mul};
            PostureModule::add_pos_2d(module_accessor,&pos);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,Hash40::new("move_down_loop"),1.0,-1.0,0.0);
            let pos = Vector2f{x: 0.0, y: ControlModule::get_stick_y(owner) * Marx::move_mul};
            PostureModule::add_pos_2d(module_accessor,&pos);
        }
    }
    else {
        if ControlModule::get_stick_x(owner) * PostureModule::lr(module_accessor) > 0.0 {
            MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,Hash40::new("move_right_loop"),1.0,-1.0,0.0);
            let pos = Vector2f{x: ControlModule::get_stick_x(owner) * Marx::move_mul, y: 0.0};
            PostureModule::add_pos_2d(module_accessor,&pos);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,Hash40::new("move_left_loop"),1.0,-1.0,0.0);
            let pos = Vector2f{x: ControlModule::get_stick_x(owner) * Marx::move_mul, y: 0.0};
            PostureModule::add_pos_2d(module_accessor,&pos);
        }
    }
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let input_1 = fighter.global_table[CMD_CAT1].get_i32();
    let input_2 = fighter.global_table[CMD_CAT2].get_i32();
    marx_move_inputs(owner,input_1,input_2);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN) {
        let attack = WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        StatusModule::change_status_request(module_accessor,attack,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_movement(item: &mut L2CAgentBase) {
    let marx_move_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(marx_move_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_MOVE_STRAIGHT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),marx_move_coroutine_func);
    let marx_move_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(marx_move_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_MOVE_STRAIGHT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),marx_move_status_func);
}