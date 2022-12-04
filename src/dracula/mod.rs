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

pub mod entry_wait_dead;
use entry_wait_dead::*;
pub mod move_input;
use move_input::*;
pub mod normals;
use normals::*;
pub mod specials;
use specials::*;
pub mod status_setting;
use status_setting::*;
pub mod teleport;
use teleport::*;

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    match info.name {
        "item" => {
            unsafe {
                DRACULA_CHANGE_SETTING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(dracula_change_setting);
                DRACULA_STRAIGHT_SHOT_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(dracula_straight_shot_fix);
                DRACULA_PILLAR_NUM_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(dracula_pillar_num_fix);
                DRACULA_PILLAR_FRAME_FIX += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(dracula_pillar_frame_fix);
            }
        },
        _ => {}
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}