use crate::lang::{Lang, Translations};
use web_sys::window;
use wasm_bindgen::{JsCast, JsValue};
use js_sys::{global, Array, Object};

pub struct Config {
}

impl Config {
    fn get_global(global_name: &str) -> JsValue {
        let entries = Object::entries(&global());
        let global_context = entries.find(&mut |entry, _index, _array| {
            let entry_field = entry.unchecked_into::<Array>();
            let key = entry_field.get(0);
            key == global_name
        });

        let global_field = global_context.unchecked_into::<Array>();
        global_field.get(1)
    }

    fn get_browser_lang() -> String {
        if let Some(language) = window().unwrap().navigator().language() {
            language
        } else {
            "en".to_string()
        }
    }

    pub fn get_lang() -> Translations {
        let lang: Lang = Config::get_global("lang").into_serde().unwrap();

        let browser_lang = Config::get_browser_lang();

        if browser_lang.starts_with("es") {
            lang.es
        } else {
            lang.en
        }
    }
}
