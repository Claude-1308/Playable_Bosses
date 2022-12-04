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
use std::arch::asm;

use crate::common::*;
use crate::common::{modules::*,params::*};

pub unsafe fn mh_hippataku_hold_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("hippataku_hold"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_hippataku_hold"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn mh_hippataku_hold_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_HIPPATAKU,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_hippataku_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_drill_start_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("drill_start"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_drill_start"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn mh_drill_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    if dead_range(fighter.lua_state_agent).z.abs() > PostureModule::pos_y(module_accessor) {
        if MotionModule::frame(module_accessor) > 156.0 {
            MotionModule::set_frame(module_accessor,135.0,false);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_DRILL_ATTACK,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_drill_attack_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["mh_drill_attack_status"].get_ptr());
    original(item);
    JostleModule::set_status(module_accessor,false);
    let pos = Vector2f{x: ControlModule::get_stick_x(owner), y: 0.0};
    PostureModule::add_pos_2d(module_accessor,&pos);
    if GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN_RIGHT as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN_LEFT as u32) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_DRILL_END,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_drill_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    GroundModule::set_correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    if MotionModule::frame(module_accessor) > 103.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_scratch_blow_start_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("bump_start"),0.0,1.0,false,0.0,false,false);
    let bump_speed = -5.0;
    SET_SPEED_EX(item,bump_speed * PostureModule::lr(module_accessor),0.0,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    PLAY_SE(item,Hash40::new("se_common_smash_start"));
    return L2CValue::I32(0)
}

pub unsafe fn mh_scratch_blow_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        KineticModule::clear_speed_all(module_accessor);
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_SCRATCH_BLOW_LOOP,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_scratch_blow_loop_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let bump_hold_timer = 300;
    let owner = BossModule::get_owner(module_accessor);
    WorkModule::set_int(owner,bump_hold_timer,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    MotionModule::change_motion(module_accessor,Hash40::new("bump_loop"),0.0,1.0,false,0.0,false,false);
    return L2CValue::I32(0)
}

pub unsafe fn mh_scratch_blow_loop_coroutine(_item: &mut L2CAgentBase) -> L2CValue {
    return L2CValue::I32(0)
}

pub unsafe fn mh_scratch_blow_loop_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if ControlModule::check_button_off(owner,*CONTROL_PAD_BUTTON_ATTACK)
    || WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER) <= 0 {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_SCRATCH_BLOW,false);
    }
    WorkModule::dec_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    let pos = Vector2f{x: 0.0, y: ControlModule::get_stick_y(owner)};
    PostureModule::add_pos_2d(module_accessor,&pos);
    return L2CValue::I32(0)
}

pub unsafe fn mh_scratch_blow_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    STOP_SE(item,Hash40::new("se_common_smash_start"));
    PLAY_SE(item,Hash40::new("se_boss_masterhand_scrachblow_fire_master"));
    EFFECT_FOLLOW(item, Hash40::new_raw(0xf7ecfa005u64), Hash40::new("hand"), 60, 10, 0, 90, 90, 0, 1.85, true);
    LAST_EFFECT_SET_RATE(item,1.5);
    MotionModule::change_motion(module_accessor,Hash40::new("bump_end"),0.0,2.0,false,0.0,false,false);
    let bump_end_speed = 9.0;
    SET_SPEED_EX(item,bump_end_speed * PostureModule::lr(module_accessor),0.0,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let owner = BossModule::get_owner(module_accessor);
    MotionAnimcmdModule::change_script_motion_line_single(owner,*FIGHTER_ANIMCMD_GAME,Hash40::new("bump_end"),-1);
    MotionModule::set_rate(owner,1.5);
    return L2CValue::I32(0)
}

pub unsafe fn mh_scratch_blow_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        let owner = BossModule::get_owner(module_accessor);
        StatusModule::change_status_request(owner,*FIGHTER_STATUS_KIND_FALL,false);
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_WAIT_TIME,false);
    }
    if MotionModule::frame(module_accessor) >= 42.0 {
        KineticModule::clear_speed_all(module_accessor);
        MotionModule::set_rate(module_accessor,1.0);
        EFFECT_OFF_KIND(item,Hash40::new_raw(0xf7ecfa005u64),false,true);
    }
    return L2CValue::I32(0)
}

#[acmd_script(agent = "mario", script = "game_bumpend", category = ACMD_GAME)]
pub unsafe fn bump_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),20.0,361,100,0,30,40.0,0.0,30.0,30.0*PostureModule::lr(boss_boma),None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state,42.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
}

