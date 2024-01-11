use rutie::{
    methods, module, AnyException, AnyObject, Array, Boolean, Float, Module, Object,
    RString, VM,
};
use whatlang::{detect, detect_lang, detect_script, Detector, Info, Lang};

module!(Whatlang);

methods!(
    Whatlang,
    _rtself,

    fn wl_detect_without_options(text: RString) -> AnyObject {
        detect(rstring(text).to_str()).map_or(no_info(), rinfo)
    }

    fn wl_detect_with_allowlist(text: RString, list: Array) -> AnyObject {
        let detector = Detector::with_allowlist(rarray_to_lang_list(list));
        detector.detect(rstring(text).to_str()).map_or(no_info(), rinfo)
    }

    fn wl_detect_with_denylist(text: RString, list: Array) -> AnyObject {
        let detector = Detector::with_denylist(rarray_to_lang_list(list));
        detector.detect(rstring(text).to_str()).map_or(no_info(), rinfo)
    }

    fn wl_detect_lang(text: RString) -> AnyObject {
        detect_lang(rstring(text).to_str()).map_or(no_info(), rlang)
    }

    fn wl_detect_script(text: RString) -> AnyObject {
        detect_script(rstring(text).to_str()).map_or(no_info(), |script| {
            RString::new_utf8(script.name()).into()
        })
    }
);

fn rstring(rstring: Result<RString, AnyException>) -> RString {
    rstring.map_err(VM::raise_ex).unwrap()
}

fn rarray_to_lang_list(rarray: Result<Array, AnyException>) -> Vec<Lang> {
    rarray
        .map_err(VM::raise_ex)
        .unwrap()
        .into_iter()
        .filter_map(|rcode| {
            let code = rcode
                .try_convert_to::<RString>()
                .map_err(VM::raise_ex)
                .unwrap();
            Lang::from_code(code.to_str())
        })
        .collect()
}

fn rinfo(info: Info) -> AnyObject {
    Module::from_existing("Whatlang")
        .get_nested_class("Info")
        .new_instance(&[
            rlang(info.lang()),
            RString::new_utf8(info.script().name()).into(),
            Boolean::new(info.is_reliable()).into(),
            Float::new(info.confidence()).into(),
        ])
}

fn rlang(lang: Lang) -> AnyObject {
    Module::from_existing("Whatlang")
        .get_nested_class("Lang")
        .new_instance(&[
            RString::new_utf8(lang.code()).into(),
            RString::new_utf8(lang.name()).into(),
            RString::new_utf8(lang.eng_name()).into(),
        ])
}

fn no_info() -> AnyObject {
    Module::from_existing("Whatlang")
        .const_get("NO_INFO")
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_whatlang() {
    Module::new("Whatlang").define(|itself| {
        itself.def_self("detect_without_options", wl_detect_without_options);
        itself.def_self("detect_with_allowlist", wl_detect_with_allowlist);
        itself.def_self("detect_with_denylist", wl_detect_with_denylist);
        itself.def_self("detect_lang", wl_detect_lang);
        itself.def_self("detect_script", wl_detect_script);
    });
}
