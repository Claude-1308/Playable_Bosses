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
use crate::common::{modules::*,params::*};

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ENTRY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn entry_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS) {
        HitModule::sleep(module_accessor,true);
        AreaModule::set_whole(module_accessor,false);
        fighter.clear_lua_stack();
        lua_args!(fighter,Hash40::new_raw(0x1e61567377u64));
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,Hash40::new_raw(0x1f20a9d549u64),true);
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        BossModule::summon_boss(module_accessor);
        MotionModule::change_motion(module_accessor,Hash40::new("fall"),0.0,1.0,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(entry_main_loop as *const () as _))
    }
    else {
        fighter.status_Entry()
    }
}

unsafe fn entry_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    BossModule::disable_terms(module_accessor);
    GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fighter.set_situation(SITUATION_KIND_AIR.into());
    CAM_ZOOM_OUT(fighter);
    if ArticleModule::is_exist(module_accessor,*FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN) {
        ArticleModule::remove_exist(module_accessor,*FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if MotionModule::frame(module_accessor) >= 20.0 {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS) {
        WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_REBIRTH_STATUS);
        let rebirth_timer = Common::rebirth_timer;
        WorkModule::set_int(module_accessor,rebirth_timer,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_REBIRTH_TIMER);
        WorkModule::off_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS_DEAD);
        WorkModule::off_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
        WorkModule::off_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
        WorkModule::off_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES);
        BossModule::summon_boss(module_accessor);
    }
    fighter.status_Rebirth()
}