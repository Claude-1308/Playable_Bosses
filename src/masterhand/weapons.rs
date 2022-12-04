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
use crate::masterhand::move_input::*;

pub static mut MH_CHAKRAM_THROW_SUB : usize = 0x5643f0;

#[skyline::hook(replace=MH_CHAKRAM_THROW_SUB)]
pub unsafe fn mh_chakram_throw_sub(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
    }
    original!()(item)
}

pub static mut MH_IRON_BALL_THROW_SUB : usize = 0x569d50;

#[skyline::hook(replace=MH_IRON_BALL_THROW_SUB)]
pub unsafe fn mh_iron_ball_throw_sub(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
    }
    original!()(item)
}

pub static mut MH_KENZAN_NEEDLE_SUB : usize = 0x56e7f0;

#[skyline::hook(replace=MH_KENZAN_NEEDLE_SUB)]
pub unsafe fn mh_kenzan_needle_sub(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
        if AttackModule::is_attack(module_accessor,1,false) {
            AttackModule::set_target_category(module_accessor,1,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
    }
    original!()(item)
}