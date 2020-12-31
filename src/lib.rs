use rutie::{
    methods, module, AnyException, AnyObject, Array, Boolean, Float, Module, NilClass, Object,
    RString, VM,
};
use whatlang::{detect, detect_lang, detect_script, detect_with_options, Info, Lang, Options};

module!(Whatlang);

methods!(
    Whatlang,
    _rtself,

    fn wl_detect_without_options(text: RString) -> AnyObject {
        detect(rstring(text).to_str()).map_or(NilClass::new().into(), rinfo)
    }

    fn wl_detect_with_whitelist(text: RString, list: Array) -> AnyObject {
        let rlist = list.map_err(VM::raise_ex).unwrap();
        let list = rlist
            .into_iter()
            .map(|rcode| {
                let code = rcode
                    .try_convert_to::<RString>()
                    .map_err(VM::raise_ex)
                    .unwrap();
                Lang::from_code(code.to_str())
            })
            .filter(|lang_res| lang_res.is_some())
            .map(|lang_res| lang_res.unwrap())
            .collect::<Vec<Lang>>();
        let options = Options::new().set_whitelist(list);

        detect_with_options(rstring(text).to_str(), &options).map_or(NilClass::new().into(), rinfo)
    }

    fn wl_detect_with_blacklist(text: RString, list: Array) -> AnyObject {
        let rlist = list.map_err(VM::raise_ex).unwrap();
        let list = rlist
            .into_iter()
            .map(|rcode| {
                let code = rcode
                    .try_convert_to::<RString>()
                    .map_err(VM::raise_ex)
                    .unwrap();
                Lang::from_code(code.to_str())
            })
            .filter(|lang_res| lang_res.is_some())
            .map(|lang_res| lang_res.unwrap())
            .collect::<Vec<Lang>>();
        let options = Options::new().set_blacklist(list);

        detect_with_options(rstring(text).to_str(), &options).map_or(NilClass::new().into(), rinfo)
    }

    fn wl_detect_lang(text: RString) -> AnyObject {
        detect_lang(rstring(text).to_str()).map_or(NilClass::new().into(), rlang)
    }

    fn wl_detect_script(text: RString) -> AnyObject {
        detect_script(rstring(text).to_str()).map_or(NilClass::new().into(), |script| {
            RString::new_utf8(script.name()).into()
        })
    }
);

fn rstring(rstring: Result<RString, AnyException>) -> RString {
    rstring.map_err(VM::raise_ex).unwrap()
}

fn rinfo(info: Info) -> AnyObject {
    Module::from_existing("Whatlang")
        .get_nested_class("Info")
        .new_instance(&[
            rlang(info.lang()).into(),
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_whatlang() {
    Module::new("Whatlang").define(|itself| {
        itself.def_self("detect_without_options", wl_detect_without_options);
        itself.def_self("detect_with_whitelist", wl_detect_with_whitelist);
        itself.def_self("detect_with_blacklist", wl_detect_with_blacklist);
        itself.def_self("detect_lang", wl_detect_lang);
        itself.def_self("detect_script", wl_detect_script);
    });
}
