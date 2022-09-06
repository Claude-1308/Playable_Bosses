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
use crate::common::modules::*;
use crate::crazyhand::move_input::*;

pub unsafe fn ch_okuhikouki_start_setting(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
    if smashball::is_training_mode() == false {
        crazyhand_sync(owner);
    }
    init_status_data(lua_state,ItemKineticType(*ITEM_KINETIC_TYPE_MOTION_LINKED),SituationKind(*SITUATION_KIND_AIR),GroundCorrectKind(*GROUND_CORRECT_KIND_NONE),true);
    return L2CValue::I32(0)
}

pub unsafe fn ch_okuhikouki_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["ch_okuhikouki_status"].get_ptr());
    original(item);
    let pos = Vector2f{x: ControlModule::get_stick_x(owner), y: ControlModule::get_stick_y(owner)};
    PostureModule::add_pos_2d(module_accessor,&pos);
    for i in 0..6 {
        if AttackModule::is_attack(module_accessor,i,false) {
            AttackModule::set_power(module_accessor,i,22.0,false);
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_okuhikouki_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::set_rate(module_accessor,2.0);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_yubi_bomb_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["ch_yubi_bomb_status"].get_ptr());
    original(item);
    let pos = Vector2f{x: ControlModule::get_stick_x(owner), y: 0.0};
    PostureModule::add_pos_2d(module_accessor,&pos);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_yubi_beam_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["ch_yubi_beam_status"].get_ptr());
    original(item);
    let pos = Vector2f{x: ControlModule::get_stick_x(owner), y: 0.0};
    PostureModule::add_pos_2d(module_accessor,&pos);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_super_specials(item: &mut L2CAgentBase) {
    let ch_okuhikouki_start_setting_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_okuhikouki_start_setting as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_HIKOUKI_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_SETTING),ch_okuhikouki_start_setting_func);

    let owner = BossModule::get_owner(&mut *item.module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    fighter.global_table["ch_okuhikouki_status"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_HIKOUKI),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS)));

    let ch_okuhikouki_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_okuhikouki_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_HIKOUKI),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_okuhikouki_status_func);

    let ch_okuhikouki_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_okuhikouki_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_HIKOUKI_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_okuhikouki_end_status_func);

    fighter.global_table["ch_yubi_bomb_status"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_BOMB_ATTACK),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS)));

    let ch_yubi_bomb_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_yubi_bomb_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_BOMB_ATTACK),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_yubi_bomb_status_func);

    fighter.global_table["ch_yubi_beam_status"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_YUBI_BEAM),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS)));

    let ch_yubi_beam_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_yubi_beam_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_YUBI_BEAM),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_yubi_beam_status_func);
}