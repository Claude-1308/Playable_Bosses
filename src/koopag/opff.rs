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

use crate::common::{*,params::*};
use crate::common::modules::BossModule::fighter_info;

#[fighter_frame(agent = FIGHTER_KIND_KOOPAG)]
pub fn koopag(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH);
        WorkModule::enable_transition_term_forbid_group(module_accessor,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
        WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
        WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_JUMP);
        WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB);
        WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_ATTACK);
        acmd!(lua_state, { sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0) });
        if FighterUtil::is_hp_mode(module_accessor) {
            let player_hp = smash::app::lua_bind::FighterInformation::hit_point(fighter_info(module_accessor));
            if player_hp <= 0.0
            && WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES) == false {
                fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(),false.into());
                WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES);
            }
        }
        else if DamageModule::damage(module_accessor,0) >= Koopag::health
        && WorkModule::is_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES) == false {
            fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(),false.into());
            WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES);
        }
    }
}