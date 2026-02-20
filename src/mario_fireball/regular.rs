use super::*;

unsafe extern "C" fn mario_fireball_regular_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mario_fireball_regular_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(mario_fireball_regular_main_loop as *const () as _))
}

unsafe extern "C" fn mario_fireball_regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_EFFECT, Hash40::new("effect_bound"), -1);
    }
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn mario_fireball_regular_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

pub fn install() {
    Agent::new("mario_fireball")
    .status(Pre, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, mario_fireball_regular_pre_status)
    .status(Main, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, mario_fireball_regular_main_status)
    .status(Exec, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, mario_fireball_regular_exec_status)
    .install()
    ;
}