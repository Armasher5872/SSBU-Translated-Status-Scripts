use super::*;

unsafe extern "C" fn bayonetta_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_combo_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_type"), 0);
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    if attack_combo_type != *FIGHTER_COMBO_TYPE_NONE {
        if attack_combo_type != *FIGHTER_COMBO_TYPE_HIT {
            if attack_combo_type == *FIGHTER_COMBO_TYPE_SUCCEED {
                if !StopModule::is_stop(fighter.module_accessor) {
                    fighter.attack_combo_uniq_chk(false.into());
                }
                fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_attack_combo_uniq_chk as *const () as _));
            }
        }
        else {
            if !StopModule::is_stop(fighter.module_accessor) {
                fun_7100011a70(fighter, false.into());
            }
            fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_7100011a70 as *const () as _));
        }
    }
    else {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
            }
        }
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.attack_combo_none_uniq_chk(false.into());
        }
        fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attack_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100011a70(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let combo_count = ComboModule::count(fighter.module_accessor);
    let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if bool_check.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK)
        || (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) && WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0)) {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    else {
        fighter.attack_uniq_chk();
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_COMBO) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                if combo_count as i32 == attack_combo_max {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
                    ComboModule::reset(fighter.module_accessor);
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO) {
                        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER) {
                            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
                        }
                    }
                }
            }
        }
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_stop = fighter.global_table[IS_STOP].get_bool();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let attack_100_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let reserve_log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let attack_combo_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_type"), 0);
    let attack100_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack100_type"), 0);
    let attack_100_enable_cnt = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_100_enable_cnt"), 0);
    let data = fighter.get_mini_jump_attack_data_cancel_function(motion_kind.into());
    if motion_kind == hash40("attack_13") {
        fighter.check_100_count();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if attack100_type != *FIGHTER_ATTACK100_TYPE_NONE {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100) {
                if attack_100_enable_cnt <= attack_100_count {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_100.into(), false.into());
                        return 1.into();
                    }
                }
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if 0 < mini_jump_attack_frame {
        if fighter.sub_check_button_jump().get_bool() {
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(data.get_u64()), -1);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if mini_jump_attack_frame == 1 {
        if !is_stop {
            if 0 < reserve_log_attack_kind {
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            }
        }
    }
    if attack_combo_type != *FIGHTER_COMBO_TYPE_NONE {
        if attack_combo_type == *FIGHTER_COMBO_TYPE_HIT {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                return 0.into();
            }
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let no_self = get_table_value(table, "no_self_").try_integer().unwrap() as i32;
    let absolute = get_table_value(table, "absolute_").try_bool().unwrap() as bool;
    let attack_data = AttackModule::attack_data(fighter.module_accessor, no_self, absolute);
    AttackData::store_l2c_table(attack_data);
    let collision_attr = get_table_value(table, "attr_").try_integer().unwrap() as u64;
    if collision_attr != hash40("collision_attr_normal_bullet") {
        fighter.FighterStatusUniqProcessAttack_check_attack(no_self.into(), absolute.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("bayonetta")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK, bayonetta_attack_main_status)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK, bayonetta_attack_check_attack_status)
    .install()
    ;
}