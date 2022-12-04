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
use std::arch::asm;

use crate::common::*;
use crate::common::modules::*;
use crate::darz::*;

pub static mut DARZ_HOOK1 : usize = 0x3c60a0;

#[skyline::hook(replace = DARZ_HOOK1, inline)]
pub unsafe fn darz_hook1(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let value: f32 = -1.0;
    asm!("fmov s0, w8", in("w8") value);
}

pub static mut DARZ_HOOK2 : usize = 0x3ba080;

#[skyline::hook(replace = DARZ_HOOK2)]
pub unsafe fn darz_hook2(item: &mut L2CAgentBase, unk: L2CValue) {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        original!()(item,unk)
    }
}