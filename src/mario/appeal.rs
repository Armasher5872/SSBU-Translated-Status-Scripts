use super::*;

unsafe extern "C" fn mario_appeal_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, ArticleOperationTarget(0));
    EffectModule::remove_post_effect_line(fighter.module_accessor, 0x1E, true);
    fighter.status_end_Appeal();
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .status(End, *FIGHTER_STATUS_KIND_APPEAL, mario_appeal_end_status)
    .install()
    ;
}