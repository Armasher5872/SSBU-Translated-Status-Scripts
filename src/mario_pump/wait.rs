use super::*;

unsafe extern "C" fn mario_pump_wait_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let parent_lr = LinkModule::get_parent_lr(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    PostureModule::set_lr(weapon.module_accessor, parent_lr);
    0.into()
}

unsafe extern "C" fn mario_pump_wait_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(mario_pump_wait_main_loop as *const () as _))
}

unsafe extern "C" fn mario_pump_wait_main_loop(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("mario_pump")
    .status(Pre, *WEAPON_MARIO_PUMP_STATUS_KIND_WAIT, mario_pump_wait_pre_status)
    .status(Main, *WEAPON_MARIO_PUMP_STATUS_KIND_WAIT, mario_pump_wait_main_status)
    .install()
    ;
}