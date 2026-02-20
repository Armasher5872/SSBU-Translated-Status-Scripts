use super::*;

unsafe extern "C" fn mario_final_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_FinalCommon();
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32, 0);
    0.into()
}

unsafe extern "C" fn mario_final_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));
    AreaModule::set_whole(fighter.module_accessor, false);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    fun_71000073a0(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_final_main_loop as *const () as _))
}

unsafe extern "C" fn fun_71000073a0(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_FINAL_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("final_air"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_air"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_FINAL_FLAG_FIRST);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_FINAL_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("final"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_FINAL_FLAG_FIRST);
        }
    }
}

unsafe extern "C" fn mario_final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if (situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR)
        || (situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND) {
            fun_71000073a0(fighter);
        }
    }
    else {
        if (situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR)
        || (situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND) {
            fun_71000073a0(fighter);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            if situation_kind == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_STATUS_KIND_FINAL_JUMP_END.into(), false.into());
            }
        }
    }
    0.into()
}

unsafe extern "C" fn mario_final_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_HUGE_FLAME, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d6));
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .status(Pre, *FIGHTER_STATUS_KIND_FINAL, mario_final_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_FINAL, mario_final_main_status)
    .status(End, *FIGHTER_STATUS_KIND_FINAL, mario_final_end_status)
    .install()
    ;
}