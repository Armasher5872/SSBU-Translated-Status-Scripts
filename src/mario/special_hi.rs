use super::*;

unsafe extern "C" fn mario_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn mario_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_hi_lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_lr_stick_x"));
    let special_hi_dir_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_dir_stick_x"));
    let special_hi_dir_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_dir_mul"));
    let special_hi_pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_pass_mul"));
    let special_air_hi_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_air_hi_accel_y"));
    let special_air_hi_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_air_hi_start_x_mul"));
    let special_air_hi_pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_air_hi_pass_mul"));
    let special_hi_fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_x_mul"));
    let special_hi_trans_move_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_trans_move_end_speed_x_mul"));
    let special_hi_trans_move_end_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_trans_move_end_speed_y_mul"));
    let cappy_prob = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("cappy_prob"));
    let special_hi_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_landing_frame"));
    let fighter_rand = sv_math::randf(hash40("fighter"), 1.0);
    let cappy_chance = cappy_prob/100.0;
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
    WorkModule::set_float(fighter.module_accessor, special_hi_lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, special_hi_dir_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, special_hi_dir_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
    WorkModule::set_float(fighter.module_accessor, special_hi_pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, special_air_hi_accel_y, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    WorkModule::set_float(fighter.module_accessor, special_air_hi_start_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(fighter.module_accessor, special_air_hi_pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, special_hi_fall_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_float(fighter.module_accessor, special_hi_trans_move_end_speed_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_X_MUL);
    WorkModule::set_float(fighter.module_accessor, special_hi_trans_move_end_speed_y_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_Y_MUL);
    WorkModule::set_int(fighter.module_accessor, special_hi_landing_frame, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_NO_SET_CLIFF_CHECK);
    if fighter_rand < cappy_chance {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, false, -1);
    }
    fighter.super_jump_punch(L2CValue::Void());
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.super_jump_punch_main();
    0.into()
}

unsafe extern "C" fn mario_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.super_jump_punch_end(L2CValue::Ptr(L2CFighterCommon_super_jump_punch_reset_common_condition as *const () as _));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, ArticleOperationTarget(0));
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, mario_special_hi_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, mario_special_hi_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, mario_special_hi_end_status)
    .install()
    ;
}