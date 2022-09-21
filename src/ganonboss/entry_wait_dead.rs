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
use crate::ganonboss::move_input::*;

pub unsafe fn entry_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if FighterUtil::is_hp_mode(owner) {
        let hp = WorkModule::get_float(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP);
        set_hp(lua_state,hp);
    }
    else {
        set_hp(lua_state,Ganonboss::health);
        WorkModule::set_float(owner,Ganonboss::health,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_PREV);
    }
    set_visibility_whole_force(lua_state,true);
    let pos = Vector2f{x: PostureModule::pos_x(owner), y: PostureModule::pos_y(owner)};
    PostureModule::set_pos_2d(module_accessor,&pos);
    if PostureModule::lr(owner) >= 0.0 {
        PostureModule::set_lr(module_accessor,1.0);
    }
    else {
        PostureModule::set_lr(module_accessor,-1.0);
    }
    PostureModule::update_rot_y_lr(module_accessor);
    MotionModule::change_motion(module_accessor,Hash40::new("entry"),0.0,1.5,false,0.0,false,false);
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

pub unsafe fn wait_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_PLAY_ENTRY) {
        entry_pre(item)
    }
    else {
        AttackModule::set_power_mul(module_accessor,1.0);
        AttackModule::set_reaction_mul(module_accessor,1.0);
        MotionModule::change_motion(module_accessor,Hash40::new("wait"),0.0,1.0,false,0.0,false,false);
        boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GANONBOSS),Hash40::new("energy_param_wait"),0.0);
        WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
        WorkModule::set_int(module_accessor,0,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
        return L2CValue::I32(0)
    }
}

pub unsafe fn wait_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_PLAY_ENTRY) {
        WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_PLAY_ENTRY);
        entry_status(item)
    }
    else {
        let min_stick = Common::min_stick;
        if ControlModule::get_stick_x(owner).abs() > min_stick
        || ControlModule::get_stick_y(owner).abs() > min_stick {
            ganonboss_movement(owner);
        }
        let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
        let input_1 = fighter.global_table[CMD_CAT1].get_i32();
        let input_2 = fighter.global_table[CMD_CAT2].get_i32();
        ganonboss_move_inputs(owner,input_1,input_2);
        if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN) {
            let attack = WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
            StatusModule::change_status_request(module_accessor,attack,false);
        }
        return L2CValue::I32(0)
    }
}

pub unsafe fn install_entry_dead_wait(item: &mut L2CAgentBase) {
    let wait_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(wait_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_WAIT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),wait_coroutine_func);
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_TRANS_PHASE),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),wait_coroutine_func);
    let wait_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(wait_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_WAIT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),wait_status_func);
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_TRANS_PHASE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),wait_status_func);
}