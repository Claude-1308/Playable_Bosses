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

pub unsafe fn read_tag(addr: u64) -> String {
    let mut s: Vec<u8> = vec![];
    let mut addr = addr as *const u16;
    loop {
        if *addr == 0_u16 {
            break;
        }
        s.push(*(addr as *const u8));
        addr = addr.offset(1);
    }
    std::str::from_utf8(&s).unwrap().to_owned()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ENTRY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn entry_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if smash::app::lua_bind::FighterInformation::is_operation_cpu(BossModule::fighter_info(module_accessor)) == false {
        let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let name_base = text + 0x52c3758;
        let name: u64 = hash40(&read_tag(name_base + 0x260 * entry_id as u64 + 0x8e));
        let boss_names = ["MASTER HAND", "CRAZY HAND", "BEAST GANON"];
        for boss_name in boss_names {
            if hash40(boss_name) == name {
                WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS);
                BossModule::allot_boss(module_accessor,name);
            }
        }
    }
    fighter.status_pre_Entry()
}