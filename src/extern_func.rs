use super::*;

extern "C" {
    #[link_name = "\u{1}_ZN3app32WeaponSpecializer_MarioHugeFlame14request_effectERNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fE"]
	pub fn weapon_specializer_mario_hugeflame_request_effect(boma: *mut smash::app::BattleObjectModuleAccessor, bone: Hash40, pos: &Vector3f) -> Vector2f;
    #[link_name = "\u{1}_ZN3app27WeaponSpecializer_MarioPump10set_chargeERNS_26BattleObjectModuleAccessorEf"]
	pub fn weapon_specializer_mario_pump_set_charge(boma: *mut smash::app::BattleObjectModuleAccessor, charge: f32);
}