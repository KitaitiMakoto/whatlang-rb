use rutie::{Module, Object};
use whatlang;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_whatlang() {
    Module::new("Whatlang").define(|_itself| {});
}
