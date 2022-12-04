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
pub mod movement;
use movement::*;

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    match info.name {
        "item" => {
            unsafe {
                MARX_4_CUTTER_SETTING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(marx_4_cutter_setting);
                MARX_THICK_LASER_STOP_FRAME += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(marx_thick_laser_stop_frame);
                MARX_BLACK_HOLE_END_STATUS += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(marx_black_hole_end_status);
                MARX_THICK_LASER_END_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(marx_thick_laser_end_ending);
                MARX_PLANT_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(marx_plant_ending);
            }
        },
        _ => {}
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}