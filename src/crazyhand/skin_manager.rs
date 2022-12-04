use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;

use crate::common::*;
use crate::common::modules::*;

pub unsafe fn reset_skins(item_boma: *mut BattleObjectModuleAccessor) {
    let meshes = ["demon_hand_base","demon_panorama_zwriteoff","demon_panorama_zwriteon",
                  "ganon_hand_base","ganon_panorama_zwriteoff","ganon_panorama_zwriteon",
                  "bayo_hand_base","bayo_panorama_zwriteoff","bayo_panorama_zwriteon",
                  "samusd_hand_base","samusd_panorama_zwriteoff","samusd_panorama_zwriteon",
                  "wario_hand_base","wario_panorama_zwriteoff","wario_panorama_zwriteon",
                  "wolf_hand_base","wolf_panorama_zwriteoff","wolf_panorama_zwriteon",
                  "sephiroth_hand_base","sephiroth_panorama_zwriteoff","sephiroth_panorama_zwriteon"];
    for mesh in meshes {
        ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),false);
    }
}

pub unsafe fn set_skins(item_boma: *mut BattleObjectModuleAccessor) {
    let owner = BossModule::get_owner(&mut *item_boma);
    let color_id = WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_COLOR_ID);
    if color_id != 0 {
        let meshes = ["hand_alphae_zwriteoff","hand_alphae_zwriteon",
                      "hand_alphad_zwriteoff","hand_alphad_zwriteon",
                      "hand_alphac_zwriteoff","hand_alphac_zwriteon",
                      "hand_alphab_zwriteoff","hand_alphab_zwriteon",
                      "hand_alphaa_zwriteoff","hand_alphaa_zwriteon",
                      "panorama_zwriteoff","panorama_z_writeon","hand_base"];
        for mesh in meshes {
            ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),false);
        }
        match color_id {
            1 => {
                let meshes = ["wario_hand_base","wario_panorama_zwriteoff","wario_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            2 => {
                let meshes = ["ganon_hand_base","ganon_panorama_zwriteoff","ganon_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            3 => {
                let meshes = ["bayo_hand_base","bayo_panorama_zwriteoff","bayo_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            4 => {
                let meshes = ["samusd_hand_base","samusd_panorama_zwriteoff","samusd_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            5 => {
                let meshes = ["demon_hand_base","demon_panorama_zwriteoff","demon_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            6 => {
                let meshes = ["sephiroth_hand_base","sephiroth_panorama_zwriteoff","sephiroth_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            7 => {
                let meshes = ["wolf_hand_base","wolf_panorama_zwriteoff","wolf_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            _ => {},
        };
    }
}

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

pub unsafe fn allot_slots(item_boma: &mut BattleObjectModuleAccessor) {
    let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
    let name_base = text + 0x52c3758;
    let owner = BossModule::get_owner(&mut *item_boma);
    let entry_id = WorkModule::get_int(owner,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let name = hash40(&read_tag(name_base + 0x260 * entry_id as u64 + 0x8e));
    let mut slot = 0;
    if name == hash40("WARIO CHAND") {
        slot = 1;
    }
    else if name == hash40("GANON CHAND") {
        slot = 2;
    }
    else if name == hash40("BAYONETTA CHAND") {
        slot = 3;
    }
    else if name == hash40("DARK SAMUS CHAND") {
        slot = 4;
    }
    else if name == hash40("KAZUYA CHAND") {
        slot = 5;
    }
    else if name == hash40("SEPHIROTH CHAND") {
        slot = 6;
    }
    else if name == hash40("WOLF CHAND") {
        slot = 7;
    }
    WorkModule::set_int(owner,slot,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_COLOR_ID);
}