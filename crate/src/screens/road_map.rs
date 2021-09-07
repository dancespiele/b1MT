use web_sys::Element;
use yew::prelude::*;
use yew::services::ConsoleService;
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
    element: Option<Element>,
    is_dragging: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    MouseMoveHandler(MouseEvent),
    MouseUpHandler,
    MouseDownHandler(MouseEvent),
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
            Msg::MouseUpHandler => {
                self.is_dragging = false;
            }
            Msg::MouseDownHandler(event) => {
                event.prevent_default();
                if let Some(element) = self.element.clone() {
                    self.is_dragging = true;
                    self.left = element.scroll_left();
                    self.top = element.scroll_top();
                    self.x = event.client_x();
                    self.y = event.client_y();
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
            let scrollgrab_element = document().get_element_by_id("scrollgrab").unwrap();

            self.element = Some(scrollgrab_element.clone());
            self.left = 500;
            self.top = 500;
        }
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap justify_content=JustifyContent::Center(Mode::NoMode) id="roadmap">
                <Item layouts=vec![ItemLayout::ItXs(8)]>
                    <div
                        onmouseup=self.link.callback(|_| Msg::MouseUpHandler)
                        onmousemove=self.link.callback(Msg::MouseMoveHandler)
                        onmousedown=self.link.callback(Msg::MouseDownHandler)
                        id="scrollgrab"
                        class="scrollgrab"
                >
                        <img src="/roadmap.svg" alt="roadmap"/>
                    </div>
                </Item>
            </Container>
        }
    }
}
