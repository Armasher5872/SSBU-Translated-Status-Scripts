//Related to CheckAttack
pub unsafe fn get_table_value(table: *mut smash2::lib::L2CTable, key: &str) -> smash2::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash2::phx::Hash40::from_hex_str(key).unwrap()
    } 
    else {
        smash2::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}