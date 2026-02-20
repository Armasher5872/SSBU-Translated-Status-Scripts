use super::*;

unsafe extern "C" fn bayonetta_attach_wall_wait_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, true);
    fighter.status_attach_wall_wait();
    0.into()
}

pub fn install() {
    Agent::new("bayonetta")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACH_WALL, bayonetta_attach_wall_wait_main_status)
    .install()
    ;
}