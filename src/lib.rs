use rutie::{Module, Object};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_whatlang() {
    Module::new("Whatlang").define(|_itself| {});
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
