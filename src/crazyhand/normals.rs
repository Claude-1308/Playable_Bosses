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

pub unsafe fn ch_hippataku_hold_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("hippataku_hold"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_CRAZYHAND),Hash40::new("energy_param_hippataku_hold"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ch_hippataku_hold_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_HIPPATAKU,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_hippataku_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_drill_start_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("drill_start"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_CRAZYHAND),Hash40::new("energy_param_drill_start"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ch_drill_start_status(item: &mut L2CAgentBase) -> L2CValue {
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
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_DRILL_ATTACK,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_drill_attack_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["ch_drill_attack_status"].get_ptr());
    original(item);
    JostleModule::set_status(module_accessor,false);
    let pos = Vector2f{x: ControlModule::get_stick_x(owner), y: 0.0};
    PostureModule::add_pos_2d(module_accessor,&pos);
    if GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN_RIGHT as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN_LEFT as u32) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_DRILL_END,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_drill_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    GroundModule::set_correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    if MotionModule::frame(module_accessor) > 103.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_grow_finger_loop_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    MotionModule::change_motion(module_accessor,Hash40::new("grow_finger_loop"),0.0,1.0,false,0.0,false,false);
    let hold_timer = 300;
    WorkModule::set_int(owner,hold_timer,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    return L2CValue::I32(0)
}

pub unsafe fn ch_grow_finger_loop_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["ch_grow_finger_loop_status"].get_ptr());
    original(item);
    boss_private::unable_energy_all(lua_state);
    let pos = Vector2f{x: ControlModule::get_stick_x(owner), y: ControlModule::get_stick_y(owner)};
    PostureModule::add_pos_2d(module_accessor,&pos);
    WorkModule::dec_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    if WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER) <= 0
    || ControlModule::check_button_off(owner,*CONTROL_PAD_BUTTON_ATTACK) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_GROW_FINGER_END,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_grow_finger_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["ch_grow_finger_end_status"].get_ptr());
    original(item);
    let power = 20.0;
    for i in 0..2 {
        if AttackModule::is_attack(module_accessor,i,false) {
            AttackModule::set_power(module_accessor,i,power,false);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_scratch_blow_start_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("scratch_start"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_CRAZYHAND),Hash40::new("energy_param_scratch_blow_start"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ch_scratch_blow_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_SCRATCH_BLOW,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_scratch_blow_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("scratch_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_CRAZYHAND),Hash40::new("energy_param_scratch_blow"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn ch_scratch_blow_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) > 100.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    if WorkModule::is_flag(module_accessor,*ITEM_CRAZYHAND_INSTANCE_WORK_FLAG_SCRATCH_JOSTLE_OFF) {
        JostleModule::set_status(module_accessor,false);
        WorkModule::off_flag(module_accessor,*ITEM_CRAZYHAND_INSTANCE_WORK_FLAG_SCRATCH_JOSTLE_OFF);
    }
    if WorkModule::is_flag(module_accessor,*ITEM_CRAZYHAND_INSTANCE_WORK_FLAG_SCRATCH_JOSTLE_ON) {
        JostleModule::set_status(module_accessor,true);
        WorkModule::off_flag(module_accessor,*ITEM_CRAZYHAND_INSTANCE_WORK_FLAG_SCRATCH_JOSTLE_ON);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let ch_hippataku_hold_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_hippataku_hold_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_HIPPATAKU_HOLD),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),ch_hippataku_hold_pre_func);
    let ch_hippataku_hold_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_hippataku_hold_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_HIPPATAKU_HOLD),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_hippataku_hold_status_func);

    let ch_hippataku_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_hippataku_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_HIPPATAKU),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_hippataku_status_func);

    let ch_drill_start_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_drill_start_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_DRILL_START),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),ch_drill_start_pre_func);
    let ch_drill_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_drill_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_DRILL_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_drill_start_status_func);

    let owner = BossModule::get_owner(&mut *item.module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    fighter.global_table["ch_drill_attack_status"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_DRILL_ATTACK),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS)));
    let ch_drill_attack_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_drill_attack_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_DRILL_ATTACK),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_drill_attack_status_func);

    let ch_drill_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_drill_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_DRILL_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_drill_end_status_func);

    fighter.global_table["ch_grow_finger_loop_status"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_GROW_FINGER_LOOP),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS)));

    let ch_grow_finger_loop_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_grow_finger_loop_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_GROW_FINGER_LOOP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),ch_grow_finger_loop_coroutine_func);
    let ch_grow_finger_loop_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_grow_finger_loop_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_GROW_FINGER_LOOP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_grow_finger_loop_status_func);

    fighter.global_table["ch_grow_finger_end_status"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_GROW_FINGER_END),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS)));
    
    let ch_grow_finger_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_grow_finger_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_GROW_FINGER_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_grow_finger_end_status_func);

    let ch_scratch_blow_start_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_scratch_blow_start_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_SCRATCH_BLOW_START),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),ch_scratch_blow_start_pre_func);
    let ch_scratch_blow_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_scratch_blow_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_SCRATCH_BLOW_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_scratch_blow_start_status_func);
    
    let ch_scratch_blow_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_scratch_blow_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_SCRATCH_BLOW),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),ch_scratch_blow_pre_func);
    let ch_scratch_blow_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_scratch_blow_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_SCRATCH_BLOW),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_scratch_blow_status_func);
}