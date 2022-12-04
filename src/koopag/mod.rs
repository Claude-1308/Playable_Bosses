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

mod acmd;
use acmd::*;
mod opff;
use opff::*;
mod status;
use status::*;

pub fn install() {
    install_acmd_scripts!(
        attack_12,
        attack_s3_s,
        attack_s3_hi,
        attack_s3_lw,
        attack_hi3,
        attack_s4_s,
        attack_hi4,
        attack_lw4,
        cliff_attack,
        attack_air_b,
        attack_air_f,
        attack_air_hi,
        turn,
        special_s_catch,
    );
    install_agent_frame!(koopag);
    install_status_scripts!(
        escape_b_main,
        dead,
    );
}