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

pub static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    let attacker_kind = sv_battle_object::kind(attacker_id);
    if attacker_kind == *ITEM_KIND_CRAZYHANDFIRE && WorkModule::get_int(attacker_boma,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE) == CHFireStatus::SHOOT as i32 {
        if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
        && utility::get_kind(&mut *defender_boma) != *FIGHTER_KIND_KOOPAG {
            let defender = BossModule::get_fighter_common_from_fighter_boma(&mut *defender_boma);
            if defender.global_table["ch_is_fire_flare_target"].get_bool() == false {
                defender.global_table["ch_is_fire_flare_target"].assign(&L2CValue::new_bool(true));
                defender.global_table["ch_fire_flare_attacker_id"].assign(&L2CValue::new_int(attacker_id as u64));
            }
            else {
                defender.global_table["ch_fire_flare_timer_reset"].assign(&L2CValue::new_bool(true));
            }
        }
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again)
}

pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];

pub static mut STAGGER_1 : usize = 0x8e9784;

#[skyline::hook(replace = STAGGER_1, inline)]
pub unsafe fn stagger_1(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[21].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let value: f32 = 201.0;
    asm!("fmov s0, w8", in("w8") value);
}

pub static mut STAGGER_2 : usize = 0x8e9d50;

#[skyline::hook(replace = STAGGER_2, inline)]
pub unsafe fn stagger_2(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[19].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let value: f32 = 201.0;
    asm!("fmov s0, w8", in("w8") value);
}

pub static mut STAGGER_3 : usize = 0x8e9df0;

#[skyline::hook(replace = STAGGER_3, inline)]
pub unsafe fn stagger_3(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[19].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let value: f32 = 201.0;
    asm!("fmov s0, w8", in("w8") value);
}

pub static mut CALL_SOME_SETUP : usize = 0x8da000;

#[skyline::hook(replace = CALL_SOME_SETUP, inline)]
pub unsafe fn call_some_setup(item: &mut L2CAgentBase) {
    original!()(item);
}