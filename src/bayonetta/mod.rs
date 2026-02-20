use super::*;

mod attachwallwait;
mod attack;
mod attack100;

pub fn install() {
    attachwallwait::install();
    attack::install();
    attack100::install();
}