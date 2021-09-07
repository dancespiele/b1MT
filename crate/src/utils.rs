use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::utils::document;

pub enum ScrollStyle {
    ScrollUp,
    ScrollDown,
}

pub fn set_scroll_style(scroll_style: ScrollStyle, id: &str, style: &str) {
    let screen_element = document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    let screen_style = screen_element.class_name();

    match scroll_style {
        ScrollStyle::ScrollUp => {
            screen_element.set_class_name(&format!("{} {}-up", screen_style, style));
        }
        ScrollStyle::ScrollDown => {
            screen_element.set_class_name(&format!("{} {}-down", screen_style, style));
        }
    };
}
