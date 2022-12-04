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
use crate::marx::{move_input::*,normals::*};

pub unsafe fn entry_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    if FighterUtil::is_hp_mode(owner) {
        let hp = WorkModule::get_float(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP);
        set_hp(lua_state,hp);
    }
    else {
        set_hp(lua_state,Marx::health);
        WorkModule::set_float(owner,Marx::health,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_PREV);
    }
    set_visibility_whole_force(lua_state,true);
    let pos = Vector2f{x: PostureModule::pos_x(owner), y: PostureModule::pos_y(owner)};
    PostureModule::set_pos_2d(module_accessor,&pos);
    let rot = Vector3f{x: 0.0, y: -90.0, z: 0.0};
    PostureModule::set_rot(module_accessor,&rot,0);
    MotionModule::change_motion(module_accessor,Hash40::new("entry"),400.0,1.5,false,0.0,false,false);
    return L2CValue::I32(0)
}

pub unsafe fn entry_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_OFF),0);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn entry_end(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    boss_private::unable_energy_all(lua_state);
    let marx_four_cutter_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(marx_four_cutter_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_ATTACK_4_CUTTER),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),marx_four_cutter_coroutine_func);
    let marx_four_cutter_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(marx_four_cutter_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_ATTACK_4_CUTTER),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),marx_four_cutter_status_func);
    return L2CValue::I32(0)
}

pub unsafe fn wait_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    AttackModule::set_power_mul(module_accessor,1.0);
    AttackModule::set_power_mul_status(module_accessor,1.0);
    AttackModule::set_reaction_mul(module_accessor,1.0);
    let rng = sv_math::rand(hash40("marx"),2);
    if rng == 1 {
        MotionModule::change_motion(module_accessor,Hash40::new("wait_convulsion"),0.0,1.0,false,0.0,false,false);
        boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MARX),Hash40::new("energy_param_wait_convulsion"),0.0);
        boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_MARX),Hash40::new("energy_param_wait_convulsion_brake"));
        WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("wait"),0.0,1.0,false,0.0,false,false);
        boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MARX),Hash40::new("energy_param_wait"),0.0);
        boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_MARX),Hash40::new("energy_param_wait_brake"));
    }
    WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
    WorkModule::set_int(module_accessor,0,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
    WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    return L2CValue::I32(0)
}

pub unsafe fn wait_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    if MotionModule::is_end(module_accessor)
    && WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON) {
        MotionModule::change_motion(module_accessor,Hash40::new("wait"),0.0,1.0,false,0.0,false,false);
        boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MARX),Hash40::new("energy_param_wait"),0.0);
        boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_MARX),Hash40::new("energy_param_wait_brake"));
        WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON);
    }
    let input_1 = fighter.global_table[CMD_CAT1].get_i32();
    let input_2 = fighter.global_table[CMD_CAT2].get_i32();
    marx_move_inputs(owner,input_1,input_2);
    marx_movement(owner);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN) {
        let attack = WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        StatusModule::change_status_request(module_accessor,attack,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_entry_dead_wait(item: &mut L2CAgentBase) {
    let entry_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(entry_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_ATTACK_4_CUTTER),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),entry_coroutine_func);
    let entry_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(entry_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_ATTACK_4_CUTTER),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),entry_status_func);
    let entry_end_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(entry_end as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_ATTACK_4_CUTTER),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_END),entry_end_func);
    
    let wait_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(wait_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_WAIT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),wait_coroutine_func);
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_TRANS_PHASE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),wait_coroutine_func);
    let wait_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(wait_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_WAIT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),wait_status_func);
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_TRANS_PHASE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),wait_status_func);
}