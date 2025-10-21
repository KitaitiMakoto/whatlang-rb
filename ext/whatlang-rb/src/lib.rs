use magnus::{
    Error, Ruby, Value, function, method,
    prelude::*,
    scan_args::{get_kwargs, scan_args},
};
use whatlang as wl;

#[magnus::wrap(class = "Whatlang::Lang")]
struct Lang(wl::Lang);

impl Lang {
    fn code(&self) -> &str {
        self.0.code()
    }

    fn name(&self) -> &str {
        self.0.name()
    }

    fn eng_name(&self) -> &str {
        self.0.eng_name()
    }
}

#[magnus::wrap(class = "Whatlang::Info")]
struct Info(wl::Info);

impl Info {
    fn lang(&self) -> Lang {
        Lang(self.0.lang())
    }

    fn script(&self) -> String {
        self.0.script().to_string()
    }

    fn confidence(&self) -> f64 {
        self.0.confidence()
    }

    fn is_reliable(&self) -> bool {
        self.0.is_reliable()
    }
}

fn detect(ruby: &Ruby, args: &[Value]) -> Result<Option<Info>, Error> {
    type LangList = Option<Vec<String>>;

    let args = scan_args::<(Value,), (), (), (), _, ()>(args)?;
    let kw_args = get_kwargs::<_, (), (LangList, LangList), ()>(
        args.keywords,
        &[],
        &["allowlist", "denylist"],
    )?;
    let (allowlist, denylist) = kw_args.optional;
    if allowlist.is_some() && denylist.is_some() {
        return Err(Error::new(
            ruby.exception_arg_error(),
            "Couldn't specify `allowlist' and `denylist' at a time. Choose one.",
        ));
    }

    let text = args.required.0.to_r_string()?.to_string()?;
    Ok(if let Some(allowlist) = allowlist {
        detect_with_allowlist(text, allowlist)
    } else if let Some(denylist) = denylist {
        detect_with_denylist(text, denylist)
    } else {
        detect_without_options(text)
    })
}

fn detect_without_options(text: String) -> Option<Info> {
    wl::detect(&text).map(Info)
}

fn detect_with_allowlist(text: String, allowlist: Vec<String>) -> Option<Info> {
    wl::Detector::with_allowlist(allowlist.iter().filter_map(wl::Lang::from_code).collect())
        .detect(&text)
        .map(Info)
}

fn detect_with_denylist(text: String, denylist: Vec<String>) -> Option<Info> {
    wl::Detector::with_denylist(denylist.iter().filter_map(wl::Lang::from_code).collect())
        .detect(&text)
        .map(Info)
}

fn detect_lang(text: String) -> Option<Lang> {
    wl::detect_lang(&text).map(Lang)
}

fn detect_script(text: String) -> Option<String> {
    wl::detect_script(&text).map(|script| script.to_string())
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Whatlang")?;
    module.define_singleton_method(
        "detect_without_options",
        function!(detect_without_options, 1),
    )?;
    module.define_singleton_method("detect", function!(detect, -1))?;
    module.define_singleton_method("detect_with_allowlist", function!(detect_with_allowlist, 2))?;
    module.define_singleton_method("detect_with_denylist", function!(detect_with_denylist, 2))?;
    module.define_singleton_method("detect_lang", function!(detect_lang, 1))?;
    module.define_singleton_method("detect_script", function!(detect_script, 1))?;

    let lang = module.define_class("Lang", ruby.class_object())?;
    lang.define_method("code", method!(Lang::code, 0))?;
    lang.define_method("name", method!(Lang::name, 0))?;
    lang.define_method("eng_name", method!(Lang::eng_name, 0))?;

    let info = module.define_class("Info", ruby.class_object())?;
    info.define_method("lang", method!(Info::lang, 0))?;
    info.define_method("script", method!(Info::script, 0))?;
    info.define_method("confidence", method!(Info::confidence, 0))?;
    info.define_method("reliable?", method!(Info::is_reliable, 0))?;

    Ok(())
}
