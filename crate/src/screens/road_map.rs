use crate::config::Config;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::utils::document;
use yew_styles::layouts::{
    container::{Container, Direction, JustifyContent, Mode, Wrap},
    item::{Item, ItemLayout},
};

pub struct RoadMap {
    left: i32,
    top: i32,
    x: i32,
    y: i32,
    element: Option<HtmlElement>,
    is_dragging: bool,
    link: ComponentLink<Self>,
    browser_lang: String,
}

pub enum Msg {
    MouseMoveHandler(MouseEvent),
    MouseUpHandler,
    MouseOverHandler,
    MouseDownHandler(MouseEvent),
    TouchStartHandle(TouchEvent),
    TouchMoveHandle(TouchEvent),
}

impl Component for RoadMap {
    type Properties = ();
    type Message = Msg;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            left: 0,
            top: 0,
            x: 0,
            y: 0,
            element: None,
            is_dragging: false,
            link,
            browser_lang: Config::get_browser_lang(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MouseMoveHandler(event) => {
                if let Some(element) = self.element.clone() {
                    if self.is_dragging {
                        let dx = event.client_x() - self.x;
                        let dy = event.client_y() - self.y;
                        element.set_scroll_top(self.top - dy);
                        element.set_scroll_left(self.left - dx);
                    }
                }
            }
            Msg::MouseOverHandler => {
                if let Some(element) = self.element.clone() {
                    let element_style = element.style();
                    element_style.set_property("cursor", "grab").unwrap();
                    element_style.remove_property("user-select").unwrap();
                }
            }
            Msg::MouseUpHandler => {
                self.is_dragging = false;
                if let Some(element) = self.element.clone() {
                    let element_style = element.style();
                    element_style.set_property("cursor", "grab").unwrap();
                    element_style.remove_property("user-select").unwrap();
                }
            }

            Msg::MouseDownHandler(event) => {
                event.prevent_default();
                if let Some(element) = self.element.clone() {
                    self.is_dragging = true;
                    self.left = element.scroll_left();
                    self.top = element.scroll_top();
                    self.x = event.client_x();
                    self.y = event.client_y();
                    let element_style = element.style();
                    element_style.set_property("cursor", "grabbing").unwrap();
                    element_style.set_property("user-select", "none").unwrap();
                }
            }
            Msg::TouchStartHandle(event) => {
                event.prevent_default();
                if let Some(element) = self.element.clone() {
                    self.is_dragging = true;
                    self.left = element.scroll_left();
                    self.top = element.scroll_top();
                    self.x = event.layer_x();
                    self.y = event.layer_y();
                    let element_style = element.style();
                    element_style.set_property("cursor", "grabbing").unwrap();
                    element_style.set_property("user-select", "none").unwrap();
                }
            }
            Msg::TouchMoveHandle(event) => {
                if let Some(element) = self.element.clone() {
                    if self.is_dragging {
                        let dx = event.layer_x() - self.x;
                        let dy = event.layer_y() - self.y;
                        element.set_scroll_top(self.top - dy);
                        element.set_scroll_left(self.left - dx);
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let scrollgrab_element = document()
                .get_element_by_id("scrollgrab")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();

            self.element = Some(scrollgrab_element.clone());
            self.left = 500;
            self.top = 500;
        }
    }

    fn view(&self) -> Html {
        let roadmap = if self.browser_lang.starts_with("es") {
            "/roadmap_es.svg"
        } else {
            "/roadmap.svg"
        };

        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap justify_content=JustifyContent::Center(Mode::NoMode) id="roadmap">
                <Item layouts=vec![ItemLayout::ItXs(8)]>
                    <div
                        onmouseup=self.link.callback(|_| Msg::MouseUpHandler)
                        onmousemove=self.link.callback(Msg::MouseMoveHandler)
                        ontouchmove=self.link.callback(Msg::TouchMoveHandle)
                        onmousedown=self.link.callback(Msg::MouseDownHandler)
                        ontouchstart=self.link.callback(Msg::TouchStartHandle)
                        onmouseover=self.link.callback(|_| Msg::MouseOverHandler)
                        id="scrollgrab"
                        class="scrollgrab"
                    >
                        <img src=roadmap alt="roadmap"/>
                    </div>
                    <div class="scrollgrab-mobile">
                        <img src=roadmap alt="roadmap"/>
                    </div>
                </Item>
            </Container>
        }
    }
}
