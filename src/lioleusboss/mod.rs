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

pub const ITEM_LIOLEUSBOSS_STATUS_KIND_ENTRY: i32 = 0x81;

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    match info.name {
        "item" => {
            unsafe {
                CH_WAIT_TIME_SETTING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_wait_time_setting);
                CH_NIGIRU_LOOP_PRE += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_nigiru_loop_pre);
                CH_NIGIRU_LOOP_STATUS += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_nigiru_loop_status);
                CH_FIRE_CHARIOT_MOTION += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_chariot_motion);
                CH_CHARIOT_SPEED += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_chariot_speed);
                CH_CHARIOT_RADIUS_MAX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_chariot_radius_max);
                CH_CHARIOT_RADIUS_MIN += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_chariot_radius_min);
                CH_SEARCH_LIGHT_MAX_START_SPEED += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_search_light_max_start_speed);
                CH_SEARCH_LIGHT_ACCEL += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_search_light_accel);
                CH_SEARCH_LIGHT_MAX_SPEED += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_search_light_max_speed);
                CH_SEARCH_LIGHT_CHASE_FRAME += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_search_light_chase_frame);
                CH_GRAVITY_BALL_CHASE_FRAME += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_gravity_ball_chase_frame);
                CH_BOMB_FALL_MAIN += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_bomb_fall_main);
                CH_FIRE_THROW_SUB += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_fire_throw_sub);
                CH_FIRE_PURGE_MAIN += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_fire_purge_main);
                CH_SHOCKWAVE_THROW_SUB += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ch_shockwave_throw_sub);
            }
        },
        _ => {}
    }
}

pub fn install() {
    install_acmd_scripts!(flare_end);
    install_agent_frame_callbacks!(crazyhandfire);
    skyline::nro::add_hook(nro_hook);
}