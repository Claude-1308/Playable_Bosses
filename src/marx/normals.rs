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

pub unsafe fn marx_four_cutter_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("four_cutter"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MARX),Hash40::new("energy_param_four_cutter"),0.0);
    boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_MARX),Hash40::new("energy_param_wait_brake"));
    WorkModule::off_flag(module_accessor,*ITEM_MARX_INSTANCE_WORK_FLAG_4_CUTTER);
    return L2CValue::I32(0)
}

pub unsafe fn marx_four_cutter_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) < 40.0
    || MotionModule::frame(module_accessor) > 119.0 {
        MotionModule::set_rate(module_accessor,2.0);
    }
    else {
        MotionModule::set_rate(module_accessor,1.0);
    }
    if WorkModule::is_flag(module_accessor,*ITEM_MARX_INSTANCE_WORK_FLAG_4_CUTTER) {
        WorkModule::off_flag(module_accessor,*ITEM_MARX_INSTANCE_WORK_FLAG_4_CUTTER);
        let pos_x = PostureModule::pos_x(module_accessor);
        let pos_y = PostureModule::pos_y(module_accessor);
        let y = item__self_param_float(ItemKind(*ITEM_KIND_MARX),Hash40::new("marxcutter_offset_pos_y"));
        let cutter1 = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_MARXCUTTER),pos_x,pos_y + y,0.0,1.0) as *mut BattleObjectModuleAccessor;
        if cutter1.is_null() == false {
            action(cutter1,*ITEMMARXCUTTER_ACTION_SET_ANGLE,0.0);
        }
        let cutter2 = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_MARXCUTTER),pos_x,pos_y + y,0.0,-1.0) as *mut BattleObjectModuleAccessor;
        if cutter2.is_null() == false {
            action(cutter2,*ITEMMARXCUTTER_ACTION_SET_ANGLE,180.0);
        }
        let cutter3 = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_MARXCUTTER),pos_x,pos_y + y,0.0,-1.0) as *mut BattleObjectModuleAccessor;
        if cutter3.is_null() == false {
            action(cutter3,*ITEMMARXCUTTER_ACTION_SET_ANGLE,225.0);
        }
        let cutter4 = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_MARXCUTTER),pos_x,pos_y + y,0.0,1.0) as *mut BattleObjectModuleAccessor;
        if cutter4.is_null() == false {
            action(cutter4,*ITEMMARXCUTTER_ACTION_SET_ANGLE,315.0);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub static mut MARX_BLACK_HOLE_END_STATUS : usize = 0x4f3d50;

#[skyline::hook(replace = MARX_BLACK_HOLE_END_STATUS, inline)]
pub unsafe fn marx_black_hole_end_status(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    WorkModule::on_flag(agent_base.module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON);
    *ctx.registers[0].w.as_mut() = *ITEM_MARX_STATUS_KIND_MOVE_TELEPORT as u32;
}

pub unsafe fn marx_spew_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("spew_start"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MARX),Hash40::new("energy_param_spew_start"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn marx_spew_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MARX_STATUS_KIND_ATTACK_ICE_BOMB_LOOP,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn marx_spew_loop_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    MotionModule::change_motion(module_accessor,Hash40::new("spew_loop"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MARX),Hash40::new("energy_param_spew_loop"),0.0);
    WorkModule::set_int(owner,0,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    return L2CValue::I32(0)
}

pub unsafe fn marx_spew_loop_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let pos = Vector2f{x: ControlModule::get_stick_x(owner), y: 0.0};
    PostureModule::add_pos_2d(module_accessor,&pos);
    WorkModule::inc_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    if ControlModule::check_button_off(owner,*CONTROL_PAD_BUTTON_ATTACK) {
        let mul = (((WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER) as f32)/60.0) * 0.4) + 1.0;
        AttackModule::set_power_mul_status(module_accessor,mul);
        StatusModule::change_status_request(module_accessor,*ITEM_MARX_STATUS_KIND_ATTACK_ICE_BOMB_SHOOT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let marx_spew_start_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(marx_spew_start_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_ATTACK_ICE_BOMB_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),marx_spew_start_coroutine_func);
    let marx_spew_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(marx_spew_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_ATTACK_ICE_BOMB_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),marx_spew_start_status_func);

    let marx_spew_loop_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(marx_spew_loop_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_ATTACK_ICE_BOMB_LOOP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),marx_spew_loop_coroutine_func);
    let marx_spew_loop_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(marx_spew_loop_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MARX_STATUS_KIND_ATTACK_ICE_BOMB_LOOP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),marx_spew_loop_status_func);
}