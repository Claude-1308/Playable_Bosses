pub mod BossModule {

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

    use crate::common::{params::*,*};
    use crate::masterhand::{*,status_setting::*};
    use crate::crazyhand::{*,status_setting::*};
    use crate::ganonboss::{*,status_setting::*};

    pub unsafe fn allot_boss(fighter_boma: &mut BattleObjectModuleAccessor, name: u64) {
        let mut boss_kind = -1;
        if name == hash40("MASTER HAND") {
            boss_kind = BossKind::MASTERHAND;
        }
        else if name == hash40("CRAZY HAND") {
            boss_kind = BossKind::CRAZYHAND;
        }
        else if name == hash40("BEAST GANON") {
            boss_kind = BossKind::GANONBOSS;
        }
        WorkModule::set_int(fighter_boma,boss_kind,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_KIND);
    }

    pub unsafe fn fighter_info(fighter_boma: &mut BattleObjectModuleAccessor) -> *mut smash::app::FighterInformation {
        let entry_id = WorkModule::get_int(fighter_boma,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        return smash::app::lua_bind::FighterManager::get_fighter_information(fighter_manager,FighterEntryID(entry_id));
    }

    pub unsafe fn summon_boss(fighter_boma: &mut BattleObjectModuleAccessor) {
        let boss = WorkModule::get_int(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_KIND);
        let kind = match boss {
            BossKind::MASTERHAND => *ITEM_KIND_MASTERHAND,
            BossKind::CRAZYHAND => *ITEM_KIND_CRAZYHAND,
            BossKind::GANONBOSS => *ITEM_KIND_GANONBOSS,
            _ => 0
        };
        ItemModule::have_item(fighter_boma,ItemKind(kind),0,0,false,false);
        SoundModule::stop_se(fighter_boma,Hash40::new("se_item_item_get"),0);
        let entry_id = WorkModule::get_int(fighter_boma,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boss_id = ItemModule::get_have_item_id(fighter_boma,0) as u32;
        let boss_boma = sv_battle_object::module_accessor(boss_id);
        WorkModule::set_int64(fighter_boma,boss_id as i64,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID);
        WorkModule::on_flag(boss_boma,ITEM_INSTANCE_WORK_FLAG_PLAYER);
        let level = Common::ai_level;
        WorkModule::set_float(boss_boma,level,*ITEM_INSTANCE_WORK_FLOAT_LEVEL);
        WorkModule::set_int(boss_boma,entry_id as i32,ITEM_INSTANCE_WORK_INT_ENTRY_ID);
        if FighterUtil::is_hp_mode(fighter_boma) {
            let player_hp = smash::app::lua_bind::FighterInformation::hit_point(fighter_info(fighter_boma));
            let base_dmg_mul = Common::base_dmg_mul;
            let dmg_mul = (150.0 / player_hp)*base_dmg_mul;
            WorkModule::set_float(fighter_boma,dmg_mul,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_MUL);
            DamageModule::set_damage_mul(boss_boma,dmg_mul);
        }
        else {
            let base_dmg_mul = Common::base_dmg_mul;
            DamageModule::set_damage_mul(boss_boma,base_dmg_mul);
            WorkModule::set_float(fighter_boma,base_dmg_mul,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_MUL);
        }
        WorkModule::set_float(fighter_boma,150.0,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_PREV);
        match boss {
            BossKind::MASTERHAND => {
                StatusModule::change_status_request(boss_boma,*ITEM_MASTERHAND_STATUS_KIND_WAIT_TIME,false);
            },
            BossKind::CRAZYHAND => {
                StatusModule::change_status_request(boss_boma,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
            },
            BossKind::GANONBOSS => {
                WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_PLAY_ENTRY);
                StatusModule::change_status_request(boss_boma,*ITEM_STATUS_KIND_WAIT,false);
            },
            _ => {}
        };
    }

    pub unsafe fn health_manager(fighter_boma: &mut BattleObjectModuleAccessor, prev: f32, curr: f32) {
        let new_curr = (((curr*10.0) as i32) as f32)/10.0;
        if new_curr > 0.0 {
            let dmg_mul = WorkModule::get_float(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_MUL);
            let base_dmg_mul = Common::base_dmg_mul;
            let dmg = (prev - new_curr)/(dmg_mul/base_dmg_mul);
            DamageModule::add_damage(fighter_boma,dmg,0);
            WorkModule::set_float(fighter_boma,new_curr,FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_BOSS_HP_PREV);
        }
        else {
            WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_BOSS_DEAD);
            let fighter = get_fighter_common_from_fighter_boma(fighter_boma);
            fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(),false.into());
            if smash::app::lua_bind::FighterInformation::stock_count(fighter_info(fighter_boma)) <= 1 {
                WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_LAST_STOCK);
            }
            let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
            let boss_boma = sv_battle_object::module_accessor(boss_id);
            StatusModule::change_status_request(boss_boma,*ITEM_STATUS_KIND_DEAD,false);
        }
    }

    pub unsafe fn get_owner(item_boma: &mut BattleObjectModuleAccessor) -> *mut BattleObjectModuleAccessor {
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        let identifier = WorkModule::get_int(item_boma,ITEM_INSTANCE_WORK_INT_ENTRY_ID) as usize;
        let entry = smash::app::lua_bind::FighterManager::get_fighter_entry(fighter_manager,FighterEntryID(identifier as i32));
        let battle_object = *(entry as *mut u64).add(0x4160/8) as *mut u64;
        let owner = *battle_object.add(0x20/8) as *mut BattleObjectModuleAccessor;
        return owner;
    }

    pub unsafe fn disable_terms(fighter_boma: &mut BattleObjectModuleAccessor) {
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_WALL_JUMP);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_TREAD_JUMP);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
        WorkModule::enable_transition_term_forbid_group(fighter_boma,*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
        VisibilityModule::set_whole(fighter_boma,false);
        HitModule::set_whole(fighter_boma,HitStatus(*HIT_STATUS_OFF),0);
        JostleModule::set_status(fighter_boma,false);
        AreaModule::set_whole(fighter_boma,false);
    }

    pub unsafe fn boss_entry_id(item_boma: &mut BattleObjectModuleAccessor) -> usize {
        return WorkModule::get_int(item_boma,ITEM_INSTANCE_WORK_INT_ENTRY_ID) as usize;
    }

    pub unsafe fn get_fighter_common_from_fighter_boma<'a>(fighter_boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
        let lua_module = *(fighter_boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }

    pub unsafe fn install_moves(item: &mut L2CAgentBase, kind: i32) {
        match kind {
            BossKind::MASTERHAND => mh_install_moves(item),
            BossKind::CRAZYHAND => ch_install_moves(item),
            BossKind::GANONBOSS => ganonboss_install_moves(item),
            _ => {}
        };
    }
}