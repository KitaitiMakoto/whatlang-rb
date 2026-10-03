use rutie::{AnyException, AnyObject, Array, Boolean, Class, Exception, Float, Module, NilClass, Object, RString, VM, class, methods, module, rutie_callback, types::Argc, util::parse_arguments, wrappable_struct};
use whatlang_rs as wl;

module!(Whatlang);

wrappable_struct!(wl::Lang, LangWrapper, LANG_WRAPPER);

class!(Lang);

fn get_lang(rtself: &Lang) -> wl::Lang {
    *rtself.get_data(&*LANG_WRAPPER)
}

impl Lang {
    fn new(lang: wl::Lang) -> AnyObject {
        Module::from_existing("Whatlang").get_nested_class("Lang").wrap_data(lang, &*LANG_WRAPPER)
    }
}

methods!(
    Lang,
    rtself,

    fn wl_lang_all() -> Array {
        let langs = wl::Lang::all();
        let mut ary = Array::with_capacity(langs.len());
        for lang in langs {   
            ary.push(Lang::new(*lang));
        }
        ary
    }

    fn wl_lang_code() -> RString {
        get_lang(&rtself).code().into()
    }

    fn wl_lang_name() -> RString {
        get_lang(&rtself).name().into()
    }

    fn wl_lang_eng_name() -> RString {
        get_lang(&rtself).eng_name().into()
    }
);

wrappable_struct!(wl::Info, InfoWrapper, INFO_WRAPPER);

class!(Info);

impl Info {
    fn new(info: wl::Info) -> AnyObject {
        Module::from_existing("Whatlang").get_nested_class("Info").wrap_data(info, &*INFO_WRAPPER)
    }
}

methods!(
    Info,
    rtself,

    fn wl_info_lang() -> AnyObject {
        Lang::new(rtself.get_data(&*INFO_WRAPPER).lang())
    }

    fn wl_info_script() -> RString {
        rtself.get_data(&*INFO_WRAPPER).script().name().to_owned().into()
    }

    fn wl_info_confidence() -> Float {
        Float::new(rtself.get_data(&*INFO_WRAPPER).confidence())
    }

    fn wl_info_is_reliable() -> Boolean {
        Boolean::new(rtself.get_data(&*INFO_WRAPPER).is_reliable())
    }
);

rutie_callback! {
    fn wl_detect(argc: Argc, argv: *const AnyObject, _rtself: AnyObject) -> AnyObject {
        let arguments = parse_arguments(argc, argv);
        let args = match VM::scan_args(&arguments, "10:") {
            Ok(args) => args,
            Err(error) => VM::raise_message(error.class(), &error.message())
        };

        let Ok(text) = args.required[0].try_convert_to::<RString>() else {
            return NilClass::new().into()
        };

        let keywords = VM::get_kwargs((&args.keywords).into(), &[], &["allowlist", "denylist"], false).map_err(VM::raise_ex).unwrap();

        let info = match (keywords.optional[0].clone(), keywords.optional[1].clone()) {
            (Some(..), Some(..)) => {
                VM::raise_message(Class::argument_error(), "Couldn't specify `allowlist' and `denylist' at a time. Choose one.");
            },
            (Some(allowlist), None) => detect_with_allowlist(text.to_str(), lang_list(allowlist.try_convert_to::<Array>().map_err(VM::raise_ex).unwrap())),
            (None, Some(denylist)) => detect_with_denylist(text.to_str(), lang_list(denylist.try_convert_to::<Array>().map_err(VM::raise_ex).unwrap())),
            (None, None) => detect_without_options(text.to_str())
        };
        option_to_nillable(info)
    }
}

methods!(
    Whatlang,
    _rtself,

    fn wl_detect_without_options(text: RString) -> AnyObject {
        let text = text.map_err(VM::raise_ex).unwrap();
        let info = detect_without_options(text.to_str());
        option_to_nillable(info)
    }

    fn wl_detect_with_allowlist(text: RString, allowlist: Array) -> AnyObject {
        let text = text.map_err(VM::raise_ex).unwrap();
        let allowlist = allowlist.map_err(VM::raise_ex).unwrap();
        let info = detect_with_allowlist(text.to_str(), lang_list(allowlist));
        option_to_nillable(info)
    }

    fn wl_detect_with_denylist(text: RString, denylist: Array) -> AnyObject {
        let text = text.map_err(VM::raise_ex).unwrap();
        let denylist = denylist.map_err(VM::raise_ex).unwrap();
        let info = detect_with_denylist(text.to_str(), lang_list(denylist));
        option_to_nillable(info)
    }

    fn wl_detect_lang(text: RString) -> AnyObject {
        let lang = wl::detect_lang(rstring(text).to_str()).map(Lang::new);
        option_to_nillable(lang)
    }

    fn wl_detect_script(text: RString) -> AnyObject {
        let script = wl::detect_script(rstring(text).to_str())
            .map(|script| RString::new_utf8(script.name()).into());
        option_to_nillable(script)
    }
);

fn detect_without_options(text: &str) -> Option<AnyObject> {
    wl::detect(text).map(Info::new)
}

fn detect_with_allowlist(text: &str, allowlist: Vec<wl::Lang>) -> Option<AnyObject> {
    wl::Detector::with_allowlist(allowlist)
        .detect(text)
        .map(Info::new)
}

fn detect_with_denylist(text: &str, denylist: Vec<wl::Lang>) -> Option<AnyObject> {
    wl::Detector::with_denylist(denylist)
        .detect(text)
        .map(Info::new)
}

fn rstring(s: Result<RString, AnyException>) -> RString {
    s.map_err(VM::raise_ex).unwrap()
}

fn lang_list(list: Array) -> Vec<wl::Lang> {
    list.into_iter().filter_map(|s| wl::Lang::from_code(s.as_string().to_str())).collect()
}

fn option_to_nillable(nillable: Option<AnyObject>) -> AnyObject {
    nillable.unwrap_or_else(|| NilClass::new().into())
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn Init_whatlang() {
    Module::new("Whatlang").define(|whatlang| {
        whatlang.def_self("detect", wl_detect);
        whatlang.def_self("detect_with_allowlist", wl_detect_with_allowlist);
        whatlang.def_self("detect_without_options", wl_detect_without_options);
        whatlang.def_self("detect_with_denylist", wl_detect_with_denylist);
        whatlang.def_self("detect_lang", wl_detect_lang);
        whatlang.def_self("detect_script", wl_detect_script);

        whatlang.define_nested_class("Lang", None).define(|lang| {
            lang.def_self("all", wl_lang_all);
            lang.def("code", wl_lang_code);
            lang.def("name", wl_lang_name);
            lang.def("eng_name", wl_lang_eng_name);
        });

        whatlang.define_nested_class("Info", None).define(|info| {
            info.def("lang", wl_info_lang);
            info.def("script", wl_info_script);
            info.def("confidence", wl_info_confidence);
            info.def("is_reliable", wl_info_is_reliable);
        });
    });
}
