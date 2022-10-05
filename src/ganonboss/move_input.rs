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
use crate::ganonboss::*;

pub unsafe fn ganonboss_move_inputs(fighter_boma: *mut BattleObjectModuleAccessor, input_1: i32, input_2: i32) {
    let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let ganonboss_boma = sv_battle_object::module_accessor(boss_id);
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
        WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_DOUBLE_SLASH_EXEC,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
        WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_BIG_JUMP,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
        if PostureModule::lr(ganonboss_boma) * ControlModule::get_stick_x(fighter_boma) > 0.0 {
            WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SLASH_UP,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        }
        else {
            WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_BACK_SLASH,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        }
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
        WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_JUMP_SLASH_EXEC,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
        WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_HOMING_BOMB_HOLD,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
        if PostureModule::lr(ganonboss_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
            PostureModule::reverse_lr(ganonboss_boma);
        }
        WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_BODY_ATTACK_HOLD,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
        WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_THUNDER_SLASH_EXEC,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
        WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_SPIN_SLASH_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if ControlModule::check_button_on(fighter_boma,*CONTROL_PAD_BUTTON_JUMP) {
        if PostureModule::lr(ganonboss_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
            WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_BACK_JUMP,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        }
        else {
            WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_TURN_JUMP,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        }
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0
    || (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0
    || (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0
    || (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0 {
        WorkModule::set_int(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_ATTACK_LASER_BEAM_HOLD,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
}

pub unsafe fn ganonboss_movement(fighter_boma: *mut BattleObjectModuleAccessor) {
    let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let ganonboss_boma = sv_battle_object::module_accessor(boss_id);
    if PostureModule::lr(ganonboss_boma) * ControlModule::get_stick_x(fighter_boma) > 0.0 {
        StatusModule::change_status_request_from_script(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_WALK_FRONT,false);
    }
    else if PostureModule::lr(ganonboss_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
        StatusModule::change_status_request_from_script(ganonboss_boma,*ITEM_GANONBOSS_STATUS_KIND_WALK_BACK,false);
    }
}