pub unsafe fn mh_paatsubushi_start_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("paatsubushi_hold"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_paatsubushi_hold"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn mh_paatsubushi_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_PAA_TSUBUSHI_HOLD,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_paatsubushi_hold_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("slam_loop"),0.0,1.0,false,0.0,false,false);
    PLAY_SE(item,Hash40::new("se_boss_masterhand_12"));
    let slam_speed = -10.0;
    SET_SPEED_EX(item,0.0,slam_speed,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let owner = BossModule::get_owner(module_accessor);
    MotionAnimcmdModule::change_script_motion_line_single(owner,*FIGHTER_ANIMCMD_GAME,Hash40::new("slam_loop"),-1);
    return L2CValue::I32(0)
}

pub unsafe fn mh_paatsubushi_hold_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN_RIGHT as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN_LEFT as u32) {
        let owner = BossModule::get_owner(module_accessor);
        StatusModule::change_status_request(owner,*FIGHTER_STATUS_KIND_FALL,false);
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_PAA_TSUBUSHI_END,false);
        KineticModule::clear_speed_all(module_accessor);
    }
    let slam_speed = -10.0;
    SET_SPEED_EX(item,0.0,slam_speed,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    return L2CValue::I32(0)
}

#[acmd_script(agent = "mario", script = "game_slamloop", category = ACMD_GAME)]
pub unsafe fn slam_loop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),10.0,270,90,0,40,20.0,0.0,0.0,10.0*PostureModule::lr(boss_boma),Some(0.0),Some(0.0),Some(-15.0*PostureModule::lr(boss_boma)),1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_PUNCH);
    }
}

pub unsafe fn mh_paatsubushi_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("paatsubushi"),6.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_paatsubushi"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn mh_paatsubushi_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    for i in 0..2 {
        if AttackModule::is_attack(module_accessor,i,false) {
            AttackModule::set_power(module_accessor,i,15.0,false);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let mh_hippataku_hold_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_hippataku_hold_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_HIPPATAKU_HOLD),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),mh_hippataku_hold_pre_func);
    let mh_hippataku_hold_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_hippataku_hold_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_HIPPATAKU_HOLD),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_hippataku_hold_status_func);

    let mh_hippataku_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_hippataku_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_HIPPATAKU),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_hippataku_status_func);

    let mh_drill_start_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_drill_start_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_DRILL_START),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),mh_drill_start_pre_func);
    let mh_drill_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_drill_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_DRILL_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_drill_start_status_func);

    let owner = BossModule::get_owner(&mut *item.module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    fighter.global_table["mh_drill_attack_status"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_DRILL_ATTACK),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS)));
    let mh_drill_attack_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_drill_attack_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_DRILL_ATTACK),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_drill_attack_status_func);

    let mh_drill_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_drill_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_DRILL_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_drill_end_status_func);

    let mh_scratch_blow_start_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_scratch_blow_start_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_SCRATCH_BLOW_START),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),mh_scratch_blow_start_pre_func);
    let mh_scratch_blow_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_scratch_blow_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_SCRATCH_BLOW_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_scratch_blow_start_status_func);
    
    let mh_scratch_blow_loop_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_scratch_blow_loop_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_SCRATCH_BLOW_LOOP),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),mh_scratch_blow_loop_pre_func);
    let mh_scratch_blow_loop_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_scratch_blow_loop_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_SCRATCH_BLOW_LOOP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),mh_scratch_blow_loop_coroutine_func);
    let mh_scratch_blow_loop_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_scratch_blow_loop_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_SCRATCH_BLOW_LOOP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_scratch_blow_loop_status_func);

    let mh_scratch_blow_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_scratch_blow_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_SCRATCH_BLOW),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),mh_scratch_blow_pre_func);
    let mh_scratch_blow_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_scratch_blow_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_SCRATCH_BLOW),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_scratch_blow_status_func);

    let mh_paatsubushi_start_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_paatsubushi_start_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_PAA_TSUBUSHI_START),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),mh_paatsubushi_start_pre_func);
    let mh_paatsubushi_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_paatsubushi_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_PAA_TSUBUSHI_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_paatsubushi_start_status_func);

    let mh_paatsubushi_hold_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_paatsubushi_hold_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_PAA_TSUBUSHI_HOLD),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),mh_paatsubushi_hold_pre_func);
    let mh_paatsubushi_hold_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_paatsubushi_hold_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_PAA_TSUBUSHI_HOLD),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_paatsubushi_hold_status_func);

    let mh_paatsubushi_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_paatsubushi_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_PAA_TSUBUSHI_END),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),mh_paatsubushi_pre_func);
    let mh_paatsubushi_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_paatsubushi_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_PAA_TSUBUSHI_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_paatsubushi_status_func);
}