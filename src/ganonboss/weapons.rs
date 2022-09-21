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
use crate::common::modules::*;
use crate::ganonboss::*;

pub static mut GANONBOSSSHOT_SPEED_1 : usize = 0x458744;

#[skyline::hook(replace = GANONBOSSSHOT_SPEED_1, inline)]
pub unsafe fn ganonbossshot_speed_1(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let value: f32 = 2.0;
    asm!("fmov s0, w8", in("w8") value);
}

pub static mut GANONBOSSSHOT_SPEED_2 : usize = 0x4587d8;

#[skyline::hook(replace = GANONBOSSSHOT_SPEED_2, inline)]
pub unsafe fn ganonbossshot_speed_2(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let value: f32 = 2.0;
    asm!("fmov s0, w8", in("w8") value);
}