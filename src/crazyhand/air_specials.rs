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
use std::arch::asm;

use crate::common::*;
use crate::common::{modules::*,params::*};
use crate::crazyhand::*;

pub static mut CH_FIRE_CHARIOT_MOTION : usize = 0x36ba10;

#[skyline::hook(replace = CH_FIRE_CHARIOT_MOTION, inline)]
pub unsafe fn ch_chariot_motion(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let value: u64 = hash40("fire_chariot_start_5");
    asm!("mov x0, {}", in(reg) value);
}

pub static mut CH_CHARIOT_SPEED : usize = 0x36c038;

#[skyline::hook(replace = CH_CHARIOT_SPEED, inline)]
pub unsafe fn ch_chariot_speed(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[22].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let chariot_speed: f32 = 10.0;
    asm!("fmov s0, w8", in("w8") chariot_speed);
}

pub static mut CH_CHARIOT_RADIUS_MIN : usize = 0x36c0fc;

#[skyline::hook(replace = CH_CHARIOT_RADIUS_MIN, inline)]
pub unsafe fn ch_chariot_radius_min(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[22].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let min_radius: f32 = 35.0;
    asm!("fmov s0, w8", in("w8") min_radius);
}

pub static mut CH_CHARIOT_RADIUS_MAX : usize = 0x36c0fc;

#[skyline::hook(replace = CH_CHARIOT_RADIUS_MAX, inline)]
pub unsafe fn ch_chariot_radius_max(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[22].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let max_radius: f32 = 70.0;
    asm!("fmov s0, w8", in("w8") max_radius);
}

pub unsafe fn ch_fire_chariot_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    let original: extern "C" fn(&mut L2CAgentBase) -> L2CValue = std::mem::transmute(fighter.global_table["ch_fire_chariot_end_status"].get_ptr());
    original(item);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

pub static mut CH_SEARCH_LIGHT_MAX_START_SPEED : usize = 0x395f04;

#[skyline::hook(replace = CH_SEARCH_LIGHT_MAX_START_SPEED, inline)]
pub unsafe fn ch_search_light_max_start_speed(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let max_spd: f32 = 6.0;
    asm!("fmov s0, w8", in("w8") max_spd);
}

pub static mut CH_SEARCH_LIGHT_ACCEL : usize = 0x396750;

#[skyline::hook(replace = CH_SEARCH_LIGHT_ACCEL, inline)]
pub unsafe fn ch_search_light_accel(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let accel: f32 = 4.5;
    asm!("fmov s0, w8", in("w8") accel);
}

pub static mut CH_SEARCH_LIGHT_MAX_SPEED : usize = 0x3967b8;

#[skyline::hook(replace = CH_SEARCH_LIGHT_MAX_SPEED, inline)]
pub unsafe fn ch_search_light_max_speed(ctx: &InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let max_spd = 4.5;
    asm!("fmov s0, w8", in("w8") max_spd);
}

pub static mut CH_SEARCH_LIGHT_CHASE_FRAME : usize = 0x396534;

#[skyline::hook(replace = CH_SEARCH_LIGHT_CHASE_FRAME, inline)]
pub unsafe fn ch_search_light_chase_frame(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 90;
}

pub static mut CH_GRAVITY_BALL_CHASE_FRAME : usize = 0x393378;

#[skyline::hook(replace = CH_GRAVITY_BALL_CHASE_FRAME, inline)]
pub unsafe fn ch_gravity_ball_chase_frame(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    *ctx.registers[0].w.as_mut() = 120;
}

pub unsafe fn ch_fire_flare_start_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("flare_start"),0.0,1.0,false,0.0,false,false);
    return L2CValue::I32(0)
}

pub unsafe fn ch_fire_flare_start_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,ITEM_CRAZYHAND_STATUS_KIND_FIRE_FLARE_LOOP,false);
    }
    return L2CValue::I32(0)
}

pub unsafe fn ch_fire_flare_loop_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    let hold_timer = 300;
    WorkModule::set_int(owner,hold_timer,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    MotionModule::change_motion(module_accessor,Hash40::new("flare_loop"),0.0,1.0,false,0.0,false,false);
    return L2CValue::I32(0)
}

pub unsafe fn ch_fire_flare_loop_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    WorkModule::dec_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER);
    if WorkModule::get_int(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_MOVE_HOLD_TIMER) <= 0
    || ControlModule::check_button_off(owner,*CONTROL_PAD_BUTTON_SPECIAL) {
        StatusModule::change_status_request(module_accessor,ITEM_CRAZYHAND_STATUS_KIND_FIRE_FLARE_END,false);
    }
    let pos = Vector2f{x: 0.0, y: ControlModule::get_stick_y(owner)};
    PostureModule::add_pos_2d(module_accessor,&pos);
    return L2CValue::I32(0)
}

pub unsafe fn ch_fire_flare_end_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    MotionModule::change_motion(module_accessor,Hash40::new("flare_end"),0.0,1.0,false,0.0,false,false);
    MotionAnimcmdModule::change_script_motion_line_single(owner,*FIGHTER_ANIMCMD_GAME,Hash40::new("flare_end"),-1);
    return L2CValue::I32(0)
}

