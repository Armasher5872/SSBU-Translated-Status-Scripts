use super::*;

unsafe extern "C" fn mario_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn mario_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
    }
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, *WEAPON_MARIO_PUMP_STATUS_KIND_WAIT, ArticleOperationTarget(0));
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_REMOVE);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, Hash40::new("start"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_FLAG_FIRST);
                }
            }
        }
        if prev_situation_kind == *SITUATION_KIND_AIR {
            if situation_kind == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_FLAG_FIRST);
                }
            }
        }
    }
    else {
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_FLAG_FIRST);
                }
            }
        }
        if prev_situation_kind == *SITUATION_KIND_AIR {
            if situation_kind == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_PUMP_FLAG_FIRST);
                }
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let special_lw_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
        let special_lw_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_charge_frame"));
        if special_lw_charge_frame > special_lw_charge {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE {
        if status_kind != *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT {
            let special_lw_remove_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_remove_frame"));
            ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, *WEAPON_MARIO_PUMP_STATUS_KIND_WAIT, ArticleOperationTarget(0));
            WorkModule::set_int(fighter.module_accessor, special_lw_remove_frame, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_REMOVE);
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_REMOVE_COMMON, Hash40::new("charge_max"));
            sv_module_access::effect(fighter.lua_state_agent);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_special_lw_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_special_lw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_special_lw_end_status)
    .install()
    ;
}