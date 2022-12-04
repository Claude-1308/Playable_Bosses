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

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ENTRY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn entry_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if smash::app::lua_bind::FighterInformation::is_operation_cpu(BossModule::fighter_info(module_accessor)) == false {
        let charas = ["ui_chara_masterhand", "ui_chara_crazyhand", "ui_chara_ganonboss",
                      "ui_chara_lioleus", "ui_chara_galleom", "ui_chara_marx", "ui_chara_dracula",
                      "ui_chara_kiila", "ui_chara_darz"];
        for chara in charas {
            if BOSS_TYPE[entry_id] == hash40(chara) {
                WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS);
                BossModule::allot_boss(module_accessor,BOSS_TYPE[entry_id]);
                break;
            }
        }
    }
    fighter.status_pre_Entry()
}