pub unsafe fn ch_fire_flare_end_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = BossModule::get_owner(module_accessor);
    if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON) {
        PLAY_SE(item,Hash40::new("se_boss_crazyhand_searchlight_attack"));
        let mut pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: 0.0};
        let global_pos = ModelModule::joint_global_position(module_accessor,Hash40::new("index2"),&mut pos,true);
        let flare = boss_private::create_weapon(lua_state,ItemKind(*ITEM_KIND_CRAZYHANDFIRE),global_pos.x,global_pos.y,0.0,PostureModule::lr(module_accessor)) as *mut BattleObjectModuleAccessor;
        WorkModule::on_flag(flare,ITEM_INSTANCE_WORK_FLAG_PLAYER);
        WorkModule::set_int(flare,CHFireStatus::SHOOT,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        StatusModule::change_status_request(flare,*ITEM_CRAZYHANDFIRE_STATUS_KIND_PURGE,false);
        WorkModule::set_int64(flare,lua_state as i64,ITEM_INSTANCE_WORK_INT_ENTRY_ID);
        WorkModule::off_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON);
    }
    if MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request(module_accessor,*ITEM_CRAZYHAND_STATUS_KIND_WAIT_TIME,false);
    }
    return L2CValue::I32(0)
}

#[acmd_script(agent = "mario", script = "game_flareend", category = ACMD_GAME)]
pub unsafe fn flare_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_CREATE_WEAPON);
    }
}

#[smashline::fighter_frame_callback]
pub fn crazyhandfire(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = sv_system::battle_object_module_accessor(lua_state);
        if fighter.global_table["ch_is_fire_flare_target"].get_bool() {
            if fighter.global_table["ch_is_fire_flare_set"].get_bool() == false {
                let global_pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: 0.0};
                let radius = WorkModule::get_param_float(module_accessor,hash40("edge_flare_radius"),0) + 5.0;
                let calling_fire = sv_battle_object::module_accessor(fighter.global_table["ch_fire_flare_attacker_id"].get_u32());
                let caller_lua = WorkModule::get_int64(calling_fire,ITEM_INSTANCE_WORK_INT_ENTRY_ID);
                let caller = sv_system::battle_object_module_accessor(caller_lua);
                let fire1 = boss_private::create_weapon(caller_lua,ItemKind(*ITEM_KIND_CRAZYHANDFIRE),global_pos.x,global_pos.y - radius,0.0,PostureModule::lr(caller));
                let fire2 = boss_private::create_weapon(caller_lua,ItemKind(*ITEM_KIND_CRAZYHANDFIRE),global_pos.x - (0.866*radius),global_pos.y + (0.5*radius),0.0,PostureModule::lr(caller));
                let fire3 = boss_private::create_weapon(caller_lua,ItemKind(*ITEM_KIND_CRAZYHANDFIRE),global_pos.x + (0.866*radius),global_pos.y + (0.5*radius),0.0,PostureModule::lr(caller));
                fighter.global_table["ch_fire_flare1"].assign(&L2CValue::new_int(fire1));
                fighter.global_table["ch_fire_flare2"].assign(&L2CValue::new_int(fire2));
                fighter.global_table["ch_fire_flare3"].assign(&L2CValue::new_int(fire3));
                let rng_num = 121;
                let rnd = sv_math::rand(hash40("item"),rng_num);
                let base_timer = 180;
                fighter.global_table["ch_fire_flare_timer"].assign(&L2CValue::I32(base_timer + rnd));
                fighter.global_table["ch_fire_flare_angle"].assign(&L2CValue::new_num(0.0));
                fighter.global_table["ch_is_fire_flare_set"].assign(&L2CValue::new_bool(true));
            }
            if fighter.global_table["ch_fire_flare_timer_reset"].get_bool() {
                let rng_num = 121;
                let rnd = sv_math::rand(hash40("item"),rng_num);
                let base_timer = 180;
                fighter.global_table["ch_fire_flare_timer"].assign(&L2CValue::I32(base_timer + rnd));
                fighter.global_table["ch_fire_flare_timer_reset"].assign(&L2CValue::new_bool(false));
            }
            if fighter.global_table["ch_fire_flare_timer"].get_i32() <= 0 {
                fighter.global_table["ch_is_fire_flare_set"].assign(&L2CValue::new_bool(false));
                fighter.global_table["ch_is_fire_flare_target"].assign(&L2CValue::new_bool(false));
                let fire1 = fighter.global_table["ch_fire_flare1"].get_u64() as *mut BattleObjectModuleAccessor;
                let fire2 = fighter.global_table["ch_fire_flare2"].get_u64() as *mut BattleObjectModuleAccessor;
                let fire3 = fighter.global_table["ch_fire_flare3"].get_u64() as *mut BattleObjectModuleAccessor;
                WorkModule::set_int(fire1,CHFireStatus::TYPE1,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
                WorkModule::set_int64(fire1,lua_state as i64,ITEM_INSTANCE_WORK_INT_ENTRY_ID);
                StatusModule::change_status_request(fire1,*ITEM_CRAZYHANDFIRE_STATUS_KIND_PURGE,false);
                WorkModule::set_int(fire2,CHFireStatus::TYPE2,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
                WorkModule::set_int64(fire2,lua_state as i64,ITEM_INSTANCE_WORK_INT_ENTRY_ID);
                StatusModule::change_status_request(fire2,*ITEM_CRAZYHANDFIRE_STATUS_KIND_PURGE,false);
                WorkModule::set_int(fire3,CHFireStatus::TYPE3,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
                WorkModule::set_int64(fire3,lua_state as i64,ITEM_INSTANCE_WORK_INT_ENTRY_ID);
                StatusModule::change_status_request(fire3,*ITEM_CRAZYHANDFIRE_STATUS_KIND_PURGE,false);
            }
            else {
                let timer = fighter.global_table["ch_fire_flare_timer"].get_i32();
                fighter.global_table["ch_fire_flare_timer"].assign(&L2CValue::I32(timer - 1));
                let fire1 = fighter.global_table["ch_fire_flare1"].get_u64() as *mut BattleObjectModuleAccessor;
                let fire2 = fighter.global_table["ch_fire_flare2"].get_u64() as *mut BattleObjectModuleAccessor;
                let fire3 = fighter.global_table["ch_fire_flare3"].get_u64() as *mut BattleObjectModuleAccessor;
                let global_pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: 0.0};
                let radius = WorkModule::get_param_float(module_accessor,hash40("edge_flare_radius"),0) as f32 + 5.0;
                let angle1 = 270.0;
                let angle2 = 30.0;
                let angle3 = 150.0;
                let pos1 = Vector2f{x: global_pos.x + (radius*(fighter.global_table["ch_fire_flare_angle"].get_f32() + angle1).to_radians().cos()), y: global_pos.y + (radius*(fighter.global_table["ch_fire_flare_angle"].get_f32() + angle1).to_radians().sin())};
                let pos2 = Vector2f{x: global_pos.x + (radius*(fighter.global_table["ch_fire_flare_angle"].get_f32() + angle2).to_radians().cos()), y: global_pos.y + (radius*(fighter.global_table["ch_fire_flare_angle"].get_f32() + angle2).to_radians().sin())};
                let pos3 = Vector2f{x: global_pos.x + (radius*(fighter.global_table["ch_fire_flare_angle"].get_f32() + angle3).to_radians().cos()), y: global_pos.y + (radius*(fighter.global_table["ch_fire_flare_angle"].get_f32() + angle3).to_radians().sin())};
                PostureModule::set_pos_2d(fire1,&pos1);
                PostureModule::set_pos_2d(fire2,&pos2);
                PostureModule::set_pos_2d(fire3,&pos3);
                let angle = fighter.global_table["ch_fire_flare_angle"].get_f32();
                let angle_inc = 5.0;
                fighter.global_table["ch_fire_flare_angle"].assign(&L2CValue::new_num(angle + angle_inc));
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
                fighter.global_table["ch_is_fire_flare_target"].assign(&L2CValue::new_bool(false));
            }
        }
    }
}

