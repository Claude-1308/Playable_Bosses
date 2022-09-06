use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use crate::FIGHTER_MANAGER;
use crate::ITEM_MANAGER;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::hooks::InlineCtx;

use crate::common::*;
use crate::common::modules::*;

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS) {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("fall"),0.0,1.0,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(fall_main_loop as *const () as _))
    }
    else {
        fighter.status_Fall()
    }
}

unsafe fn fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    BossModule::disable_terms(module_accessor);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::set_correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    smash::app::lua_bind::FighterManager::set_cursor_whole(fighter_manager,false);
    if MotionModule::is_end(module_accessor) {
        MotionModule::change_motion(module_accessor,Hash40::new("fall"),0.0,1.0,false,0.0,false,false);
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_DEAD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn dead_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS) {
        if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS_DEAD) {
            fighter.status_DeadSub();
            let kind = WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_KIND);
            match kind {
                BossKind::MASTERHAND => WorkModule::set_int(module_accessor,Masterhand::finish_frame,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER),
                BossKind::CRAZYHAND => WorkModule::set_int(module_accessor,Crazyhand::finish_frame,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER),
                BossKind::GANONBOSS => WorkModule::set_int(module_accessor,Ganonboss::finish_frame,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER),
                BossKind::LIOLEUSBOSS => {
                    if WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION) == *SITUATION_KIND_AIR {
                        WorkModule::set_int(module_accessor,Lioleusboss::finish_frame_air,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
                    }
                    else {
                        WorkModule::set_int(module_accessor,Lioleusboss::finish_frame_ground,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
                    }
                },
                _ => {},
            };
            fighter.sub_shift_status_main(L2CValue::Ptr(dead_main_loop as *const () as _))
        }
        else {
            WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS_DEAD);
            WorkModule::set_int(module_accessor,10,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
            fighter.sub_shift_status_main(L2CValue::Ptr(dead_main_loop as *const () as _))
        }
    }
    else {
        fighter.status_Dead()
    }
}

unsafe fn dead_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    WorkModule::dec_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    if WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER) <= 0 {
        let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
        let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,boss_id);
        fighter.change_status(FIGHTER_STATUS_KIND_STANDBY.into(),false.into());
    }
    return L2CValue::I32(0)
}