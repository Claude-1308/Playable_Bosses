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
    let meshes = ["gamemodel_c01","gamemodel_c02","gamemodel_c03","gamemodel_c04",
                  "breakhead_c01","breakhead_c02","breakhead_c03","breakhead_c04",
                  "breakback_c01","breakback_c02","breakback_c03","breakback_c04",
                  "body_c05","eye_c05","body_c06","body_c07","eye_c07"];
    for mesh in meshes {
        ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),false);
    }
}

pub unsafe fn set_skins(item_boma: *mut BattleObjectModuleAccessor) {
    let owner = BossModule::get_owner(&mut *item_boma);
    let color_id = WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_COLOR_ID);
    if color_id != 0 {
        let meshes = ["gamemodel_c00","breakhead_c00","breakback_c00"];
        for mesh in meshes {
            ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),false);
        }
        match color_id {
            1 => {
                let meshes = ["gamemodel_c01","breakhead_c01","breakback_c01"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            2 => {
                let meshes = ["gamemodel_c02","breakhead_c02","breakback_c02"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            3 => {
                let meshes = ["gamemodel_c03","breakhead_c03","breakback_c03"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            4 => {
                let meshes = ["gamemodel_c04","breakhead_c04","breakback_c04"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            5 => {
                let meshes = ["body_c05","eye_c05"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            6 => {
                let meshes = ["body_c06"];
                for mesh in meshes {
                    ModelModule::set_mesh_visibility(item_boma,Hash40::new(&mesh),true);
                }
            },
            7 => {
                let meshes = ["body_c07","eye_c07"];
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
    if name == hash40("MARIO MHAND") {
        slot = 1;
    }
    else if name == hash40("LINK MHAND") {
        slot = 2;
    }
    else if name == hash40("CPT. FALCON MHAND") {
        slot = 3;
    }
    else if name == hash40("ZSS MHAND") {
        slot = 4;
    }
    else if name == hash40("RYU MHAND") {
        slot = 5;
    }
    else if name == hash40("CLOUD MHAND") {
        slot = 6;
    }
    else if name == hash40("JOKER MHAND") {
        slot = 7;
    }
    WorkModule::set_int(owner,slot,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_COLOR_ID);
}