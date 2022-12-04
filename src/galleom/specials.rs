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

pub unsafe fn galleom_missile_fix(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if StatusModule::status_kind(boss_boma) == *ITEM_GALLEOM_STATUS_KIND_MISSILE {
        if MotionModule::rate(boss_boma) <= 1.0 {
            let mul = match WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT) {
                1 => 1.75,
                2 => 2.0,
                _ => 1.5,
            };
            MotionModule::set_rate(boss_boma,mul);
            AttackModule::set_power_mul_status(boss_boma,mul);
        }
    }
}

pub unsafe fn galleom_large_jump_fix(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if StatusModule::status_kind(boss_boma) == *ITEM_GALLEOM_STATUS_KIND_LARGE_JUMP_MAIN {
        let add_pos = Vector2f{x: ControlModule::get_stick_x(module_accessor), y: 0.0};
        PostureModule::add_pos_2d(boss_boma,&add_pos);
        for i in 0..7 {
            if AttackModule::is_attack(boss_boma,i,false) {
                AttackModule::set_power(boss_boma,i,15.0,false);
            }
        }
    }
}

pub unsafe fn galleom_grab_fix(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if StatusModule::status_kind(boss_boma) == *ITEM_GALLEOM_STATUS_KIND_GRAB_CRUSH {
        let mul = match WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT) {
            1 => 1.75,
            2 => 2.0,
            _ => 1.5,
        };
        AttackModule::set_power_mul(boss_boma,mul);
    }
}

pub unsafe fn galleom_jump_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("jump_start"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_jump_start"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_jump_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_LARGE_JUMP_MAIN,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_jump_land_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("jump_land"),0.0,1.5,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_large_jump_landing"),0.0);
    boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_large_jump_landing_brake"));
    return L2CValue::I32(0)
}

