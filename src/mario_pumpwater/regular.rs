use super::*;

unsafe extern "C" fn mario_pumpwater_regular_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mario_pumpwater_regular_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        mario_pumpwater_regular_sub_status(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(mario_pumpwater_regular_sub_status as *const () as _));
    weapon.fastshift(L2CValue::Ptr(mario_pumpwater_regular_main_loop as *const () as _))
}

unsafe extern "C" fn mario_pumpwater_regular_sub_status(weapon: &mut L2CWeaponCommon, should_dec_life: L2CValue) -> L2CValue {
    if should_dec_life.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn mario_pumpwater_regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.clear_lua_stack();
    lua_args!(weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let get_speed_x = smash::app::sv_kinetic_energy::get_speed_x(weapon.lua_state_agent);
    weapon.clear_lua_stack();
    lua_args!(weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let get_speed_y = smash::app::sv_kinetic_energy::get_speed_y(weapon.lua_state_agent);
    let lr = PostureModule::lr(weapon.module_accessor);
    let rot_x = ((get_speed_x*lr)*get_speed_y).atan().to_degrees();
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: rot_x, y: 0.0, z: 0.0}, 0);
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) {
        weapon.change_status(WEAPON_MARIO_PUMP_WATER_STATUS_KIND_CLASH.into(), false.into());
    }
    else {
        if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_MARIO_PUMP_WATER_INSTANCE_WORK_ID_FLAG_CLASH_GROUND);
            weapon.change_status(WEAPON_MARIO_PUMP_WATER_STATUS_KIND_CLASH.into(), false.into());
        }
        else {
            let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
            if life <= 0 {
                weapon.change_status(WEAPON_MARIO_PUMP_WATER_STATUS_KIND_DIE.into(), false.into());
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("mario_pumpwater")
    .status(Pre, *WEAPON_MARIO_PUMP_WATER_STATUS_KIND_REGULAR, mario_pumpwater_regular_pre_status)
    .status(Main, *WEAPON_MARIO_PUMP_WATER_STATUS_KIND_REGULAR, mario_pumpwater_regular_main_status)
    .install()
    ;
}