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
use crate::crazyhand::*;

pub static mut CH_BOMB_FALL_MAIN : usize = 0x38e1e0;

#[skyline::hook(replace=CH_BOMB_FALL_MAIN)]
pub unsafe fn ch_bomb_fall_main(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
    }
    original!()(item)
}

pub static mut CH_FIRE_THROW_SUB : usize = 0x38f890;

#[skyline::hook(replace=CH_FIRE_THROW_SUB)]
pub unsafe fn ch_fire_throw_sub(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_NO_STAGE as u32);
        }
    }
    original!()(item)
}

pub static mut CH_FIRE_PURGE_MAIN : usize = 0x390c40;

#[skyline::hook(replace=CH_FIRE_PURGE_MAIN)]
pub unsafe fn ch_fire_purge_main(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE) == CHFireStatus::SHOOT {
        MotionModule::change_motion(module_accessor,Hash40::new("throw"),0.0,1.0,false,0.0,false,false);
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_power(module_accessor,0,1.5,false);
            AttackModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
        let owner = sv_battle_object::module_accessor(owner_id(lua_state));
        let lr = PostureModule::lr(owner);
        let pos = Vector2f{x: PostureModule::pos_x(owner) + (32.5*lr), y: PostureModule::pos_y(owner) + (35.0*lr)};
        PostureModule::set_pos_2d(module_accessor,&pos);
        SET_SPEED_EX(item,5.0*PostureModule::lr(owner),0.0,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        return L2CValue::I32(0)
    }
    else if WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE) == CHFireStatus::TYPE1 {
        MotionModule::change_motion(module_accessor,Hash40::new("throw"),0.0,1.0,false,0.0,false,false);
        let defender_lua = WorkModule::get_int64(module_accessor,ITEM_INSTANCE_WORK_INT_ENTRY_ID);
        let defender_boma = sv_system::battle_object_module_accessor(defender_lua);
        let global_pos = Vector3f{x: PostureModule::pos_x(defender_boma), y: PostureModule::pos_y(defender_boma), z: 0.0};
        let radius = WorkModule::get_param_float(defender_boma,hash40("edge_flare_radius"),0) + 5.0;
        let pos = Vector2f{x: global_pos.x, y: global_pos.y - radius};
        PostureModule::set_pos_2d(module_accessor,&pos);
        SET_SPEED_EX(item,0.0,3.0,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
        return L2CValue::I32(0)
    }
    else if WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE) == CHFireStatus::TYPE2 {
        MotionModule::change_motion(module_accessor,Hash40::new("throw"),0.0,1.0,false,0.0,false,false);
        let defender_lua = WorkModule::get_int64(module_accessor,ITEM_INSTANCE_WORK_INT_ENTRY_ID);
        let defender_boma = sv_system::battle_object_module_accessor(defender_lua);
        let global_pos = Vector3f{x: PostureModule::pos_x(defender_boma), y: PostureModule::pos_y(defender_boma), z: 0.0};
        let radius = WorkModule::get_param_float(defender_boma,hash40("edge_flare_radius"),0) + 5.0;
        let pos = Vector2f{x: global_pos.x - (radius*0.866), y: global_pos.y + (radius*0.5)};
        PostureModule::set_pos_2d(module_accessor,&pos);
        SET_SPEED_EX(item,3.0*0.866,-3.0*0.5,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
        return L2CValue::I32(0)
    }
    else if WorkModule::get_int(module_accessor,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE) == CHFireStatus::TYPE3 {
        MotionModule::change_motion(module_accessor,Hash40::new("throw"),0.0,1.0,false,0.0,false,false);
        let defender_lua = WorkModule::get_int64(module_accessor,ITEM_INSTANCE_WORK_INT_ENTRY_ID);
        let defender_boma = sv_system::battle_object_module_accessor(defender_lua);
        let global_pos = Vector3f{x: PostureModule::pos_x(defender_boma), y: PostureModule::pos_y(defender_boma), z: 0.0};
        let radius = WorkModule::get_param_float(defender_boma,hash40("edge_flare_radius"),0) + 5.0;
        let pos = Vector2f{x: global_pos.x + (radius*0.866), y: global_pos.y + (radius*0.5)};
        PostureModule::set_pos_2d(module_accessor,&pos);
        SET_SPEED_EX(item,-3.0*0.866,-3.0*0.5,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
        return L2CValue::I32(0)
    }
    else {
        original!()(item)
    }
}

pub static mut CH_SHOCKWAVE_THROW_SUB : usize = 0x39b1f0;

#[skyline::hook(replace=CH_SHOCKWAVE_THROW_SUB)]
pub unsafe fn ch_shockwave_throw_sub(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        if AttackModule::is_attack(module_accessor,0,false) {
            AttackModule::set_power_mul(module_accessor,1.5);
            AttackModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
        if AttackModule::is_attack(module_accessor,1,false) {
            AttackModule::set_power_mul(module_accessor,1.5);
            AttackModule::set_target_category(module_accessor,1,*COLLISION_CATEGORY_MASK_ALL as u32);
        }
    }
    original!()(item)
}