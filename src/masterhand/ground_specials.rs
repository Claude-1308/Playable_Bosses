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

pub unsafe fn mh_nigiru_start(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if MotionModule::motion_kind(boss_boma) == hash40("nigiru_start") {
        if MotionModule::frame(boss_boma) < 20.0 || MotionModule::frame(boss_boma) > 55.0 {
            MotionModule::set_rate(boss_boma,2.0);
        }
        else {
            MotionModule::set_rate(boss_boma,1.0);
        }
    }
}

pub static mut MH_NIGIRU_LOOP_PRE : usize = 0x539510;

#[skyline::hook(replace=MH_NIGIRU_LOOP_PRE)]
pub unsafe fn mh_nigiru_loop_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        MotionModule::change_motion(module_accessor,Hash40::new("nigiru_loop"),0.0,1.0,false,0.0,false,false);
        return L2CValue::I32(0)
    }
    else {
        original!()(item)
    }
}

pub static mut MH_NIGIRU_LOOP_STATUS : usize = 0x5395f0;

#[skyline::hook(replace=MH_NIGIRU_LOOP_STATUS)]
pub unsafe fn mh_nigiru_loop_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        let owner = BossModule::get_owner(module_accessor);
        let min_stick = Common::min_stick;
        if ControlModule::check_button_trigger(owner,*CONTROL_PAD_BUTTON_SPECIAL)
        && ControlModule::get_stick_x(owner).abs() < min_stick
        && MotionModule::motion_kind(module_accessor) == hash40("nigiru_loop") {
            MotionModule::change_motion(module_accessor,Hash40::new("nigiru"),60.0,1.0,false,0.0,false,false);
        }
        if MotionModule::motion_kind(module_accessor) == hash40("nigiru") {
            let power_mul = 2.0;
            AttackModule::set_power_mul(module_accessor,power_mul);
            if MotionModule::frame(module_accessor) >= 70.0 {
                MotionModule::set_rate(module_accessor,2.0);
            }
            if MotionModule::frame(module_accessor) >= 80.0 {
                MotionModule::change_motion(module_accessor,Hash40::new("nigiru_loop"),0.0,1.0,false,0.0,false,false);
            }
        }
        if ControlModule::get_stick_x(owner).abs() > min_stick
        && MotionModule::motion_kind(module_accessor) == hash40("nigiru_loop") {
            AttackModule::set_power_mul(module_accessor,1.0);
            let reaction_mul = 1.5;
            AttackModule::set_reaction_mul(module_accessor,reaction_mul);
            StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_NIGIRU_THROW_END_1,false);
        }
        return L2CValue::I32(0)
    }
    else {
        original!()(item)
    }
}

pub unsafe fn mh_iron_ball_start_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    WorkModule::set_int(owner,0,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT);
    MotionModule::change_motion(module_accessor,Hash40::new("iron_ball_start"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_iron_ball_start"),0.0);
    WorkModule::off_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_IRON_BALL_CREATE);
    WorkModule::off_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_IRON_BALL_THROW);
    return L2CValue::I32(0)
}

