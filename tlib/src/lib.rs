#![warn(bare_trait_objects)]

#[no_mangle]
#[export_name="ConfigureW"]
pub extern "system" fn configure() -> usize {
    0
}
