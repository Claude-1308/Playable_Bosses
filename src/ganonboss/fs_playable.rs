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

use crate::common::{*,manage_mario::fall_main,opff::common};
use crate::common::{modules::*,params::*};
use crate::ganonboss::*;

#[smashline::fighter_frame(agent = FIGHTER_KIND_GANON)]
pub fn fsganonboss(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = sv_system::battle_object_module_accessor(lua_state);
        if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS) {
            if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS_DEAD) == false {
                let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
                let kind = WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_KIND);
                let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
                let boss_boma = sv_battle_object::module_accessor(boss_id);
                let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                let prev = WorkModule::get_float(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_PREV);
                let curr = WorkModule::get_float(boss_boma,*ITEM_INSTANCE_WORK_FLOAT_HP);
                if curr != prev {
                    BossModule::health_manager(module_accessor,prev,curr);
                }
                if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_REBIRTH_STATUS) {
                    BossModule::disable_terms(module_accessor);
                }
                for i in 0..10 {
                    if AttackModule::is_attack(boss_boma,i,false) {
                        AttackModule::set_target_category(boss_boma,i,*COLLISION_CATEGORY_MASK_ALL as u32);
                    }
                }
                if smashball::is_training_mode() {
                    if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS) == false {
                        let pos = Vector3f{x: PostureModule::pos_x(boss_boma), y: PostureModule::pos_y(boss_boma), z: 0.0};
                        PostureModule::set_pos(module_accessor,&pos);
                        smash::app::lua_bind::FighterManager::set_position_lock(fighter_manager,FighterEntryID(entry_id as i32),false);
                    }
                    else {
                        smash::app::lua_bind::FighterManager::set_position_lock(fighter_manager,FighterEntryID(entry_id as i32),true);
                    }
                }
                if kind == BossKind::GANONBOSS {
                    ganonboss_thunder_slash_fix(fighter);
                    ganonboss_body_attack_loop_fix(fighter);
                    ganonboss_big_jump_fix(fighter);
                }
                WorkModule::dec_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
                if WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER) <= 0
                && (StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_WAIT
                || StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_TRANS_PHASE) {
                    fighter.change_status(FIGHTER_GANON_STATUS_KIND_FINAL_END.into(),false.into());
                }
            }
        }
        if fighter.global_table["fs_ganon_ending"].get_bool() {
            let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
            let boss_boma = sv_battle_object::module_accessor(boss_id);
            if PostureModule::pos_x(boss_boma).abs() > dead_range(lua_state).x.abs() {
                let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
                smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,boss_id);
                fighter.global_table["fs_ganon_ending"].assign(&L2CValue::new_bool(false));
            }
        }
        if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS_DEAD) {
            let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
            let kind = WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_KIND);
            STOP_SE(fighter,Hash40::new("vc_mario_missfoot01"));
            STOP_SE(fighter,Hash40::new("vc_mario_missfoot02"));
            if kind != BossKind::GANONBOSS {
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_dead"),true,true);
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_dead_dark"),true,true);
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_dead_flash"),true,true);
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_dead_front"),true,true);
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_dead_light"),true,true);
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_dead2"),true,true);
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_dead2_ground"),true,true);
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_hit_dead"),true,true);
            }
            if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_RESET_BOSS) {
                println!("reset");
                let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
                smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,boss_id);
                fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(),false.into());
                WorkModule::off_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_RESET_BOSS);
            }
        }
    }
}

#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_FINAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn ganon_final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if Ganonboss::playable_fs
    && smash::app::lua_bind::FighterInformation::is_operation_cpu(BossModule::fighter_info(module_accessor)) == false {
        WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS);
        WorkModule::set_int(module_accessor,BossKind::GANONBOSS,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_KIND);
        WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_PLAY_ENTRY);
        BossModule::summon_boss(module_accessor);
        MotionModule::change_motion(module_accessor,Hash40::new("boss_entry"),30.0,1.0,false,0.0,false,false);
        let timer = Ganonboss::fs_timer;
        WorkModule::set_int(module_accessor,timer,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
        fighter.global_table["fs_ganon_ending"].assign(&L2CValue::new_bool(false));
        fighter.sub_shift_status_main(L2CValue::Ptr(ganon_final_main_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

unsafe fn ganon_final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_XLU),0);
    if MotionModule::frame(module_accessor) >= 70.0 {
        VisibilityModule::set_whole(module_accessor,false);
    }
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn ganon_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS) {
        fall_main(fighter)
    }
    else {
        fighter.status_Fall()
    }
}

#[status_script(agent = "ganon", status = FIGHTER_GANON_STATUS_KIND_FINAL_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn ganon_final_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS) {
        let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
        let boss_boma = sv_battle_object::module_accessor(boss_id);
        VisibilityModule::set_whole(module_accessor,true);
        HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_NORMAL),0);
        JostleModule::set_status(module_accessor,true);
        AreaModule::set_whole(module_accessor,true);
        smash::app::lua_bind::FighterManager::set_position_lock(fighter_manager,FighterEntryID(entry_id),false);
        if PostureModule::lr(boss_boma) * PostureModule::pos_x(boss_boma) > 0.0 {
            PostureModule::reverse_lr(boss_boma);
        }
        StatusModule::change_status_request(boss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_BODY_ATTACK_HOLD,false);
        WorkModule::off_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS);
        fighter.global_table["fs_ganon_ending"].assign(&L2CValue::new_bool(true));
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion(module_accessor,Hash40::new("final_air_end"),0.0,1.0,false,0.0,false,false);
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("final_end"),0.0,1.0,false,0.0,false,false);
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(ganon_final_end_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

unsafe fn ganon_final_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_XLU),0);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    smash::app::lua_bind::FighterManager::set_position_lock(fighter_manager,FighterEntryID(entry_id as i32),false);
    if MotionModule::is_end(module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_DEAD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn ganon_dead_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS)
    && WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS_DEAD) == false {
        if LinkModule::is_link(module_accessor,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(module_accessor,*ITEM_LINK_NO_HAVE);
        }
        WorkModule::off_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS);
        let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
        let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,boss_id);
    }
    fighter.status_pre_Dead()
}