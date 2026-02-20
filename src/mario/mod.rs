use super::*;

mod appeal;
mod final_smash;
mod special_hi;
mod special_lw_charge;
mod special_lw_shoot;
mod special_lw;
mod special_n;
mod special_s;

pub fn install() {
    appeal::install();
    final_smash::install();
    special_hi::install();
    special_lw_charge::install();
    special_lw_shoot::install();
    special_lw::install();
    special_n::install();
    special_s::install();
}