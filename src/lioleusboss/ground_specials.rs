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
use crate::common::{modules::*,params::*,hooks::*};

pub unsafe fn fireball_shot3_start_rate_fix(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if MotionModule::motion_kind(boss_boma) == hash40("fireball_shot3_start") {
        MotionModule::set_rate(module_accessor,2.0);
    }
}

pub unsafe fn assault_loop_fix(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if MotionModule::motion_kind(boss_boma) == hash40("assault_loop")
    || MotionModule::motion_kind(boss_boma) == hash40("assault_loop_reverse") {
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_power(module_accessor,0,25.0,false);
            AttackModule::set_power(module_accessor,1,25.0,false);
            AttackModule::set_power(module_accessor,2,25.0,false);
            AttackModule::set_power(module_accessor,3,25.0,false);
            AttackModule::set_power(module_accessor,4,25.0,false);
            AttackModule::set_power(module_accessor,5,25.0,false);
        }
        let pos = Vector2f{x: ControlModule::get_stick_x(module_accessor), y: 0.0};
        PostureModule::add_pos_2d(boss_boma,&pos);
    }
}

pub unsafe fn lioleusboss_fireball_shot3_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("fireball_shot3_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_fireball3_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_fireball_shot3_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        let owner = BossModule::get_owner(module_accessor);
        WorkModule::set_int(owner,*SITUATION_KIND_GROUND,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_tackle_loop_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if AttackModule::is_attack(module_accessor,0,false) {
        AttackModule::set_power(module_accessor,0,15.0,false);
        AttackModule::set_power(module_accessor,1,15.0,false);
        AttackModule::set_power(module_accessor,2,15.0,false);
        AttackModule::set_power(module_accessor,3,15.0,false);
    }
    if PostureModule::lr(module_accessor) * ControlModule::get_stick_x(owner) < 0.0 {
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_TURN,false);
    }
    else if ControlModule::check_button_trigger(owner,*CONTROL_PAD_BUTTON_JUMP) {
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_JUMP,false);
    }
    else if ControlModule::get_stick_x(owner).abs() < Common::min_stick
    && ControlModule::check_button_off(owner,*CONTROL_PAD_BUTTON_SPECIAL) {
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_END,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_tackle_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("body_attack_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_tackle_end"),0.0);
    boss_private::sub1_energy_from_param_inherit(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_tackle_end_brake"));
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_tackle_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 45.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    if AttackModule::is_attack(module_accessor,0,false) {
        AttackModule::set_power(module_accessor,0,10.0,false);
        AttackModule::set_power(module_accessor,1,10.0,false);
        AttackModule::set_power(module_accessor,2,10.0,false);
        AttackModule::set_power(module_accessor,3,10.0,false);
    }
    if MotionModule::is_end(module_accessor) {
        let owner = BossModule::get_owner(module_accessor);
        WorkModule::set_int(owner,*SITUATION_KIND_GROUND,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_tackle_turn_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("body_attack_turn"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_tackle_turn"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_tackle_turn_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        let owner = BossModule::get_owner(module_accessor);
        WorkModule::set_int(owner,*SITUATION_KIND_GROUND,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_tackle_jump_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["lioleusboss_tackle_jump_coroutine"].get_ptr());
    original(item);
    if AttackModule::is_attack(module_accessor,0,false) {
        AttackModule::set_power(module_accessor,0,5.0,false);
        AttackModule::set_poison_param(module_accessor,0,600,40,2.0,false);
        AttackModule::set_power(module_accessor,1,5.0,false);
        AttackModule::set_poison_param(module_accessor,1,600,40,2.0,false);
    }
    if MotionModule::is_end(module_accessor) {
        WorkModule::set_int(owner,*SITUATION_KIND_AIR,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR,false);
        return L2CValue::I32(1)
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_flight_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["lioleusboss_flight_coroutine"].get_ptr());
    original(item);
    if MotionModule::is_end(module_accessor) {
        WorkModule::set_int(owner,*SITUATION_KIND_AIR,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR,false);
        return L2CValue::I32(1)
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_ground_specials(item: &mut L2CAgentBase) {
    let lioleusboss_fireball_shot3_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_fireball_shot3_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_fireball_shot3_end_coroutine_func);
    let lioleusboss_fireball_shot3_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_fireball_shot3_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_fireball_shot3_end_status_func);

    let lioleusboss_tackle_loop_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_loop_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_tackle_loop_status_func);

    let lioleusboss_tackle_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_tackle_end_coroutine_func);
    let lioleusboss_tackle_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_tackle_end_status_func);

    let lioleusboss_tackle_turn_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_turn_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_TURN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_tackle_turn_coroutine_func);
    let lioleusboss_tackle_turn_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_turn_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_TURN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_tackle_turn_status_func);
    
    let owner = BossModule::get_owner(&mut *item.module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    fighter.global_table["lioleusboss_flight_coroutine"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_CHANGE_MODE_AIR),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE)));

    let lioleusboss_flight_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_flight_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_CHANGE_MODE_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_flight_coroutine_func);

    fighter.global_table["lioleusboss_tackle_jump_coroutine"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_JUMP),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE)));

    let lioleusboss_tackle_jump_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_tackle_jump_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_JUMP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_tackle_jump_coroutine_func);
}