pub unsafe fn galleom_jump_land_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_lariat_loop_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    WorkModule::set_int(owner,0,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    MotionModule::change_motion(module_accessor,Hash40::new("lariat_loop"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_lariat_loop"),0.0);
    boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_common_brake"));
    return L2CValue::I32(0)
}

pub unsafe fn galleom_lariat_loop_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    WorkModule::inc_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    if ControlModule::check_button_off(owner,*CONTROL_PAD_BUTTON_SPECIAL)
    || WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER) >= 60 {
        let base = match WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT) {
            1 => 1.25,
            2 => 1.1,
            _ => 1.0,
        };
        let lariat_hold_frames = 60.0;
        let held_frames = WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER) as f32;
        let power_mul = (held_frames/lariat_hold_frames) + 1.0;
        AttackModule::set_power_mul(module_accessor,power_mul);
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_DOUBLE_LARIAT_MAIN,false);
        return L2CValue::I32(1)
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_lariat_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("lariat"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_double_lariat"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_lariat_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_man_to_tank_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    WorkModule::set_int(owner,*ITEM_GALLEOM_STATUS_KIND_SHOOT_MAIN,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
    MotionModule::change_motion(module_accessor,Hash40::new("man_to_tank"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_man_to_tank"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_man_to_tank_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if ControlModule::check_button_on(owner,*CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
        WorkModule::set_int(owner,*ITEM_GALLEOM_STATUS_KIND_RUSH_MAIN,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
    }
    if ControlModule::check_button_on(owner,*CONTROL_PAD_BUTTON_SPECIAL) {
        WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
        WorkModule::set_int(owner,*ITEM_GALLEOM_STATUS_KIND_SHOOT_MAIN,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
    }
    if MotionModule::is_end(module_accessor) {
        let status = WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,status,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_shoot_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("shoot"),0.0,1.5,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_shoot"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_shoot_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["galleom_shoot_status"].get_ptr());
    original(item);
    let mul = match WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT) {
        1 => 1.75,
        2 => 2.0,
        _ => 1.5,
    };
    AttackModule::set_power_mul_status(module_accessor,mul);
    if MotionModule::is_end(module_accessor) {
        if ControlModule::check_button_on(owner,*CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
            StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_RUSH_MAIN,false);
        }
        else {
            StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_SHOOT_END,false);
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_tank_attack_rush_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("tank_attack_rush"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_rush_main"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_tank_attack_rush_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if AttackModule::is_attack(module_accessor,0,false) {
        AttackModule::set_power(module_accessor,0,7.0,false);
    }
    if AttackModule::is_attack(module_accessor,1,false) {
        AttackModule::set_power(module_accessor,1,16.0,false);
    }
    let table = &*(((item as *const L2CAgentBase as u64) + 0x6a8) as *const L2CValue);
    if ((table[0xb395de0deu64].get_f32() - PostureModule::pos_x(module_accessor)) * PostureModule::lr(module_accessor)) < 0.0 {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_RUSH_RETURN,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_tank_attack_return_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let table = &mut *(((item as *const L2CAgentBase as u64) + 0x6a8) as *mut L2CValue);
    if PostureModule::lr(module_accessor) > 0.0 {
        let unk = table[0x1e0a9e367fu64].get_f32();
        table[0xb395de0deu64].assign(&L2CValue::new_num(unk));
    }
    else {
        let unk = table[0x1d3c3f38a5u64].get_f32();
        table[0xb395de0deu64].assign(&L2CValue::new_num(unk));
    }
    PostureModule::reverse_lr(module_accessor);
    PostureModule::update_rot_y_lr(module_accessor);
    MotionModule::change_motion(module_accessor,Hash40::new("tank_attack_return"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_rush_return"),0.0);
    boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_rush_return_movement"));
    return L2CValue::I32(0)
}

pub unsafe fn galleom_tank_attack_return_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let table = &*(((item as *const L2CAgentBase as u64) + 0x6a8) as *const L2CValue);
    if ((table[0xb395de0deu64].get_f32() - PostureModule::pos_x(module_accessor)) * PostureModule::lr(module_accessor)) < 0.0 {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_RUSH_FINISH_START,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_tank_attack_finish_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("tank_attack_finish_start"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_rush_finish_start"),0.0);
    boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_rush_finish_start_brake"));
    return L2CValue::I32(0)
}

pub unsafe fn galleom_tank_attack_finish_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_RUSH_FINISH_END,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_tank_attack_finish_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("tank_attack_finish_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_rush_finish_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_tank_attack_finish_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_RUSH_RETURN_TRANSFORM,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn galleom_tank_to_man_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("tank_to_man"),0.0,1.5,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_GALLEOM),Hash40::new("energy_param_tank_to_man"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn galleom_tank_to_man_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_GALLEOM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_specials(item: &mut L2CAgentBase) {
    let galleom_jump_start_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_jump_start_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_LARGE_JUMP_SQUAT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_jump_start_coroutine_func);
    let galleom_jump_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_jump_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_LARGE_JUMP_SQUAT),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_jump_start_status_func);

    let galleom_jump_land_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_jump_land_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_LARGE_JUMP_LANDING),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_jump_land_coroutine_func);
    let galleom_jump_land_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_jump_land_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_LARGE_JUMP_LANDING),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_jump_land_status_func);

    let galleom_lariat_loop_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_lariat_loop_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_DOUBLE_LARIAT_CHARGE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_lariat_loop_coroutine_func);
    let galleom_lariat_loop_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_lariat_loop_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_DOUBLE_LARIAT_CHARGE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_lariat_loop_status_func);

    let galleom_lariat_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_lariat_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_DOUBLE_LARIAT_MAIN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_lariat_coroutine_func);
    let galleom_lariat_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_lariat_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_DOUBLE_LARIAT_MAIN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_lariat_status_func);
    
    let galleom_man_to_tank_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_man_to_tank_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_SHOOT_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_man_to_tank_coroutine_func);
    let galleom_man_to_tank_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_man_to_tank_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_SHOOT_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_man_to_tank_status_func);
    
    let galleom_shoot_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_shoot_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_SHOOT_MAIN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_shoot_coroutine_func);

    let owner = BossModule::get_owner(&mut *item.module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    fighter.global_table["galleom_shoot_status"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_SHOOT_MAIN),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS)));

    let galleom_shoot_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_shoot_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_SHOOT_MAIN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_shoot_status_func);
    
    let galleom_tank_attack_rush_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_tank_attack_rush_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_RUSH_MAIN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_tank_attack_rush_coroutine_func);
    let galleom_tank_attack_rush_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_tank_attack_rush_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_RUSH_MAIN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_tank_attack_rush_status_func);
    
    let galleom_tank_attack_return_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_tank_attack_return_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_RUSH_RETURN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_tank_attack_return_coroutine_func);
    let galleom_tank_attack_return_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_tank_attack_return_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_RUSH_RETURN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_tank_attack_return_status_func);
    
    let galleom_tank_attack_finish_start_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_tank_attack_finish_start_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_RUSH_FINISH_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_tank_attack_finish_start_coroutine_func);
    let galleom_tank_attack_finish_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_tank_attack_finish_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_RUSH_FINISH_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_tank_attack_finish_start_status_func);
    
    let galleom_tank_attack_finish_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_tank_attack_finish_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_RUSH_FINISH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_tank_attack_finish_end_coroutine_func);
    let galleom_tank_attack_finish_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_tank_attack_finish_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_RUSH_FINISH_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_tank_attack_finish_end_status_func);
    
    let galleom_tank_to_man_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_tank_to_man_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_SHOOT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_tank_to_man_coroutine_func);
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_RUSH_RETURN_TRANSFORM),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),galleom_tank_to_man_coroutine_func);
    let galleom_tank_to_man_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(galleom_tank_to_man_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_SHOOT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_tank_to_man_status_func);
    item.sv_set_status_func(L2CValue::I32(*ITEM_GALLEOM_STATUS_KIND_RUSH_RETURN_TRANSFORM),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),galleom_tank_to_man_status_func);
}
