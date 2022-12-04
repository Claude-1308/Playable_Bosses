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
                DARZ_TEAR_UP_SETTING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_tear_up_setting);
                DARZ_SPACE_RUSH_INTERVAL_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_space_rush_interval_fix);
                DARZ_SPACE_RUSH_NUM_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_space_rush_num_fix);
                DARZ_SPACE_RUSH_PARAM_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_space_rush_param_fix);
                DARZ_PILLAR_FRAME_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_pillar_frame_fix);
                DARZ_CENTIPEDE_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_centipede_fix);
                DARZ_CENTIPEDE_LENGTH_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_centipede_length_fix);
                DARZ_SUMMON_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_summon_fix);
                DARZ_PIERCE_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_pierce_fix);
                DARZ_PIERCE_INTERVAL1_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_pierce_interval1_fix);
                DARZ_PIERCE_INTERVAL_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_pierce_interval_fix);
                DARZ_GATLING_FRAME_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_gatling_frame_fix);
                DARZ_GATLING_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_gatling_fix);
                DARZ_TEAR_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_tear_fix);
                DARZ_HOOK1 += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_hook1);
                DARZ_HOOK2 += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(darz_hook2);
            }
        },
        _ => {}
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}