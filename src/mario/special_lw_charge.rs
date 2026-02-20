use super::*;

unsafe extern "C" fn mario_special_lw_charge_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn mario_special_lw_charge_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("hold"), false, -1.0);
    if !StopModule::is_stop(fighter.module_accessor) {
        fun_7100005a80(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_7100005a80 as *const () as _));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_charge_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100005a80(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let special_lw_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_charge_frame"));
    let special_lw_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    if bool_check.get_bool() {
        if special_lw_charge < special_lw_charge_frame {
            WorkModule::set_int(fighter.module_accessor, special_lw_charge+1, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
            if special_lw_charge_frame <= special_lw_charge {
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new_raw(0x193e47e7e5), -1);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new_raw(0x18cc1e6e1a), -1);
                FighterUtil::flash_eye_info(module_accessor);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let special_lw_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    let special_lw_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_charge_frame"));
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_CHARGE_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_charge"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_charge"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_CHARGE_FLAG_FIRST);
                }
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_TRANSITION_TERM_ID_FALL);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_TRANSITION_TERM_ID_WAIT);
            }
        }
        if prev_situation_kind == *SITUATION_KIND_AIR {
            if situation_kind == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_CHARGE_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_charge"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_charge"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_CHARGE_FLAG_FIRST);
                }
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_TRANSITION_TERM_ID_FALL);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_TRANSITION_TERM_ID_WAIT);
            }
        }
    }
    else {
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_CHARGE_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_charge"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_charge"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_CHARGE_FLAG_FIRST);
                }
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_TRANSITION_TERM_ID_FALL);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_TRANSITION_TERM_ID_WAIT);
            }
        }
        if prev_situation_kind == *SITUATION_KIND_AIR {
            if situation_kind == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_CHARGE_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_charge"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_charge"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_CHARGE_FLAG_FIRST);
                }
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_TRANSITION_TERM_ID_FALL);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_TRANSITION_TERM_ID_WAIT);
            }
        }
    }
    if pad_flag & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT.into(), true.into());
    }
    else {
        if situation_kind != *SITUATION_KIND_GROUND {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0 {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
                        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
                    }
                }
            }
            if pad_flag & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 {
                if jump_count < jump_count_max {
                    if ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) {
                            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
                        }
                    }
                }
            }
            if pad_flag & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
                if jump_count < jump_count_max {
                    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
                        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
                    }
                }
            }
            if special_lw_charge_frame <= special_lw_charge {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
        else {
            if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0 {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
                }
            }
            if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0 {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
                }
            }
            if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0 {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
                }
            }
            if fighter.sub_check_command_guard().get_bool() {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
                    fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
                }
            }
            else {
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
                    if fighter.sub_check_button_jump().get_bool() {
                        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
                    }
                }
                if ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 {
                        if fighter.sub_check_button_frick().get_bool() {
                            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
                        }
                    }
                }
                if special_lw_charge_frame <= special_lw_charge {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
            }
        }
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_charge_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let special_lw_remove_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_remove_frame"));
    let special_lw_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    let special_lw_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_charge_frame"));
    if status_kind != *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, *WEAPON_MARIO_PUMP_STATUS_KIND_WAIT, ArticleOperationTarget(0));
        WorkModule::set_int(fighter.module_accessor, special_lw_remove_frame, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_REMOVE);
    }
    if status_kind != *FIGHTER_STATUS_KIND_WAIT {
        if [
            *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_GUARD_ON,
            *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT
        ].contains(&status_kind) {
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_COMMON, Hash40::new("charge_max"));
            sv_module_access::effect(fighter.lua_state_agent);
        }
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    }
    else {
        if special_lw_charge_frame > special_lw_charge {
            return 0.into();
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_COMMON, Hash40::new("charge_max"));
        sv_module_access::effect(fighter.lua_state_agent);
    }
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .status(Pre, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, mario_special_lw_charge_pre_status)
    .status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, mario_special_lw_charge_main_status)
    .status(End, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, mario_special_lw_charge_end_status)
    .install()
    ;
}