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

pub mod check_boss;
use check_boss::*;
pub mod manage_mario;
use manage_mario::*;
pub mod modules;
use modules::*;
pub mod opff;
use opff::*;
pub mod summon_boss;
use summon_boss::*;
pub mod hooks;
use hooks::*;
pub mod params;
use params::*;

pub struct BossKind(i32);

impl BossKind {
    pub const MASTERHAND: i32 = 0;
    pub const CRAZYHAND: i32 = 1;
    pub const GANONBOSS: i32 = 2;
    pub const LIOLEUSBOSS: i32 = 3;
}

pub const CMD_CAT1: i32 = 0x20;
pub const CMD_CAT2: i32 = 0x21;

pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS: i32 = 0x200000e3;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS_DEAD: i32 = 0x200000e4;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN: i32 = 0x200000e5;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_RESET_BOSS: i32 = 0x200000e6; //remove
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS: i32 = 0x200000e7;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_LAST_STOCK: i32 = 0x200000e8;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SET_FINISH_FRAME: i32 = 0x200000e9; //remove
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_TRAINING_BOSS_ALLOTED: i32 = 0x200000ea;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES: i32 = 0x200000eb;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_REBIRTH_STATUS: i32 = 0x200000ec;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON: i32 = 0x200000ed;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_PLAY_ENTRY: i32 = 0x200000ee;

pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_KIND: i32 = 0x100000bf;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_REBIRTH_TIMER: i32 = 0x100000c0;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID: i32 = 0x100000c1;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_CUTSCENE_TIMER: i32 = 0x100000c2;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER: i32 = 0x100000c3;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_WEAPON_SPAWN_COUNT: i32 = 0x100000c4;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_COLOR_ID: i32 = 0x100000c5;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION: i32 = 0x100000c6;

pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP: i32 = 0x4d;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_PREV: i32 = 0x4e;

pub const ITEM_INSTANCE_WORK_FLAG_PLAYER : i32 = 0x20000033;
pub const ITEM_INSTANCE_WORK_INT_ATTACK_TYPE : i32 = 0x20000034;
pub const ITEM_INSTANCE_WORK_FLAG_ATTACK_END : i32 = 0x20000035;
pub const ITEM_INSTANCE_WORK_INT_ENTRY_ID : i32 = 0x20000036;

extern "C" {
    #[link_name = "\u{1}_ZN3app4item10add_damageEP9lua_Statef"]
    pub fn item_add_damage(lua_state: u64, damage: f32);
}

extern "C" {
    #[link_name = "\u{1}_ZN3app4item6set_hpEP9lua_Statef"]
    pub fn set_hp(lua_state: u64, hp: f32);
}

extern "C" {
    #[link_name = "\u{1}_ZN3app4item26set_visibility_whole_forceEP9lua_Stateb"]
    pub fn set_visibility_whole_force(lua_state: u64, visibility: bool);
}

extern "C" {
    #[link_name = "\u{1}_ZN3app4item16init_status_dataEP9lua_StateNS_15ItemKineticTypeENS_13SituationKindENS_17GroundCorrectKindEb"]
    pub fn init_status_data(lua_state: u64, kinetic_type: ItemKineticType, situation: SituationKind, correction: GroundCorrectKind, unk: bool);
}

extern "C" {
    #[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
    pub fn dead_range(lua_state: u64) -> Vector4f;
}

extern "C" {
    #[link_name = "\u{1}_ZN3app10item_other6actionEPNS_26BattleObjectModuleAccessorEif"]
    pub fn action(module_accessor: *mut BattleObjectModuleAccessor, action: i32, unk: f32);
}

extern "C" {
    #[link_name = "\u{1}_ZN3app10item_other6removeEPNS_26BattleObjectModuleAccessorE"]
    pub fn remove(module_accessor: *mut BattleObjectModuleAccessor);
}

extern "C" {
    #[link_name = "\u{1}_ZN3app4item8owner_idEP9lua_State"]
    pub fn owner_id(lua_state: u64) -> u32;
}

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    match info.name {
        "item" => {
            unsafe {
                STAGGER_1 += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(stagger_1);
                STAGGER_2 += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(stagger_2);
                STAGGER_3 += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(stagger_3);
                STAGGER_4 += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(stagger_4);
            }
        },
        _ => {}
    }
}

pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
        LookupSymbol(
            &mut ITEM_MANAGER,
            "_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    install_status_scripts!(
        entry_pre,
        fall_main,
        dead_main,
        entry_main,
        rebirth_main,
    );
    install_agent_frame!(common);
    skyline::nro::add_hook(nro_hook);
    skyline::install_hook!(notify_log_event_collision_hit_replace);
}