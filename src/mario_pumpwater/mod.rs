use super::*;

mod clash;
mod die;
mod regular;

pub fn install() {
    clash::install();
    die::install();
    regular::install();
}