pub unsafe fn mh_iron_ball_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_IRON_BALL_CREATE) {
        let global_pos = &mut Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: 0.0};
        let pos = ModelModule::joint_global_position(module_accessor,Hash40::new("throw"),global_pos,true);
        let iron_ball = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_MASTERHANDIRONBALL),pos.x,pos.y,pos.z,PostureModule::lr(module_accessor));
        let owner = BossModule::get_owner(module_accessor);
        let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
        fighter.global_table["iron_ball_boma"].assign(&L2CValue::new_int(iron_ball));
        let iron_ball_boma = iron_ball as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(iron_ball_boma,true);
        if LinkModule::is_link(iron_ball_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(iron_ball_boma,*ITEM_LINK_NO_HAVE);
        }
        WorkModule::on_flag(iron_ball_boma,ITEM_INSTANCE_WORK_FLAG_PLAYER);
        let boss_id = WorkModule::get_int64(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
        LinkModule::link(iron_ball_boma,*ITEM_LINK_NO_HAVE,boss_id);
        LinkModule::set_model_constraint_pos_ort(iron_ball_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("throw"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
        WorkModule::off_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_IRON_BALL_CREATE);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_IRON_BALL,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_iron_ball_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("iron_ball"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_iron_ball"),0.0);
    let owner = BossModule::get_owner(module_accessor);
    WorkModule::add_int(owner,1,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT);
    return L2CValue::I32(0)
}

pub unsafe fn mh_iron_ball_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    if WorkModule::is_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_IRON_BALL_CREATE) {
        let global_pos = &mut Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: 0.0};
        let pos = ModelModule::joint_global_position(module_accessor,Hash40::new("throw"),global_pos,true);
        let iron_ball = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_MASTERHANDIRONBALL),pos.x,pos.y,pos.z,PostureModule::lr(module_accessor));
        fighter.global_table["iron_ball_boma"].assign(&L2CValue::new_int(iron_ball));
        let iron_ball_boma = iron_ball as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(iron_ball_boma,true);
        if LinkModule::is_link(iron_ball_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(iron_ball_boma,*ITEM_LINK_NO_HAVE);
        }
        let boss_id = WorkModule::get_int64(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
        LinkModule::link(iron_ball_boma,*ITEM_LINK_NO_HAVE,boss_id);
        LinkModule::set_model_constraint_pos_ort(iron_ball_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("throw"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
        WorkModule::on_flag(iron_ball_boma,ITEM_INSTANCE_WORK_FLAG_PLAYER);
        WorkModule::off_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_IRON_BALL_CREATE);
    }
    if WorkModule::is_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_IRON_BALL_THROW) {
        let iron_ball = fighter.global_table["iron_ball_boma"].get_u64();
        let iron_ball_boma = iron_ball as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(iron_ball_boma,true);
        if LinkModule::is_link(iron_ball_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(iron_ball_boma,*ITEM_LINK_NO_HAVE);
        }
        if ControlModule::check_button_on(owner,*CONTROL_PAD_BUTTON_GUARD) {
            let action_iron_ball = *ITEM_MASTERHANDIRONBALL_ACTION_SET_BOUND;
            fighter.global_table["action_iron_ball"].assign(&L2CValue::I32(action_iron_ball));
            if iron_ball_boma.is_null() == false {
                action(iron_ball_boma,action_iron_ball,0.0);
            }
        }
        else {
            let action_iron_ball = *ITEM_MASTERHANDIRONBALL_ACTION_SET_STRAIGHT;
            fighter.global_table["action_iron_ball"].assign(&L2CValue::I32(action_iron_ball));
            if iron_ball_boma.is_null() == false {
                action(iron_ball_boma,action_iron_ball,0.0);
            }
        }
        WorkModule::off_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_IRON_BALL_THROW);
    }
    let iron_ball = fighter.global_table["iron_ball_boma"].get_u64();
    let iron_ball_boma = iron_ball as *mut BattleObjectModuleAccessor;
    if iron_ball_boma.is_null() == false {
        if MotionModule::motion_kind(iron_ball_boma) == hash40("appear") {
            AttackModule::clear_all(iron_ball_boma);
        }
        if StatusModule::status_kind(iron_ball_boma) > 66 {
            let action_iron_ball = fighter.global_table["action_iron_ball"].get_i32();
            if action_iron_ball == *ITEM_MASTERHANDIRONBALL_ACTION_SET_BOUND {
                StatusModule::change_status_request(iron_ball_boma,*ITEM_MASTERHANDIRONBALL_STATUS_KIND_MOVE2,false);
            }
            else if action_iron_ball == *ITEM_MASTERHANDIRONBALL_ACTION_SET_STRAIGHT {
                StatusModule::change_status_request(iron_ball_boma,*ITEM_MASTERHANDIRONBALL_STATUS_KIND_MOVE1,false);
            }
        }
    }
    if MotionModule::is_end(module_accessor) {
        let iron_ball_limit = 3;
        if WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT) >= iron_ball_limit {
            StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_IRON_BALL_END,false);
        }
        else {
            StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_IRON_BALL,false);
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_iron_ball_end_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let iron_ball = fighter.global_table["iron_ball_boma"].get_u64();
    let iron_ball_boma = iron_ball as *mut BattleObjectModuleAccessor;
    if iron_ball_boma.is_null() == false {
        remove(iron_ball_boma);
    }
    boss_private::unable_energy_all(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("iron_ball_end"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_iron_ball_end"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn mh_iron_ball_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        boss_private::unable_energy_all(lua_state);
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_kenzan_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("kenzan"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_kenzan"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn mh_kenzan_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        let kenzan_boma = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_MASTERHANDKENZAN),PostureModule::pos_x(module_accessor) + (10.0 * PostureModule::lr(module_accessor)),0.0,0.0,PostureModule::lr(module_accessor)) as *mut BattleObjectModuleAccessor;
        let owner = BossModule::get_owner(module_accessor);
        let boss_id = WorkModule::get_int64(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
        LinkModule::link(kenzan_boma,*ITEM_LINK_NO_MESSAGE,boss_id);
        WorkModule::on_flag(kenzan_boma,ITEM_INSTANCE_WORK_FLAG_PLAYER);
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_KENZAN_END,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_ground_specials(item: &mut L2CAgentBase) {
    let mh_iron_ball_start_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_iron_ball_start_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_IRON_BALL_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),mh_iron_ball_start_coroutine_func);
    let mh_iron_ball_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_iron_ball_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_IRON_BALL_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_iron_ball_start_status_func);
    
    let mh_iron_ball_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_iron_ball_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_IRON_BALL),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),mh_iron_ball_coroutine_func);
    let mh_iron_ball_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_iron_ball_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_IRON_BALL),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_iron_ball_status_func);

    let mh_iron_ball_end_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_iron_ball_end_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_IRON_BALL_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),mh_iron_ball_end_coroutine_func);
    let mh_iron_ball_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_iron_ball_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_IRON_BALL_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_iron_ball_end_status_func);

    let mh_kenzan_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_kenzan_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_KENZAN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),mh_kenzan_coroutine_func);
    let mh_kenzan_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_kenzan_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_KENZAN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_kenzan_status_func);
}