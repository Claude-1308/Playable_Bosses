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
pub mod aerials;
use aerials::*;
pub mod ground_specials;
use ground_specials::*;
pub mod air_specials;
use air_specials::*;
pub mod movement;
use movement::*;
pub mod turn;
use turn::*;
pub mod status_setting;
use status_setting::*;
//pub mod weapons;
//use weapons::*;

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    match info.name {
        "item" => {
            unsafe {
                LIOLEUSBOSS_HOLE_START_SETTING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_hole_start_setting);
                LIOLEUSBOSS_NAIL_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_nail_ending);
                LIOLEUSBOSS_LOW_FIREBALL_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_low_fireball_ending);
                LIOLEUSBOSS_SKY_SLIP_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_sky_slip_ending);
                LIOLEUSBOSS_SKY_SLIP_ENDING2 += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_sky_slip_ending2);
                LIOLEUSBOSS_LANDING_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_landing_ending);
                LIOLEUSBOSS_TACKLE_JUMP_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_tackle_jump_ending);
                LIOLEUSBOSS_FLIGHT_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_flight_ending);
                LIOLEUSBOSS_BACK_JUMP_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_back_jump_ending);
                LIOLEUSBOSS_BACK_JUMP2_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_back_jump2_ending);
                LIOLEUSBOSS_FIREBALL_ENDING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(lioleusboss_fireball_ending);
            }
        },
        _ => {}
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}