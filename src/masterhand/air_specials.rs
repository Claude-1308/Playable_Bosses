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

pub static mut MH_YUBIDEPPOU_ROT_SPEED : usize = 0x528e1c;

#[skyline::hook(replace = MH_YUBIDEPPOU_ROT_SPEED, inline)]
pub unsafe fn mh_yubideppou_rot_speed(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[28].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let rot_speed: f32 = 2.0;
    asm!("fmov s0, w8", in("w8") rot_speed);
}

pub static mut MH_YUBIDEPPOU_HOMING_TIME : usize = 0x530e20;

#[skyline::hook(replace = MH_YUBIDEPPOU_HOMING_TIME, inline)]
pub unsafe fn mh_yubideppou_homing_time(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 90;
}

pub unsafe fn mh_yubideppou_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    PLAY_SE(item,Hash40::new("se_boss_masterhand_teppou_flash"));
    if ControlModule::check_button_on(owner,*CONTROL_PAD_BUTTON_GUARD) {
        MotionModule::change_motion(module_accessor,Hash40::new("yubideppou_rensha"),0.0,1.0,false,0.0,false,false);
        boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_yubideppou_rensha"),0.0);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("yubideppou"),0.0,1.0,false,0.0,false,false);
        boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_yubideppou"),0.0);
    }
    boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_wait_brake"));
    WorkModule::off_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_YUBIDEPPOU);
    return L2CValue::I32(0)
}

pub unsafe fn mh_yubideppou_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub static mut MH_CHAKRAM_RETURN_X_MAX_SPEED : usize = 0x563a6c;

#[skyline::hook(replace = MH_CHAKRAM_RETURN_X_MAX_SPEED, inline)]
pub unsafe fn mh_chakram_return_x_max_speed(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[21].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let max_speed: f32 = 8.5;
    asm!("fmov s0, w8", in("w8") max_speed);
}

pub static mut MH_CHAKRAM_X_SPEED : usize = 0x563434;

#[skyline::hook(replace = MH_CHAKRAM_X_SPEED, inline)]
pub unsafe fn mh_chakram_x_speed(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let speed: f32 = 6.5;
    asm!("fmov s0, w8", in("w8") speed);
}

