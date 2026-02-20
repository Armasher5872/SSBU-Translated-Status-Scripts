use super::*;

unsafe extern "C" fn mario_pump_shoot_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let parent_lr = LinkModule::get_parent_lr(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    PostureModule::set_lr(weapon.module_accessor, parent_lr);
    0.into()
}

unsafe extern "C" fn mario_pump_shoot_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pump"), hash40("angle"));
    WorkModule::set_float(weapon.module_accessor, angle, *WEAPON_MARIO_PUMP_STATUS_WORK_FLOAT_ANGLE);
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("control"), &Vector3f{x: -angle, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_NONE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SHOOT_NUM);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SPAN_FRAME);
    if !StopModule::is_stop(weapon.module_accessor) {
        fun_710000eaa0(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710000eaa0 as *const () as _));
    weapon.fastshift(L2CValue::Ptr(mario_pump_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn fun_710000eaa0(weapon: &mut L2CWeaponCommon, bool_check: L2CValue) -> L2CValue {
    let water_span_frame = WorkModule::get_int(weapon.module_accessor, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SPAN_FRAME);
    let water_shoot_num = WorkModule::get_int(weapon.module_accessor, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SHOOT_NUM);
    let pump_shoot_num = WorkModule::get_param_int(weapon.module_accessor, hash40("param_pump"), hash40("shoot_num"));
    let pump_span_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_pump"), hash40("span_frame"));
    if bool_check.get_bool() {
        if water_shoot_num < pump_shoot_num {
            WorkModule::dec_int(weapon.module_accessor, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SPAN_FRAME);
            if water_span_frame <= 0 {
                ArticleModule::generate_article(weapon.module_accessor, *WEAPON_MARIO_PUMP_GENERATE_ARTICLE_WATER, false, -1);
                WorkModule::set_int(weapon.module_accessor, pump_span_frame, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SPAN_FRAME);
                WorkModule::inc_int(weapon.module_accessor, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SHOOT_NUM);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn mario_pump_shoot_main_loop(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn mario_pump_shoot_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let get_stick_y = ControlModule::get_stick_y(weapon.module_accessor);
    let pump_angle = WorkModule::get_float(weapon.module_accessor, *WEAPON_MARIO_PUMP_STATUS_WORK_FLOAT_ANGLE);
    let angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pump"), hash40("angle"));
    let angle_down = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pump"), hash40("angle_down"));
    let angle_up = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pump"), hash40("angle_up"));
    let angle_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pump"), hash40("angle_speed"));
    let mut new_angle;
    if 0.0 > angle {
        new_angle = angle+((-angle_down-angle)*(-get_stick_y));
    }
    else {
        new_angle = angle+((angle_up-angle)*get_stick_y);
    }
    let mut new_angle_speed = new_angle-pump_angle;
    if angle_speed >= new_angle_speed {
        if new_angle_speed < -angle_speed {
            new_angle_speed = -angle_speed;
        }
    }
    else {
        new_angle_speed = angle_speed;
    }
    new_angle = pump_angle+new_angle_speed;
    WorkModule::set_float(weapon.module_accessor, new_angle, *WEAPON_MARIO_PUMP_STATUS_WORK_FLOAT_ANGLE);
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("control"), &Vector3f{x: 0.0, y: 0.0, z: -new_angle}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_NONE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    0.into()
}

unsafe extern "C" fn mario_pump_shoot_exec_stop_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let float_angle = WorkModule::get_float(weapon.module_accessor, *WEAPON_MARIO_PUMP_STATUS_WORK_FLOAT_ANGLE);
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("control"), &Vector3f{x: 0.0, y: 0.0, z: -float_angle}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_NONE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    0.into()
}

pub fn install() {
    Agent::new("mario_pump")
    .status(Pre, *WEAPON_MARIO_PUMP_STATUS_KIND_SHOOT, mario_pump_shoot_pre_status)
    .status(Main, *WEAPON_MARIO_PUMP_STATUS_KIND_SHOOT, mario_pump_shoot_main_status)
    .status(Exec, *WEAPON_MARIO_PUMP_STATUS_KIND_SHOOT, mario_pump_shoot_exec_status)
    .status(ExecStop, *WEAPON_MARIO_PUMP_STATUS_KIND_SHOOT, mario_pump_shoot_exec_stop_status)
    .install()
    ;
}