use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::utils::document;

pub enum ScrollStyle {
    ScrollUp,
    ScrollDown,
}

pub enum ScrollbarState {
    Auto,
    Hidden,
}

pub fn set_scroll_style(scroll_style: ScrollStyle, id: &str) {
    let screen_element = document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    let screen_style = screen_element.class_name();

    match scroll_style {
        ScrollStyle::ScrollUp => {
            screen_element.set_class_name(&format!("{} scroll-up", screen_style));
        }
        ScrollStyle::ScrollDown => {
            screen_element.set_class_name(&format!("{} scroll-down", screen_style));
        }
    };
}

pub fn set_scrollbar_state(scrollbar_state: ScrollbarState) {
    let body_style = document()
        .body()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .style();

    match scrollbar_state {
        ScrollbarState::Auto => body_style.set_property("overflow", "auto").unwrap(),
        ScrollbarState::Hidden => body_style.set_property("overflow", "hidden").unwrap(),
    };
}
