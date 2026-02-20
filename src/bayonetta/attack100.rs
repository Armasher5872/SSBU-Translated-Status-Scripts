use super::*;

unsafe extern "C" fn bayonetta_attack_100_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
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
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_100, bayonetta_attack_100_check_attack_status)
    .install()
    ;
}