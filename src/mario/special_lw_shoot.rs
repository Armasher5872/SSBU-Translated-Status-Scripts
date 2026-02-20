use super::*;

unsafe extern "C" fn mario_special_lw_shoot_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn mario_special_lw_shoot_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_lw_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    let special_lw_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_charge_frame"));
    let special_lw_speed_add = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_speed_add"));
    let charge = special_lw_charge/special_lw_charge_frame;
    let pump_article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP);
    let pump_battle_object_id = Article::get_battle_object_id(pump_article);
    let pump_boma = sv_battle_object::module_accessor(pump_battle_object_id as u32);
    let new_speed = (special_lw_charge as f32)*special_lw_speed_add;
    weapon_specializer_mario_pump_set_charge(pump_boma, charge as f32);
    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: -new_speed, y: 0.0, z: 0.0});
    0.into()
}

unsafe extern "C" fn mario_special_lw_shoot_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_lw_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    let special_lw_heavy_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_heavy_charge_frame"));
    if special_lw_charge >= special_lw_heavy_charge_frame {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_heavy") as i64, *FIGHTER_MARIO_STATUS_PUMP_SHOOT_WORK_INT_MOTION_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_heavy") as i64, *FIGHTER_MARIO_STATUS_PUMP_SHOOT_WORK_INT_MOTION_KIND_AIR);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("heavy"), false, -1.0);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_light") as i64, *FIGHTER_MARIO_STATUS_PUMP_SHOOT_WORK_INT_MOTION_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_light") as i64, *FIGHTER_MARIO_STATUS_PUMP_SHOOT_WORK_INT_MOTION_KIND_AIR);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("light"), false, -1.0);
    }
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, *WEAPON_MARIO_PUMP_STATUS_KIND_SHOOT, ArticleOperationTarget(0));
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_lw_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 0.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_AIR {
                mario_special_lw_change_motion(fighter);
            }
        }
        else {
            if situation_kind == *SITUATION_KIND_GROUND {
                mario_special_lw_change_motion(fighter);
            }
        }
    }
    else {
        mario_special_lw_change_motion(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_change_motion(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let motion_kind_ground = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_SHOOT_WORK_INT_MOTION_KIND);
    let motion_kind_air = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_SHOOT_WORK_INT_MOTION_KIND_AIR);
    if situation_kind != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_SHOOT_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion_kind_air as u64), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind_air as u64), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_SHOOT_FLAG_FIRST);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_SHOOT_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion_kind_ground as u64), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind_ground as u64), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_SHOOT_FLAG_FIRST);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    }
}

unsafe extern "C" fn mario_special_lw_shoot_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_lw_remove_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_remove_frame"));
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    WorkModule::set_int(fighter.module_accessor, special_lw_remove_frame, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_REMOVE);
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, *WEAPON_MARIO_PUMP_STATUS_KIND_WAIT, ArticleOperationTarget(0));
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .status(Pre, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_pre_status)
    .status(Init, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_init_status)
    .status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_main_status)
    .status(End, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_end_status)
    .install()
    ;
}