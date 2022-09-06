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

pub mod entry_wait_dead;
use entry_wait_dead::*;
pub mod move_input;
use move_input::*;
pub mod normals;
use normals::*;
pub mod ground_specials;
use ground_specials::*;
pub mod air_specials;
use air_specials::*;
pub mod super_specials;
use super_specials::*;
pub mod teleport_turn;
use teleport_turn::*;
pub mod status_setting;
use status_setting::*;
pub mod weapons;
use weapons::*;
pub mod skin_manager;
use skin_manager::*;

pub const ITEM_MASTERHAND_STATUS_KIND_ENTRY: i32 = 0x97;

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    match info.name {
        "item" => {
            unsafe {
                MH_WAIT_TIME_SETTING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(mh_wait_time_setting);
                MH_NIGIRU_LOOP_PRE += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(mh_nigiru_loop_pre);
                MH_NIGIRU_LOOP_STATUS += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(mh_nigiru_loop_status);
                MH_YUBIDEPPOU_ROT_SPEED += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(mh_yubideppou_rot_speed);
                MH_YUBIDEPPOU_HOMING_TIME += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(mh_yubideppou_homing_time);
                MH_CHAKRAM_RETURN_X_MAX_SPEED += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(mh_chakram_return_x_max_speed);
                MH_CHAKRAM_X_SPEED += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(mh_chakram_x_speed);
                MH_CHAKRAM_THROW_SUB += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(mh_chakram_throw_sub);
                MH_IRON_BALL_THROW_SUB += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(mh_iron_ball_throw_sub);
                MH_KENZAN_NEEDLE_SUB += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(mh_kenzan_needle_sub);
            }
        },
        _ => {}
    }
}

pub fn install() {
    install_acmd_scripts!(
        bump_end,
        slam_loop,
    );
    skyline::nro::add_hook(nro_hook);
}