pub unsafe fn install_air_specials(item: &mut L2CAgentBase) {
    let owner = BossModule::get_owner(&mut *item.module_accessor);
    let fighter = BossModule::get_fighter_common_from_fighter_boma(&mut *owner);
    fighter.global_table["ch_fire_chariot_end_status"].assign(&item.sv_get_status_func(&L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_FIRE_CHARIOT_END),&L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS)));

    let ch_fire_chariot_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_fire_chariot_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_CRAZYHAND_STATUS_KIND_FIRE_CHARIOT_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_fire_chariot_end_status_func);

    let ch_fire_flare_start_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_fire_flare_start_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(ITEM_CRAZYHAND_STATUS_KIND_FIRE_FLARE_START),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),ch_fire_flare_start_pre_func);
    let ch_fire_flare_start_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_fire_flare_start_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(ITEM_CRAZYHAND_STATUS_KIND_FIRE_FLARE_START),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_fire_flare_start_status_func);

    let ch_fire_flare_loop_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_fire_flare_loop_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(ITEM_CRAZYHAND_STATUS_KIND_FIRE_FLARE_LOOP),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),ch_fire_flare_loop_pre_func);
    let ch_fire_flare_loop_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_fire_flare_loop_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(ITEM_CRAZYHAND_STATUS_KIND_FIRE_FLARE_LOOP),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_fire_flare_loop_status_func);

    let ch_fire_flare_end_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_fire_flare_end_pre as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(ITEM_CRAZYHAND_STATUS_KIND_FIRE_FLARE_END),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),ch_fire_flare_end_pre_func);
    let ch_fire_flare_end_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(ch_fire_flare_end_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(ITEM_CRAZYHAND_STATUS_KIND_FIRE_FLARE_END),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),ch_fire_flare_end_status_func);
}