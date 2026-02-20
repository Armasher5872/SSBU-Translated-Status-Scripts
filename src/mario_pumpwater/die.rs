use super::*;

unsafe extern "C" fn mario_pumpwater_die_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mario_pumpwater_die_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("die"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(mario_pumpwater_die_main_loop as *const () as _))
}

unsafe extern "C" fn mario_pumpwater_die_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let current_frame = weapon.global_table[CURRENT_FRAME].get_f32();
    if current_frame == 5.0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub fn install() {
    Agent::new("mario_pumpwater")
    .status(Pre, *WEAPON_MARIO_PUMP_WATER_STATUS_KIND_DIE, mario_pumpwater_die_pre_status)
    .status(Main, *WEAPON_MARIO_PUMP_WATER_STATUS_KIND_DIE, mario_pumpwater_die_main_status)
    .install()
    ;
}