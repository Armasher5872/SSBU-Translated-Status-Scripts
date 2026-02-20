#![feature(proc_macro_hygiene)]

use {
    attack_func::*,
    extern_func::*,
    globals::*,
    smash::{
        app::{
            ArticleOperationTarget,
            BattleObjectModuleAccessor,
            FighterUtil,
            GroundCliffCheckKind,
            GroundCorrectKind,
            lua_bind::*,
            MotionNodeRotateCompose,
            MotionNodeRotateOrder,
            SituationKind,
            sv_battle_object,
            sv_math,
            sv_module_access
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

mod attack_func;
mod bayonetta;
mod extern_func;
mod globals;
mod mario;
mod mario_fireball;
mod mario_hugeflame;
mod mario_pump;
mod mario_pumpwater;

#[skyline::main(name = "ssbu_translated_status_scripts")]
pub fn main() {
    bayonetta::install();
    mario::install();
    mario_fireball::install();
    mario_hugeflame::install();
    mario_pump::install();
    mario_pumpwater::install();
}