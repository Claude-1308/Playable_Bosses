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
use crate::lioleusboss::move_input::*;

pub unsafe fn entry_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let hp = WorkModule::get_float(module_accessor,*ITEM_INSTANCE_WORK_FLOAT_HP);
    if hp > 150.0 {
        item_add_damage(lua_state,hp-150.0);
    }
    set_visibility_whole_force(lua_state,true);
    let owner = BossModule::get_owner(module_accessor);
    let pos = Vector2f{x: PostureModule::pos_x(owner), y: PostureModule::pos_y(owner)};
    PostureModule::set_pos_2d(module_accessor,&pos);
    if pos.x >= 0.0 {
        MotionModule::change_motion(module_accessor,Hash40::new("entry_r"),0.0,1.0,false,0.0,false,false);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("entry_l"),0.0,1.0,false,0.0,false,false);
    }
    EFFECT(item, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 15, 0, -79, 0, -9, 0, 2, 0, 0, 0, 0, 0, 0, true);
    LAST_EFFECT_SET_RATE(item,0.5);
    LAST_EFFECT_SET_COLOR(item, 0.587, 0.538, 0.494);
    EFFECT(item, Hash40::new("sys_gliding_smoke"), Hash40::new("top"), -14, 0, -69, 0, -9, 0, 2, 0, 0, 0, 0, 0, 0, true);
    LAST_EFFECT_SET_RATE(item,0.5);
    LAST_EFFECT_SET_COLOR(item, 0.587, 0.538, 0.494);
    return L2CValue::I32(0)
}

pub unsafe fn entry_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_OFF),0);
    if MotionModule::frame(module_accessor) == 74.0 {
        PLAY_SE(item,Hash40::new("se_boss_lioleusboss_entryl"));
        EFFECT_FLW_POS(item, Hash40::new_raw(0x10fb159cd4u64), Hash40::new("jaw"), 0, 0, 0, 0, 90, 0, 0.8, true);
        QUAKE(item,*CAMERA_QUAKE_KIND_M);
    }
    if MotionModule::frame(module_accessor) == 90.0 {
        sv_animcmd::QUAKE_STOP(lua_state);
    }
    if MotionModule::is_end(module_accessor) {
        let owner = BossModule::get_owner(module_accessor);
        if PostureModule::pos_x(owner) >= 0.0 {
            PostureModule::set_lr(module_accessor,-1.0);
        }
        else {
            PostureModule::set_lr(module_accessor,1.0);
        }
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn entry_end(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    boss_private::unable_energy_all(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let dmg_mul = WorkModule::get_float(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_MUL);
    WorkModule::set_float(owner,dmg_mul * Lioleusboss::extra_dmg_mul,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_MUL);
    DamageModule::set_damage_mul(module_accessor,dmg_mul * Lioleusboss::extra_dmg_mul);
    return L2CValue::I32(0)
}

pub unsafe fn wait_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let hp = WorkModule::get_float(module_accessor,*ITEM_INSTANCE_WORK_FLOAT_HP);
    if hp > 150.0 {
        item_add_damage(lua_state,hp-150.0);
    }
    if smashball::is_training_mode() {
        set_visibility_whole_force(lua_state,true);
    }
    AttackModule::set_power_mul(module_accessor,1.0);
    AttackModule::set_reaction_mul(module_accessor,1.0);
    MotionModule::change_motion(module_accessor,Hash40::new("wait"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_wait"),0.0);
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
    let input_1 = fighter.global_table[CMD_CAT1].get_i32();
    let input_2 = fighter.global_table[CMD_CAT2].get_i32();
    lioleusboss_move_inputs_ground(owner,input_1,input_2);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN) {
        let attack = WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        StatusModule::change_status_request(module_accessor,attack,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn wait_air_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let hp = WorkModule::get_float(module_accessor,*ITEM_INSTANCE_WORK_FLOAT_HP);
    if hp > 150.0 {
        item_add_damage(lua_state,hp-150.0);
    }
    if smashball::is_training_mode() {
        set_visibility_whole_force(lua_state,true);
    }
    AttackModule::set_power_mul(module_accessor,1.0);
    AttackModule::set_reaction_mul(module_accessor,1.0);
    MotionModule::change_motion(module_accessor,Hash40::new("hovering"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_wait_air"),0.0);
    WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
    WorkModule::set_int(module_accessor,0,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
    WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    return L2CValue::I32(0)
}

pub unsafe fn wait_air_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let min_stick = Common::min_stick;
    if ControlModule::get_stick_x(owner).abs() > min_stick
    || ControlModule::get_stick_y(owner).abs() > min_stick {
        lioleusboss_movement(owner);
    }
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let input_1 = fighter.global_table[CMD_CAT1].get_i32();
    let input_2 = fighter.global_table[CMD_CAT2].get_i32();
    lioleusboss_move_inputs_air(owner,input_1,input_2);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN) {
        let attack = WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        StatusModule::change_status_request(module_accessor,attack,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_entry_dead_wait(item: &mut L2CAgentBase) {
    let entry_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(entry_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_HOLE_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),entry_coroutine_func);
    let entry_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(entry_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_HOLE_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),entry_status_func);
    let entry_end_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(entry_end as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_HOLE_START),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_END),entry_end_func);
    
    let wait_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(wait_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),wait_coroutine_func);
    let wait_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(wait_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),wait_status_func);
    
    let wait_air_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(wait_air_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),wait_air_coroutine_func);
    let wait_air_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(wait_air_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),wait_air_status_func);
}