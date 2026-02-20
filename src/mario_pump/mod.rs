use super::*;

mod end;
mod shoot;
mod wait;

pub fn install() {
    end::install();
    shoot::install();
    wait::install();
}