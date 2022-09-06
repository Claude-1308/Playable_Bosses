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
use crate::lioleusboss::*;

pub unsafe fn lioleusboss_move_inputs_ground(fighter_boma: *mut BattleObjectModuleAccessor, input_1: i32, input_2: i32) {
    let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let lioleusboss_boma = sv_battle_object::module_accessor(boss_id);
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_BACK_JUMP_FIREBALL,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
        if PostureModule::lr(lioleusboss_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
            PostureModule::reverse_lr(lioleusboss_boma);
        }
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_BACK_JUMP_FIREBALL2,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
        if PostureModule::lr(lioleusboss_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
            PostureModule::reverse_lr(lioleusboss_boma);
        }
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_CHANGE_MODE_GROUND,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_CHARGE_FIREBALL_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0
    || (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0
    || (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0
    || (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_ASSAULT_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TAIL,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
}

pub unsafe fn lioleusboss_move_inputs_air(fighter_boma: *mut BattleObjectModuleAccessor, input_1: i32, input_2: i32) {
    let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let lioleusboss_boma = sv_battle_object::module_accessor(boss_id);
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING_AIR,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL_AIR,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0
    || (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_NAIL_AIR,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_AIR_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
        if PostureModule::lr(lioleusboss_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
            PostureModule::reverse_lr(lioleusboss_boma);
        }
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_GLIDE_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_CHANGE_MODE_AIR,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_CHARGE_FIREBALL_START_AIR,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 {
        WorkModule::set_int(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_TURN_AIR,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
        WorkModule::set_int(ch_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_STEP_AIR,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
}

pub unsafe fn lioleusboss_movement(fighter_boma: *mut BattleObjectModuleAccessor) {
    let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let lioleusboss_boma = sv_battle_object::module_accessor(boss_id);
    if ControlModule::get_stick_x(fighter_boma).abs() > Common::min_stick {
        StatusModule::change_status_request_from_script(lioleusboss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_FLY,false);
    }
}