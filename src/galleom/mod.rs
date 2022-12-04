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
pub mod turn;
use turn::*;
pub mod status_setting;
use status_setting::*;

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    match info.name {
        "item" => {
            unsafe {
                GALLEOM_ANGER_SETTING += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(galleom_anger_setting);
            }
        },
        _ => {}
    }
}

pub unsafe fn galleom_meter_manager(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let boss_id = WorkModule::get_int64(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let boss_boma = sv_battle_object::module_accessor(boss_id);
    if FighterUtil::is_hp_mode(module_accessor) == false {
        if DamageModule::damage(module_accessor,0) >= 100.0
        && fighter.global_table["rage_1"].get_bool() == false {
            fighter.global_table["rage_1"].assign(&L2CValue::new_bool(true));
            WorkModule::add_int(module_accessor,1,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT);
        }
        else if DamageModule::damage(module_accessor,0) >= 200.0
        && fighter.global_table["rage_2"].get_bool() == false {
            fighter.global_table["rage_2"].assign(&L2CValue::new_bool(true));
            WorkModule::add_int(module_accessor,1,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT);
        }
    }
    else {
        let tot = WorkModule::get_float(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP);
        let curr = WorkModule::get_float(boss_boma,*ITEM_INSTANCE_WORK_FLOAT_HP);
        if curr <= (2.0 * tot)/3.0
        && fighter.global_table["rage_1"].get_bool() == false {
            fighter.global_table["rage_1"].assign(&L2CValue::new_bool(true));
            WorkModule::add_int(module_accessor,1,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT);
        }
        else if curr <= tot/3.0
        && fighter.global_table["rage_2"].get_bool() == false {
            fighter.global_table["rage_2"].assign(&L2CValue::new_bool(true));
            WorkModule::add_int(module_accessor,1,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT);
        }
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}