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
pub mod fs_playable;
use fs_playable::*;
pub mod move_input;
use move_input::*;
pub mod normals;
use normals::*;
pub mod specials;
use specials::*;
pub mod movement;
use movement::*;
pub mod status_setting;
use status_setting::*;
pub mod weapons;
use weapons::*;

pub const ITEM_GANONBOSS_STATUS_KIND_ENTRY: i32 = 0x76;

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    match info.name {
        "item" => {
            unsafe {
                GANONBOSS_WAIT_SETTING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ganonboss_wait_setting);
                GANONBOSSSHOT_SPEED_1 += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ganonbossshot_speed_1);
                GANONBOSSSHOT_SPEED_2 += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(ganonbossshot_speed_2);
            }
        },
        _ => {}
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    install_agent_frame!(fsganonboss);
    install_status_scripts!(
        ganon_final_main,
        ganon_fall_main,
        ganon_final_end_main,
        ganon_dead_pre,
    );
}