pub unsafe fn mh_chakram_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    if WorkModule::is_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_CHAKRAM_CREATE) {
        let global_pos = &mut Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: 0.0};
        let pos_1 = ModelModule::joint_global_position(module_accessor,Hash40::new("throw"),global_pos,true);
        let chakram1 = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_MASTERHANDCHAKRAM),pos_1.x,pos_1.y,pos_1.z,PostureModule::lr(module_accessor));
        fighter.global_table["chakram1"].assign(&L2CValue::new_int(chakram1));
        let pos_2 = ModelModule::joint_global_position(module_accessor,Hash40::new("throw2"),global_pos,true);
        let chakram2 = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_MASTERHANDCHAKRAM),pos_2.x,pos_2.y,pos_2.z,PostureModule::lr(module_accessor));
        fighter.global_table["chakram2"].assign(&L2CValue::new_int(chakram2));
        let chakram1_boma = fighter.global_table["chakram1"].get_u64() as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(chakram1_boma,true);
        if LinkModule::is_link(chakram1_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(chakram1_boma,*ITEM_LINK_NO_HAVE);
        }
        let boss_id = WorkModule::get_int64(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
        LinkModule::link(chakram1_boma,*ITEM_LINK_NO_HAVE,boss_id);
        LinkModule::set_model_constraint_pos_ort(chakram1_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("throw"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
        WorkModule::on_flag(chakram1_boma,ITEM_INSTANCE_WORK_FLAG_PLAYER);
        let chakram2_boma = fighter.global_table["chakram2"].get_u64() as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(chakram2_boma,true);
        if LinkModule::is_link(chakram2_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(chakram2_boma,*ITEM_LINK_NO_HAVE);
        }
        LinkModule::link(chakram2_boma,*ITEM_LINK_NO_HAVE,boss_id);
        LinkModule::set_model_constraint_pos_ort(chakram2_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("throw2"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
        WorkModule::on_flag(chakram2_boma,ITEM_INSTANCE_WORK_FLAG_PLAYER);
        WorkModule::off_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_CHAKRAM_CREATE);
    }
    if WorkModule::is_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_CHAKRAM_THROW) {
        let chakram1_boma = fighter.global_table["chakram1"].get_u64() as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(chakram1_boma,true);
        if LinkModule::is_link(chakram1_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(chakram1_boma,*ITEM_LINK_NO_HAVE);
        }
        let base_y_chakram = 50.0;
        let pos = Vector2f{x: PostureModule::pos_x(module_accessor) + (base_y_chakram * PostureModule::lr(module_accessor)), y: PostureModule::pos_y(module_accessor) + 28.0};
        PostureModule::set_pos_2d(chakram1_boma,&pos);
        let chakram2_boma = fighter.global_table["chakram2"].get_u64() as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(chakram2_boma,true);
        if LinkModule::is_link(chakram2_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(chakram2_boma,*ITEM_LINK_NO_HAVE);
        }
        let pos = Vector2f{x: PostureModule::pos_x(module_accessor) + (base_y_chakram * PostureModule::lr(module_accessor)), y: PostureModule::pos_y(module_accessor) + 18.0};
        PostureModule::set_pos_2d(chakram2_boma,&pos);
        if ControlModule::check_button_on(owner,*CONTROL_PAD_BUTTON_GUARD) {
            action(chakram1_boma,*ITEM_MASTERHANDCHAKRAM_ACTION_SHOOT3,0.0);
            action(chakram2_boma,*ITEM_MASTERHANDCHAKRAM_ACTION_SHOOT3,0.0);
        }
        else {
            action(chakram1_boma,*ITEM_MASTERHANDCHAKRAM_ACTION_SHOOT2,0.0);
            action(chakram2_boma,*ITEM_MASTERHANDCHAKRAM_ACTION_SHOOT2,0.0);
        }
        WorkModule::off_flag(module_accessor,*ITEM_MASTERHAND_INSTANCE_WORK_FLAG_CHAKRAM_THROW);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_pacchin_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    MotionModule::change_motion(module_accessor,Hash40::new("pacchin_loop"),0.0,1.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_pacchin_loop"),0.0);
    boss_private::sub1_energy_from_param_inherit_all(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_pacchin_loop_homing"));
    let snap_hold_timer = 300;
    WorkModule::set_int(owner,snap_hold_timer,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    return L2CValue::I32(0)
}

pub unsafe fn mh_pacchin_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    boss_private::unable_energy_all(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    WorkModule::dec_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    let pos = Vector2f{x: ControlModule::get_stick_x(owner), y: ControlModule::get_stick_y(owner)};
    PostureModule::add_pos_2d(module_accessor,&pos);
    if WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER) <= 0
    || ControlModule::check_button_off(owner,*CONTROL_PAD_BUTTON_SPECIAL) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_YUBIPACCHIN_END_START,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_pacchin_end_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let power = 10.0;
    for i in 0..10 {
        if AttackModule::is_attack(module_accessor,i,false) {
            AttackModule::set_add_reaction_frame_revised(module_accessor,i,power,false);
        }
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_YUBIPACCHIN_END,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn mh_pacchin_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_MASTERHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn install_air_specials(item: &mut L2CAgentBase) {
    let mh_yubideppou_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_yubideppou_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_YUBIDEPPOU),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),mh_yubideppou_pre_func);

    let mh_yubideppou_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_yubideppou_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_YUBIDEPPOU_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_yubideppou_end_status_func);

    let mh_chakram_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_chakram_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_CHAKRAM_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_chakram_start_status_func);

    let mh_pacchin_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_pacchin_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_YUBIPACCHIN),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),mh_pacchin_pre_func);
    let mh_pacchin_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_pacchin_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_YUBIPACCHIN),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_pacchin_status_func);

    let mh_pacchin_end_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_pacchin_end_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_YUBIPACCHIN_END_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_pacchin_end_start_status_func);

    let mh_pacchin_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(mh_pacchin_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_MASTERHAND_STATUS_KIND_YUBIPACCHIN_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),mh_pacchin_end_status_func);
}