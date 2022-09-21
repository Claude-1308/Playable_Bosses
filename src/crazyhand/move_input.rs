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
use crate::crazyhand::*;

pub unsafe fn crazyhand_move_inputs(fighter_boma: *mut BattleObjectModuleAccessor, input_1: i32, input_2: i32) {
    let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let ch_boma = sv_battle_object::module_accessor(boss_id);
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
        WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_HIPPATAKU_HOLD,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
        WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_DRILL_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
        if PostureModule::lr(ch_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
            PostureModule::reverse_lr(ch_boma);
        }
        WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_GROW_FINGER_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
        WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_SCRATCH_BLOW_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    let pos = Vector3f{x: PostureModule::pos_x(fighter_boma), y: PostureModule::pos_y(fighter_boma), z: PostureModule::pos_z(fighter_boma)};
    if GroundModule::get_distance_to_floor(fighter_boma,&pos,pos.y,true) <= 30.0
    && GroundModule::get_distance_to_floor(fighter_boma,&pos,pos.y,true) > 0.0 {
        if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
            WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_NIGIRU_CAPTURE,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
            WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
        }
        if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
            if PostureModule::lr(ch_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
                PostureModule::reverse_lr(ch_boma);
            }
            WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_KUMO,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
            WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
        }
        if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
            WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_DIG_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
            WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
        }
        if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
            WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_NOTAUTSU,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
            WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
        }
    }
    else {
        if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
            if PostureModule::pos_y(ch_boma) > 0.0 {
                WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_FIRE_CHARIOT_PRE_MOVE,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
            }
            else {
                WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_NIGIRU_CAPTURE,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
            }
            WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
        }
        if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
            if PostureModule::lr(ch_boma) * ControlModule::get_stick_x(fighter_boma) < 0.0 {
                PostureModule::reverse_lr(ch_boma);
            }
            WorkModule::set_int(ch_boma,ITEM_CRAZYHAND_STATUS_KIND_FIRE_FLARE_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
            WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
        }
        if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
            WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_GRAVITY_BALL_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
            WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
        }
        if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
            WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_LOOK_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
            WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
        }
    }
    if (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 {
        WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_HIKOUKI_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0
    || (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 {
        WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_BOMB_ATTACK_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0 {
        WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_YUBI_BEAM,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 {
        WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TELEPORT,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
        WorkModule::set_int(ch_boma,*ITEM_CRAZYHAND_STATUS_KIND_TURN,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
}

pub unsafe fn crazyhand_movement(fighter_boma: *mut BattleObjectModuleAccessor) {
    let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let ch_boma = sv_battle_object::module_accessor(boss_id);
    let move_mul = Crazyhand::move_mul;
    let pos = Vector2f{x: ControlModule::get_stick_x(fighter_boma)*move_mul, y: ControlModule::get_stick_y(fighter_boma)*move_mul};
    PostureModule::add_pos_2d(ch_boma,&pos);
}