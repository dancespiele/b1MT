use crate::config::Config;
use crate::lang::Translations;
use crate::screens::{Buy, Contact, Home, Info, RoadMap, Stake};
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::utils::document;
use yew_styles::{
    carousel::{Carousel, CarouselDot},
    layouts::{
        container::{AlignContent, AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
        item::{AlignSelf, Item, ItemLayout},
    },
    navbar::{
        navbar_component::{Fixed, Navbar},
        navbar_container::NavbarContainer,
        navbar_item::NavbarItem,
    },
    styles::{Palette, Style},
    text::{Text, TextType},
};

pub struct App {
    navbar_items: Vec<bool>,
    link: ComponentLink<Self>,
    lang: Translations,
}

pub enum Msg {
    ChangeNavbarItem(usize),
    ScrollMenu(WheelEvent),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            navbar_items: vec![true, false, false, false, false, false],
            link,
            lang: Config::get_lang(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeNavbarItem(index) => {
                for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                    self.navbar_items[i] = false;
                }

                self.navbar_items[index] = true;
            }
            Msg::ScrollMenu(wheel_event) => {
                let len = self.navbar_items.len();
                let index_opt = self.navbar_items.to_vec().into_iter().position(|ai| ai);
                for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                    self.navbar_items[i] = false;
                }

                if wheel_event.delta_y() < 0.00 {
                    if let Some(index) = index_opt {
                        if index == 0 {
                            self.navbar_items[0] = true
                        } else {
                            self.navbar_items[index - 1] = true
                        }
                    } else {
                        ConsoleService::error("no image active")
                    }
                } else if let Some(index) = index_opt {
                    if index == len - 1 {
                        self.navbar_items[len - 1] = true
                    } else {
                        self.navbar_items[index + 1] = true
                    }
                } else {
                    ConsoleService::error("no image active")
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
            for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                self.navbar_items[i] = false;
            }

            let screen = get_param();
            let screen_index = set_screen_index(&screen);
            self.link.send_message(Msg::ChangeNavbarItem(screen_index));
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="root">
                <Navbar
                    navbar_palette=Palette::Clean
                    navbar_style=Style::Outline
                    fixed=Fixed::Top
                    branch=html!{<img src="./1MTlite2.png"/>}
                >
                    <NavbarContainer>
                        {get_navbar(self.navbar_items.to_vec(), self.lang.clone(), self.link.clone())}
                    </NavbarContainer>
                </Navbar>

                <Carousel class_name="carousel" onwheel_signal= self.link.callback(Msg::ScrollMenu)>

                    <Container direction=Direction::Row wrap=Wrap::Wrap class_name="screen" justify_content=JustifyContent::FlexStart(Mode::NoMode)>
                        <Item layouts=vec!(ItemLayout::ItXs(11)) align_self=AlignSelf::Center class_name="content">
                            {get_content(get_position(self.navbar_items.to_vec()))}
                        </Item>

                        <Item layouts=vec!(ItemLayout::ItXs(1)) align_self=AlignSelf::Center class_name="content">
                            <Container
                                direction=Direction::Column wrap=Wrap::Wrap
                                justify_content=JustifyContent::FlexEnd(Mode::NoMode)
                                align_items=AlignItems::FlexEnd(Mode::NoMode)
                                align_content=AlignContent::FlexEnd(Mode::NoMode)
                                class_name="dots">
                                {get_dots(self.navbar_items.to_vec(), self.link.clone())}
                            </Container>
                        </Item>
                    </Container>
                </Carousel>
            </div>
        }
    }
}

fn get_navbar(items: Vec<bool>, lang: Translations, link: ComponentLink<App>) -> Html {
    let menus = vec![
        lang.home,
        lang.tokenomics,
        lang.buy,
        lang.stake,
        lang.road_map,
        lang.community,
    ];

    let mut navbar_items = vec![];

    for (i, _) in items.clone().into_iter().enumerate() {
        navbar_items.push(html! {
            <NavbarItem
                active = items[i]
                onclick_signal = link.callback(move |_| Msg::ChangeNavbarItem(i))
                >
                {get_text(menus[i].as_str())}
            </NavbarItem>
        })
    }

    navbar_items.into_iter().collect::<Html>()
}

fn get_text(text: &str) -> Html {
    html! {
        <Text
        text_type=TextType::Plain
            plain_text=text.to_string()
            html_text=None
        />
    }
}

fn get_param() -> String {
    let url = document().location().unwrap().pathname().unwrap();

    let url = url.replace("/", "");

    url
}

fn get_content(index: usize) -> Html {
    match index {
        0 => html! {<Home/>},
        1 => html! {<Info/>},
        2 => html! {<Buy/>},
        3 => html! {<Stake/>},
        4 => html! {<RoadMap/>},
        5 => html! {<Contact/>},
        _ => html! {<Home/>},
    }
}

fn set_screen_index(screen: &str) -> usize {
    match screen {
        "info" => 1,
        "buy" => 2,
        "stake" => 3,
        "roadmap" => 4,
        "community" => 5,
        &_ => 0,
    }
}

fn get_position(items: Vec<bool>) -> usize {
    items.into_iter().position(|item| item).unwrap_or(0)
}

fn get_dots(items: Vec<bool>, link: ComponentLink<App>) -> Html {
    let mut dot = vec![];

    for (i, _) in items.clone().into_iter().enumerate() {
        dot.push(html! {
            <CarouselDot active=items[i] onclick_signal = link.callback(move |_| Msg::ChangeNavbarItem(i))/>
        })
    }

    dot.into_iter().collect::<Html>()
}
