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
use crate::dracula2::*;

pub unsafe fn dracula2_move_inputs(fighter_boma: *mut BattleObjectModuleAccessor, input_1: i32, input_2: i32) {
    let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let dracula_boma = sv_battle_object::module_accessor(boss_id);
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
        WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_SLASH,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
        if PostureModule::lr(dracula_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
            WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_TURN_SLASH,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        }
        else {
            WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_STEP_SLASH,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        }
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
        WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_STRIKE,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
        WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_SQUASH_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
        WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_FIRE_SHOT_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
        if PostureModule::lr(dracula_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
            WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_SHOCK_WAVE_TURN,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        }
        else {
            WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_SHOCK_WAVE,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        }
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
        WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_HOMING_SHOT_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
        WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_STEP_STRIKE,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 {
        WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_TURN,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
        WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_SLASH_THREE,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if ControlModule::check_button_trigger(fighter_boma,*CONTROL_PAD_BUTTON_JUMP) {
        if PostureModule::lr(dracula_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
            WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_BACK_JUMP,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        }
        else {
            WorkModule::set_int(dracula_boma,*ITEM_DRACULA2_STATUS_KIND_FRONT_JUMP,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        }
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
}