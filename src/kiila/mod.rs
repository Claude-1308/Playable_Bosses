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
pub mod specials;
use specials::*;
pub mod teleport;
use teleport::*;
pub mod status_setting;
use status_setting::*;
pub mod hooks;
use hooks::*;

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    match info.name {
        "item" => {
            unsafe {
                KIILA_TORRENT_SETTING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_torrent_setting);
                KIILA_CRUSH_DOWN_NUM_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_crush_down_num_fix);
                KIILA_CRUSH_DOWN_WAIT_TIME_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_crush_down_wait_time_fix);
                KIILA_CRUSH_DOWN_WAVE_NUM_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_crush_down_wave_num_fix);
                KIILA_MISSILE_STOP_FRAME_FIX += (*info.module.ModuleObject).module_base as usize;
                //skyline::install_hook!(kiila_missile_stop_frame_fix);
                KIILA_MISSILE_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_missile_fix);
                KIILA_LASER_TIME_NUM_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_laser_time_num_fix);
                KIILA_LASER_NUM_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_laser_num_fix);
                KIILA_LASER_WAIT_TIME_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_laser_wait_time_fix);
                KIILA_THREAT_NUM_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_threat_num_fix);
                KIILA_SUMMON_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_summon_fix);
                KIILA_HOOK1 += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(kiila_hook1);
            }
        },
        _ => {}
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}