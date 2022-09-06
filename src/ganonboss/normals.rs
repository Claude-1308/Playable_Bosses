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
use crate::ganonboss::*;

pub unsafe fn ganonboss_double_slash_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("double_slash"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_attack_double_slash"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_double_slash_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 74.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    if MotionModule::frame(module_accessor) > 65.0 {
        for i in 0..6 {
            if AttackModule::is_attack(module_accessor,i,false) {
                AttackModule::set_power(module_accessor,i,10.0,false);
            }
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_big_jump_fix(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if StatusModule::status_kind(boss_boma) == *ITEM_GANONBOSS_STATUS_KIND_ATTACK_BIG_JUMP {
        WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
    }
    if StatusModule::status_kind(boss_boma) == *ITEM_GANONBOSS_STATUS_KIND_ATTACK_BIG_JUMP_END {
        if MotionModule::frame(boss_boma) < 16.0 {
            let pos = Vector2f{x: ControlModule::get_stick_x(module_accessor), y: 0.0};
            PostureModule::add_pos_2d(boss_boma,&pos);
        }
        if MotionModule::frame(boss_boma) > 20.0 {
            MotionModule::set_rate(boss_boma,1.5);
        }
        for i in 0..4 {
            if AttackModule::is_attack(boss_boma,i,false) {
                AttackModule::set_power(boss_boma,i,20.0,false);
            }
        }
    }
}

pub unsafe fn ganonboss_slash_up_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("slash_up"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_attack_slash_up"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_slash_up_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 100.0 {
        let rate = 2.0;
        MotionModule::set_rate(module_accessor,rate);
    }
    if MotionModule::frame(module_accessor) > 84.0 {
        let power = 8.0;
        for i in 0..10 {
            if i == 8 {
                continue;
            }
            if AttackModule::is_attack(module_accessor,i,false) {
                AttackModule::set_power(module_accessor,i,power,false);
            }
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_back_slash_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("back_slash"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_attack_back_slash"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_back_slash_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 90.0 {
        let rate = 2.0;
        MotionModule::set_rate(module_accessor,rate);
    }
    if MotionModule::frame(module_accessor) > 69.0 {
        let power = 15.0;
        for i in 0..10 {
            if i == 8 {
                continue;
            }
            if AttackModule::is_attack(module_accessor,i,false) {
                AttackModule::set_power(module_accessor,i,power,false);
            }
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_jump_slash_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    JostleModule::set_status(module_accessor,false);
    WorkModule::off_flag(module_accessor,*ITEM_GANONBOSS_INSTANCE_WORK_FLAG_IS_TOUCH);
    MotionModule::change_motion(module_accessor,Hash40::new("jump_slash"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_attack_jump_slash"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ganonboss_jump_slash_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,*ITEM_GANONBOSS_INSTANCE_WORK_FLAG_IS_TOUCH) {
        JostleModule::set_status(module_accessor,true);
        StatusModule::set_situation_kind(module_accessor,SituationKind(*SITUATION_KIND_GROUND),false);
        GroundModule::set_correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_attack_jump_slash_landing_brake"));
        WorkModule::off_flag(module_accessor,*ITEM_GANONBOSS_INSTANCE_WORK_FLAG_IS_TOUCH);
    }
    if MotionModule::frame(module_accessor) > 55.0 {
        let rate = 2.0;
        MotionModule::set_rate(module_accessor,rate);
    }
    let power = 20.0;
    for i in 0..7 {
        if AttackModule::is_attack(module_accessor,i,false) {
            AttackModule::set_power(module_accessor,i,power,false);
        }
    }
    let owner = BossModule::get_owner(module_accessor);
    let pos = Vector2f{x: ControlModule::get_stick_x(owner), y: 0.0};
    PostureModule::add_pos_2d(module_accessor,&pos);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let ganonboss_double_slash_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_double_slash_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_DOUBLE_SLASH_EXEC),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),ganonboss_double_slash_coroutine_func);
    let ganonboss_double_slash_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_double_slash_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_DOUBLE_SLASH_EXEC),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganonboss_double_slash_status_func);

    let ganonboss_back_slash_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_back_slash_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_BACK_SLASH),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganonboss_back_slash_coroutine_func);
    let ganonboss_back_slash_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_back_slash_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_BACK_SLASH),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganonboss_back_slash_status_func);

    let ganonboss_slash_up_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_slash_up_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SLASH_UP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganonboss_slash_up_coroutine_func);
    let ganonboss_slash_up_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_slash_up_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SLASH_UP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganonboss_slash_up_status_func);
    
    let ganonboss_jump_slash_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_jump_slash_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_JUMP_SLASH_EXEC),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganonboss_jump_slash_coroutine_func);
    let ganonboss_jump_slash_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganonboss_jump_slash_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_ATTACK_JUMP_SLASH_EXEC),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganonboss_jump_slash_status_func);
}