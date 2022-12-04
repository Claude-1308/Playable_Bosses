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
use crate::ganonboss::{normals::*,specials::*,move_input::*};
use crate::galleom::{specials::*,galleom_meter_manager};
use crate::crazyhand::move_input::*;
use crate::masterhand::{move_input::*,ground_specials::*};
use crate::lioleusboss::{air_specials::*,ground_specials::*,aerials::*};
use crate::marx::specials::*;

#[smashline::fighter_frame(agent = FIGHTER_KIND_MARIO)]
pub fn common(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = sv_system::battle_object_module_accessor(lua_state);
        if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_TRAINING_BOSS_ALLOTED) == false
        && smash::app::lua_bind::FighterInformation::is_operation_cpu(BossModule::fighter_info(module_accessor)) == false {
            if (sv_information::is_ready_go()
            && smashball::is_training_mode())
            || (StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_WAIT
            && WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS) == false) {
                let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                let charas = ["ui_chara_masterhand", "ui_chara_crazyhand", "ui_chara_ganonboss",
                        "ui_chara_lioleus", "ui_chara_galleom", "ui_chara_marx", "ui_chara_dracula"];
                for chara in charas {
                    if BOSS_TYPE[entry_id] == hash40(chara) {
                        WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS);
                        BossModule::allot_boss(module_accessor,BOSS_TYPE[entry_id]);
                        BossModule::summon_boss(module_accessor);
                        break;
                    }
                }
                WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_TRAINING_BOSS_ALLOTED);
            }
        }
        if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS) {
            BossModule::disable_terms(module_accessor);
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
                if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_REBIRTH_STATUS) {
                    HitModule::set_whole(boss_boma,HitStatus(*HIT_STATUS_INVINCIBLE),0);
                    WorkModule::dec_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_REBIRTH_TIMER);
                    if WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_REBIRTH_TIMER) <= 0 {
                        HitModule::set_whole(boss_boma,HitStatus(*HIT_STATUS_NORMAL),0);
                        WorkModule::off_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_REBIRTH_STATUS);
                    }
                }
                for i in 0..10 {
                    if AttackModule::is_attack(boss_boma,i,false) {
                        AttackModule::set_target_category(boss_boma,i,*COLLISION_CATEGORY_MASK_ALL as u32);
                    }
                }
                if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS) == false {
                    let pos = Vector3f{x: PostureModule::pos_x(boss_boma), y: PostureModule::pos_y(boss_boma), z: 0.0};
                    PostureModule::set_pos(module_accessor,&pos);
                    smash::app::lua_bind::FighterManager::set_position_lock(fighter_manager,FighterEntryID(entry_id as i32),false);
                }
                else {
                    smash::app::lua_bind::FighterManager::set_position_lock(fighter_manager,FighterEntryID(entry_id as i32),true);
                }
                if kind == BossKind::MASTERHAND {
                    mh_nigiru_start(fighter);
                }
                if kind == BossKind::GANONBOSS {
                    ganonboss_thunder_slash_fix(fighter);
                    ganonboss_body_attack_loop_fix(fighter);
                    ganonboss_big_jump_fix(fighter);
                }
                if kind == BossKind::LIOLEUSBOSS {
                    glide_correct(fighter);
                    low_fireball_shot3_start_rate_fix(fighter);
                    fireball_shot3_start_rate_fix(fighter);
                    assault_loop_fix(fighter);
                    lioleusboss_low_fireball_fix(fighter);
                    lioleusboss_nail_fix(fighter);
                    landing_correct(fighter);
                    lioleusboss_tackle_jump_fix(fighter);
                }
                if kind == BossKind::GALLEOM {
                    galleom_missile_fix(fighter);
                    galleom_large_jump_fix(fighter);
                    galleom_grab_fix(fighter);
                    galleom_meter_manager(fighter);
                }
                if kind == BossKind::MARX {
                    capillary_rate_fix(fighter);
                    eye_laser_rate_fix(fighter);
                    thick_laser_dmg_fix(fighter);
                }
            }
            else {
                let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
                let boss_boma = sv_battle_object::module_accessor(boss_id);
                STOP_SE(fighter,Hash40::new("vc_mario_missfoot01"));
                STOP_SE(fighter,Hash40::new("vc_mario_missfoot02"));
                let kind = WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_KIND);
                let mut rate = Common::non_last_stock_mt_rate;
                if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_LAST_STOCK) {
                    rate = 1.0;
                    MotionModule::set_rate(boss_boma,rate);
                    WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
                    EFFECT(fighter,Hash40::new("sys_bg_boss_finishhit"),Hash40::new("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,false);
                    let dead_slow = Common::dead_slow;
                    SlowModule::set_whole(module_accessor,dead_slow,0);
                    let cutscene_timer = Common::cutscene_timer;
                    WorkModule::set_int(module_accessor,cutscene_timer,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_CUTSCENE_TIMER);
                    WorkModule::off_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_LAST_STOCK);
                }
                if WorkModule::get_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_CUTSCENE_TIMER) <= 0 {
                    EFFECT_OFF_KIND(fighter,Hash40::new("sys_bg_boss_finishhit"),true,false);
                    SlowModule::clear_whole(module_accessor);
                    CAM_ZOOM_OUT(fighter);
                }
                else {
                    WorkModule::dec_int(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_CUTSCENE_TIMER);
                }
            }
        }
        /*let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        if smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) {
            CameraModule::set_camera_type(module_accessor,1);
            CAM_ZOOM_IN_arg5(
                fighter,
                /*frames to zoom in totally*/2.0,
                /*idk best kept 0*/0,
                /*the amt of zoom needed, higher the number lower the zoom*/2.0,
                /*y_rot*/1.2,
                /*x_rot*/0.2
            );
        }*/
    }
}