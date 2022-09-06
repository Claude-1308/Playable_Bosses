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

pub unsafe fn lioleusboss_howling_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("howling"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_howling"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_howling_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_back_jump_fire_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("back_jump_fire"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_back_jump_fireball"),0.0);
    WorkModule::off_flag(module_accessor,*ITEM_LIOLEUSBOSS_INSTANCE_WORK_FLAG_SHOOT);
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_back_jump_fire_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,*ITEM_LIOLEUSBOSS_INSTANCE_WORK_FLAG_SHOOT) {
        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        let global_pos = ModelModule::joint_global_position(module_accessor,Hash40::new("jaw"),&pos,true);
        let fireball = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),global_pos.x,global_pos.y,global_pos.z,PostureModule::lr(module_accessor)) as *mut BattleObjectModuleAccessor;
        if fireball.is_null() == false {
            action(fireball,*ITEM_LIOLEUSBOSSSHOT_ACTION_SET_ANGLE_BACKJUMP,-45.0);
        }
        WorkModule::off_flag(module_accessor,*ITEM_LIOLEUSBOSS_INSTANCE_WORK_FLAG_SHOOT);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_back_jump_fire2_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("back_jump_fire2"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_back_jump_fireball2"),0.0);
    WorkModule::off_flag(module_accessor,*ITEM_LIOLEUSBOSS_INSTANCE_WORK_FLAG_SHOOT);
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_back_jump_fire2_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,*ITEM_LIOLEUSBOSS_INSTANCE_WORK_FLAG_SHOOT) {
        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        let global_pos = ModelModule::joint_global_position(module_accessor,Hash40::new("jaw"),&pos,true);
        let fireball = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),global_pos.x,global_pos.y,global_pos.z,PostureModule::lr(module_accessor)) as *mut BattleObjectModuleAccessor;
        if fireball.is_null() == false {
            action(fireball,*ITEM_LIOLEUSBOSSSHOT_ACTION_SET_ANGLE_BACKJUMP,-45.0);
        }
        WorkModule::off_flag(module_accessor,*ITEM_LIOLEUSBOSS_INSTANCE_WORK_FLAG_SHOOT);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_fireball_shot_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let mut motion: u64 = hash40("fireball_shot");
    if ControlModule::get_stick_y(module_accessor) > Common::min_stick {
        motion = hash40("fireball_shot_up");
        fighter.global_table["up?"].assign(&L2CValue::new_bool(true));
    }
    MotionModule::change_motion(module_accessor,Hash40::new_raw(motion),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_fireball"),0.0);
    WorkModule::off_flag(module_accessor,*ITEM_LIOLEUSBOSS_INSTANCE_WORK_FLAG_SHOOT);
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_fireball_shot_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    if WorkModule::is_flag(module_accessor,*ITEM_LIOLEUSBOSS_INSTANCE_WORK_FLAG_SHOOT) {
        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        let global_pos = ModelModule::joint_global_position(module_accessor,Hash40::new("jaw"),&pos,true);
        let fireball = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),global_pos.x,global_pos.y,global_pos.z,PostureModule::lr(module_accessor)) as *mut BattleObjectModuleAccessor;
        if fireball.is_null() == false {
            let mut angle: f32 = 0.0;
            if fighter.global_table["up?"].get_bool() {
                angle = 45.0;
            }
            action(fireball,*ITEM_LIOLEUSBOSSSHOT_ACTION_SET_ANGLE,angle);
        }
        WorkModule::off_flag(module_accessor,*ITEM_LIOLEUSBOSS_INSTANCE_WORK_FLAG_SHOOT);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let lioleusboss_howling_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_howling_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_howling_coroutine_func);
    let lioleusboss_howling_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_howling_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_howling_status_func);

    let lioleusboss_back_jump_fire_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_back_jump_fire_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_BACK_JUMP_FIREBALL),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_back_jump_fire_coroutine_func);
    let lioleusboss_back_jump_fire_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_back_jump_fire_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_BACK_JUMP_FIREBALL),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_back_jump_fire_status_func);

    let lioleusboss_back_jump_fire2_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_back_jump_fire2_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_BACK_JUMP_FIREBALL2),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_back_jump_fire2_coroutine_func);
    let lioleusboss_back_jump_fire2_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_back_jump_fire2_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_BACK_JUMP_FIREBALL2),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_back_jump_fire_status2_func);

    let lioleusboss_fireball_shot_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_fireball_shot_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_fireball_shot_coroutine_func);
    let lioleusboss_fireball_shot_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_fireball_shot_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_fireball_shot_status_func);
}