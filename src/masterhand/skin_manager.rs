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
    let meshes = ["link_fingers","link_glove","link_panorama_zwriteoff","link_panorama_zwriteon",
                  "mario_glove","mario_panorama_zwriteoff","mario_panorama_zwriteon",
                  "cloud_glove","cloud_panorama_zwriteoff","cloud_panorama_zwriteon",
                  "zss_glove","zss_band",
                  "ryu_fingers","ryu_glove","ryu_panorama_zwriteoff","ryu_panorama_zwriteon",
                  "captain_glove","captain_panorama_zwriteoff","captain_panorama_zwriteon",
                  "joker_glove","joker_panorama_zwriteoff","joker_panorama_zwriteon"];
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
    }
    else {
        match color_id {
            1 => {
                let meshes = ["mario_glove","mario_panorama_zwriteoff","mario_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            2 => {
                let meshes = ["link_fingers","link_glove","link_panorama_zwriteoff","link_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            3 => {
                let meshes = ["captain_glove","captain_panorama_zwriteoff","captain_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            4 => {
                let meshes = ["zss_glove","zss_band"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            5 => {
                let meshes = ["ryu_fingers","ryu_glove","ryu_panorama_zwriteoff","ryu_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            6 => {
                let meshes = ["cloud_glove","cloud_panorama_zwriteoff","cloud_panorama_zwriteon"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            7 => {
                let meshes = ["joker_glove","joker_panorama_zwriteoff","joker_panorama_zwriteon"];
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
    if name == hash40(" MASTER HAND") {
        slot = 1;
    }
    else if name == hash40(" MASTER HAND ") {
        slot = 2;
    }
    else if name == hash40("  MASTER HAND") {
        slot = 3;
    }
    else if name == hash40("  MASTER HAND ") {
        slot = 4;
    }
    else if name == hash40("  MASTER HAND  ") {
        slot = 5;
    }
    else if name == hash40("MASTER  HAND") {
        slot = 6;
    }
    else if name == hash40(" MASTER  HAND") {
        slot = 7;
    }
    WorkModule::set_int(owner,slot,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_COLOR_ID);
}