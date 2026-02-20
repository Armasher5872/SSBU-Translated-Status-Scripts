use super::*;

unsafe extern "C" fn mario_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn mario_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 0.into();
            }
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
                }
            }
        }
        if prev_situation_kind == *SITUATION_KIND_AIR {
            if situation_kind == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
                }
            }
        }
    }
    else {
        if situation_kind != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
            }
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn mario_special_n_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, mario_special_n_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, mario_special_n_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, mario_special_n_end_status)
    .install()
    ;
}