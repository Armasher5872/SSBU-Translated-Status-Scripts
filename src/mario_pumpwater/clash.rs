use super::*;

unsafe extern "C" fn mario_pumpwater_clash_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mario_pumpwater_clash_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_MARIO_PUMP_WATER_INSTANCE_WORK_ID_FLAG_CLASH_GROUND) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("clash"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("clash_air"), 0.0, 1.0, false, 0.0, false, false);
    }
    weapon.fastshift(L2CValue::Ptr(mario_pumpwater_clash_main_loop as *const () as _))
}

unsafe extern "C" fn mario_pumpwater_clash_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let current_frame = weapon.global_table[CURRENT_FRAME].get_f32();
    if current_frame == 5.0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn mario_pumpwater_clash_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
    0.into()
}

pub fn install() {
    Agent::new("mario_pumpwater")
    .status(Pre, *WEAPON_MARIO_PUMP_WATER_STATUS_KIND_CLASH, mario_pumpwater_clash_pre_status)
    .status(Main, *WEAPON_MARIO_PUMP_WATER_STATUS_KIND_CLASH, mario_pumpwater_clash_main_status)
    .status(Exec, *WEAPON_MARIO_PUMP_WATER_STATUS_KIND_CLASH, mario_pumpwater_clash_exec_status)
    .install()
    ;
}