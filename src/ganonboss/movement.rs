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
use crate::ganonboss::move_input::*;

pub unsafe fn ganon_walk_front_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("walk_front"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_walk_front"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ganon_walk_front_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let min_stick = Common::min_stick;
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let input_1 = fighter.global_table[CMD_CAT1].get_i32();
    let input_2 = fighter.global_table[CMD_CAT2].get_i32();
    ganonboss_move_inputs(owner,input_1,input_2);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN) {
        let attack = WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        StatusModule::change_status_request(module_accessor,attack,false);
    }
    if MotionModule::is_end(module_accessor) {
        if PostureModule::lr(module_accessor) * ControlModule::get_stick_x(owner) > 0.0
        && ControlModule::get_stick_x(owner).abs() > min_stick {
            MotionModule::set_frame(module_accessor,0.0,false);
        }
        else if PostureModule::lr(module_accessor) * ControlModule::get_stick_x(owner) < 0.0
        && ControlModule::get_stick_x(owner).abs() > min_stick {
            StatusModule::change_status_request(module_accessor,*ITEM_GANONBOSS_STATUS_KIND_WALK_BACK,false);
        }
        else {
            StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn ganon_walk_back_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("walk_back"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_walk_back"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ganon_walk_back_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let min_stick = Common::min_stick;
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let input_1 = fighter.global_table[CMD_CAT1].get_i32();
    let input_2 = fighter.global_table[CMD_CAT2].get_i32();
    ganonboss_move_inputs(owner,input_1,input_2);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN) {
        let attack = WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        StatusModule::change_status_request(module_accessor,attack,false);
    }
    if MotionModule::is_end(module_accessor) {
        if PostureModule::lr(module_accessor) * ControlModule::get_stick_x(owner) < 0.0
        && ControlModule::get_stick_x(owner).abs() > min_stick {
            MotionModule::set_frame(module_accessor,0.0,false);
        }
        else if PostureModule::lr(module_accessor) * ControlModule::get_stick_x(owner) < 0.0
        && ControlModule::get_stick_x(owner).abs() > min_stick {
            StatusModule::change_status_request(module_accessor,*ITEM_GANONBOSS_STATUS_KIND_WALK_FRONT,false);
        }
        else {
            StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn ganon_back_jump_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("back_jump"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_move_back_jump"),0.0);
    boss_private::sub1_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_move_back_jump_movement"),180.0);
    WorkModule::off_flag(module_accessor,*ITEM_INSTANCE_WORK_FLAG_TAKE_OFF);
    StatusModule::set_situation_kind(module_accessor,SituationKind(*SITUATION_KIND_AIR),false);
    GroundModule::set_correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    JostleModule::set_status(module_accessor,false);
    return L2CValue::I32(0)
}

pub unsafe fn ganon_back_jump_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,*ITEM_GANONBOSS_INSTANCE_WORK_FLAG_IS_TOUCH) {
        JostleModule::set_status(module_accessor,true);
        StatusModule::set_situation_kind(module_accessor,SituationKind(*SITUATION_KIND_GROUND),false);
        GroundModule::set_correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_move_back_jump_brake"));
        WorkModule::off_flag(module_accessor,*ITEM_GANONBOSS_INSTANCE_WORK_FLAG_IS_TOUCH);
    }
    if AttackModule::is_attack(module_accessor,1,false) {
        AttackModule::set_power(module_accessor,1,5.0,false);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ganon_turn_jump_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("turn_jump"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_move_turn_jump"),0.0);
    boss_private::sub1_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_move_turn_jump_movement"),0.0);
    WorkModule::off_flag(module_accessor,*ITEM_INSTANCE_WORK_FLAG_TAKE_OFF);
    StatusModule::set_situation_kind(module_accessor,SituationKind(*SITUATION_KIND_AIR),false);
    GroundModule::set_correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    JostleModule::set_status(module_accessor,false);
    return L2CValue::I32(0)
}

pub unsafe fn ganon_turn_jump_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,*ITEM_GANONBOSS_INSTANCE_WORK_FLAG_IS_TOUCH) {
        JostleModule::set_status(module_accessor,true);
        StatusModule::set_situation_kind(module_accessor,SituationKind(*SITUATION_KIND_GROUND),false);
        GroundModule::set_correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_move_turn_jump_brake"));
        WorkModule::off_flag(module_accessor,*ITEM_GANONBOSS_INSTANCE_WORK_FLAG_IS_TOUCH);
    }
    if AttackModule::is_attack(module_accessor,1,false) {
        AttackModule::set_power(module_accessor,1,5.0,false);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_movement(item: &mut L2CAgentBase) {
    let ganon_walk_front_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganon_walk_front_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_WALK_FRONT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganon_walk_front_coroutine_func);
    let ganon_walk_front_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganon_walk_front_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_WALK_FRONT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganon_walk_front_status_func);

    let ganon_walk_back_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganon_walk_back_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_WALK_BACK),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganon_walk_back_coroutine_func);
    let ganon_walk_back_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganon_walk_back_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_WALK_BACK),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganon_walk_back_status_func);

    let ganon_back_jump_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganon_back_jump_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_BACK_JUMP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganon_back_jump_coroutine_func);
    let ganon_back_jump_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganon_back_jump_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_BACK_JUMP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganon_back_jump_status_func);

    let ganon_turn_jump_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganon_turn_jump_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_TURN_JUMP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ganon_turn_jump_coroutine_func);
    let ganon_turn_jump_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ganon_turn_jump_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GANONBOSS_STATUS_KIND_TURN_JUMP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ganon_turn_jump_status_func);
}