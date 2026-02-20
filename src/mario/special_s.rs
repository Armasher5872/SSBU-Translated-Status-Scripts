use super::*;

unsafe extern "C" fn mario_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn mario_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if ![hash40("special_s"), hash40("special_s_c2"), hash40("special_s_c3"), hash40("special_s_c4")].contains(&motion_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_HOP);
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_HOP);
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
        mario_special_s_sub_status(fighter);
    }
    0.into()
}

unsafe extern "C" fn mario_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    }
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_AIR {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
                    mario_special_s_sub_status(fighter);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE);
                }
            }
        }
        if prev_situation_kind == *SITUATION_KIND_AIR {
            if situation_kind == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE);
                }
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

unsafe extern "C" fn mario_special_s_sub_status(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.global_table[0x5].get_ptr() as *mut BattleObjectModuleAccessor;
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_s_start_mul_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_start_mul_spd_x"));
    let special_s_start_air_acl_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_start_air_acl_x"));
    let speed_x = get_sum_speed_x/special_s_start_mul_spd_x;
    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, speed_x, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_s_start_air_acl_x, 0.0);
    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, get_sum_speed_y, 0.0, 0.0, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
}

unsafe extern "C" fn mario_special_s_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let special_s_attack_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_attack_spd_y"));
    let special_s_attack_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_attack_acl_y"));
    let special_s_attack_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_attack_max_y"));
    let speed_y;
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL) {
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        }
        else {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_HOP) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_HOP);
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP) {
                    speed_y = 0.0;
                }
                else {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
                    speed_y = special_s_attack_spd_y;
                }
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
            }
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_s_attack_acl_y);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_s_attack_max_y);
        }
    }
    0.into()
}

unsafe extern "C" fn mario_special_s_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn mario_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, ArticleOperationTarget(0));
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_exit_status)
    .install()
    ;
}