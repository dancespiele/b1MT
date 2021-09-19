use crate::config::Config;
use crate::lang::Translations;
use crate::screens::get_dots_icons;
use crate::screens::{Buy, Community, Home, Info, RoadMap, Stake, UseCases};
use crate::store::{RequestCoingecko, TokenInfo, TokenInfoStore};
use crate::utils::{set_scroll_style, ScrollStyle};
use gloo::timers::callback::{Interval, Timeout};
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::utils::{document, window};
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    prelude::*,
    route::Route,
    switch::Permissive,
    Switch,
};
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
    styles::{Palette, Position, Style},
    text::{Text, TextType},
    tooltip::Tooltip,
};
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

pub struct App {
    navbar_items: Vec<bool>,
    link: ComponentLink<Self>,
    close_navbar_mobile: bool,
    lang: Translations,
    token_info: TokenInfo,
    token_info_store: Box<dyn Bridge<StoreWrapper<TokenInfoStore>>>,
    route_agent: Box<dyn Bridge<RouteAgent<()>>>,
    route: Route<()>,
}

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    HomePath,
    #[to = "/info!"]
    InfoPath,
    #[to = "/use-cases"]
    UseCasesPath,
    #[to = "/buy!"]
    BuyPath,
    #[to = "/stake"]
    StakePath,
    #[to = "/roadmap"]
    RoadMapPath,
    #[to = "/community"]
    CommunityPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

pub enum Msg {
    ChangeNavbarItem(usize),
    NavbarItemInit(usize),
    CloseNavarMobile(MouseEvent),
    GetTokenInfo,
    ScreenUp(usize),
    ScreenDown(usize, usize),
    ScrollMenu(WheelEvent),
    UpdateRoute(Route<()>),
    TokenInfoMsg(ReadOnly<TokenInfoStore>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route = Route::from("/".to_string());
        let callback_route = link.callback(Msg::UpdateRoute);
        let route_agent = RouteAgent::bridge(callback_route);
        let token_info_callback = link.callback(Msg::TokenInfoMsg);
        let token_info_store = TokenInfoStore::bridge(token_info_callback);

        App {
            navbar_items: vec![true, false, false, false, false, false, false],
            link,
            close_navbar_mobile: false,
            lang: Config::get_lang(),
            token_info: TokenInfo::default(),
            token_info_store,
            route,
            route_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TokenInfoMsg(state) => {
                if let Some(token_info) = &state.borrow().token_info {
                    self.token_info = token_info.clone();
                }

                if let Some(token_error) = state.borrow().token_info_error.clone() {
                    ConsoleService::info(&format!(
                        "Status: {}, Error: {}",
                        token_error.status, token_error.message
                    ))
                }
            }

            Msg::GetTokenInfo => {
                self.token_info_store.send(RequestCoingecko::Get1MT);
            }

            Msg::NavbarItemInit(index) => {
                for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                    self.navbar_items[i] = false;
                }

                self.navbar_items[index] = true;
            }
            Msg::ChangeNavbarItem(index) => {
                self.route_agent.send(RouteRequest::ChangeRoute(Route {
                    route: get_route(index),
                    state: (),
                }));
            }

            Msg::ScreenUp(index) => {
                for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                    self.navbar_items[i] = false;
                }
                if index == 0 {
                    self.navbar_items[0] = true;
                    self.link.send_message(Msg::ChangeNavbarItem(0))
                } else {
                    self.navbar_items[index - 1] = true;
                    self.link.send_message(Msg::ChangeNavbarItem(index - 1));
                }
            }
            Msg::ScreenDown(index, len) => {
                for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                    self.navbar_items[i] = false;
                }
                if index == len - 1 {
                    self.navbar_items[len - 1] = true;
                    self.link.send_message(Msg::ChangeNavbarItem(len - 1))
                } else {
                    self.navbar_items[index + 1] = true;
                    self.link.send_message(Msg::ChangeNavbarItem(index + 1));
                }
            }
            Msg::ScrollMenu(wheel_event) => {
                let len = self.navbar_items.len();
                let index_opt = self.navbar_items.to_vec().into_iter().position(|ai| ai);

                if let Some(index) = index_opt {
                    let screen_id = get_screen_id(index);
                    if wheel_event.delta_y() < 0.00 && check_scroll_leave_div_screen_up() {
                        let callback_screen_up = self.link.clone();
                        set_scroll_style(ScrollStyle::ScrollUp, &screen_id, "scroll");
                        Timeout::new(500, move || {
                            callback_screen_up.send_message(Msg::ScreenUp(index))
                        })
                        .forget();
                    } else if check_scroll_leave_div_screen_down() {
                        let callback_screen_down = self.link.clone();
                        set_scroll_style(ScrollStyle::ScrollDown, &screen_id, "scroll");
                        Timeout::new(500, move || {
                            callback_screen_down.send_message(Msg::ScreenDown(index, len))
                        })
                        .forget();
                    }
                }
            }
            Msg::UpdateRoute(route) => {
                let index = get_screen_index(&route.route);
                for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                    self.navbar_items[i] = false;
                }
                self.navbar_items[index] = true;
                self.route = route;
            }
            Msg::CloseNavarMobile(mouse_event) => {
                let target_class = mouse_event
                    .target()
                    .unwrap()
                    .dyn_into::<Element>()
                    .unwrap()
                    .class_list();

                let target_option = mouse_event.target();

                if let Some(target) = target_option {
                    let target_element_option = target.dyn_into::<Element>();

                    if let Ok(target_element) = target_element_option {
                        let parent_element_option = target_element.parent_element();

                        if let Some(parent_element) = parent_element_option {
                            let tag_name = parent_element.tag_name();

                            if !target_class.value().contains("navbar-menu") && tag_name != "svg" {
                                self.close_navbar_mobile = true;
                            } else {
                                self.close_navbar_mobile = false
                            }
                        }
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
            for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                self.navbar_items[i] = false;
            }

            let screen = get_param();
            let screen_index = get_screen_index(&screen);
            self.link.send_message(Msg::NavbarItemInit(screen_index));

            self.link.send_message(Msg::GetTokenInfo);

            let token_info_callback = self.link.clone();

            Interval::new(300000, move || {
                token_info_callback.send_message(Msg::GetTokenInfo);
            })
            .forget();
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="root" onclick=self.link.callback(Msg::CloseNavarMobile)>
                <Navbar
                    navbar_palette=Palette::Clean
                    navbar_style=Style::Outline
                    fixed=Fixed::Top
                    branch=html!{<img src="/1MTlite2.png"/>}
                    hide_navbar_items_mobile = self.close_navbar_mobile
                >
                    <NavbarContainer>
                        {get_navbar(self.navbar_items.to_vec(), self.lang.clone(), self.link.clone())}
                    </NavbarContainer>
                    <NavbarContainer justify_content=JustifyContent::FlexEnd(Mode::NoMode)>
                        <a class=classes!("marketing") href="https://1milliontoken.org/" target="_blank"><img src="/1MTp.png"/><span>{"1MT ETH"}</span></a>
                    </NavbarContainer>
                </Navbar>
                <Container
                    direction=Direction::Column wrap=Wrap::Wrap
                    justify_content=JustifyContent::FlexStart(Mode::NoMode)
                    align_items=AlignItems::Start(Mode::NoMode)
                    align_content=AlignContent::Start(Mode::NoMode)
                    class_name="content-container"
                >
                    <div class="header">
                        <Container
                            direction=Direction::Row wrap=Wrap::Wrap
                            justify_content=JustifyContent::FlexStart(Mode::NoMode)
                            align_items=AlignItems::Center(Mode::NoMode)
                            align_content=AlignContent::Center(Mode::NoMode)
                            class_name="header-content"
                        >
                            <Item layouts=vec![ItemLayout::ItXs(4)] align_self=AlignSelf::Center class_name="logo-b1mt">
                                <img src="/1MTlite2.png"/>
                            </Item>
                            <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItL(4)] align_self=AlignSelf::Center class_name="b1mt-market">
                                <div class="b1mt-market-content">
                                    <div><span>{format!("{}: {}$", self.lang.price, self.token_info.market_data.current_price.usd)}</span><span class="split-bar">{"|"}</span><span>{format!("{}€", self.token_info.market_data.current_price.eur)}</span></div>
                                    <div><span>{format!("{}: {}$", self.lang.market_cap, self.token_info.market_data.market_cap.usd)}</span><span class="split-bar">{"|"}</span><span>{format!("{}€", self.token_info.market_data.market_cap.eur)}</span></div>
                                </div>
                            </Item>
                            <Item layouts=vec![ItemLayout::ItXs(4)] align_self=AlignSelf::Center class_name="content-1mt">
                                <Container
                                    direction=Direction::Row wrap=Wrap::Wrap
                                    justify_content=JustifyContent::FlexEnd(Mode::NoMode)
                                    class_name="container-1mt"
                                >
                                    <div class="logo-1mt">
                                        <a class=classes!("marketing") href="https://1milliontoken.org/" target="_blank"><img src="/1MTp.png"/><span>{"1MT ETH"}</span></a>
                                    </div>
                                </Container>
                            </Item>
                        </Container>
                    </div>
                    <Item layouts=vec![ItemLayout::ItXs(12)] class_name="content-body" align_self=AlignSelf::FlexEnd>
                        <Carousel class_name="carousel" id="screen" onwheel_signal= self.link.callback(Msg::ScrollMenu)>
                            <Container direction=Direction::Row wrap=Wrap::Wrap class_name=format!("screen {}", {
                                let screen = get_param();
                                let screen_index = get_screen_index(&screen);
                                let mut route = get_route(screen_index).replace("/", "");
                                if route.is_empty() {
                                    route = String::from("home");
                                }
                                route
                            }) justify_content=JustifyContent::FlexStart(Mode::NoMode)
                                align_content=AlignContent::FlexStart(Mode::NoMode)
                            >
                                <Item layouts=vec!(ItemLayout::ItXs(1)) align_self=AlignSelf::FlexStart class_name="content carousel-items">
                                    <Container
                                        direction=Direction::Column wrap=Wrap::Wrap
                                        justify_content=JustifyContent::FlexStart(Mode::NoMode)
                                        align_items=AlignItems::FlexStart(Mode::NoMode)
                                        align_content=AlignContent::FlexStart(Mode::NoMode)
                                        class_name="dots">
                                        {get_dots(self.navbar_items.to_vec(), self.link.clone(), self.lang.clone())}
                                    </Container>
                                </Item>
                                <Item layouts=vec!(ItemLayout::ItXs(11)) align_self=AlignSelf::Center class_name="content content-screen">
                                    <div class="content-marging">
                                    <Router<AppRouter, ()>
                                        render = Router::render(|switch: AppRouter| {
                                            match switch {
                                                AppRouter::HomePath => html! {
                                                    <Home/>
                                                },
                                                AppRouter::InfoPath => html! {
                                                    <Info/>
                                                },
                                                AppRouter::UseCasesPath => html! {
                                                    <UseCases/>
                                                },
                                                AppRouter::BuyPath => html! {
                                                    <Buy/>
                                                },
                                                AppRouter::StakePath => html!{<Stake/>},
                                                AppRouter::RoadMapPath => html!{<RoadMap/>},
                                                AppRouter::CommunityPath => html!{<Community/>},
                                                AppRouter::PageNotFound(Permissive(None)) => html!{<h1>{"Page not found"}</h1>},
                                                AppRouter::PageNotFound(Permissive(Some(missed_route))) => html!{<h1>{format!("Page '{}' not found", missed_route)}</h1>}
                                            }
                                        })
                                        redirect = Router::redirect(|route: Route<()>| {
                                            AppRouter::PageNotFound(Permissive(Some(route.route)))
                                        })
                                    />
                                    </div>
                                </Item>
                            </Container>
                        </Carousel>
                    </Item>
                </Container>
            </div>
        }
    }
}

fn get_navbar(items: Vec<bool>, lang: Translations, link: ComponentLink<App>) -> Html {
    let menus = vec![
        lang.home,
        lang.tokenomics,
        lang.use_cases,
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
    document().location().unwrap().pathname().unwrap()
}

fn get_route(index: usize) -> String {
    match index {
        0 => String::from("/"),
        1 => String::from("/info"),
        2 => String::from("/use-cases"),
        3 => String::from("/buy"),
        4 => String::from("/stake"),
        5 => String::from("/roadmap"),
        6 => String::from("/community"),
        _ => String::from("/"),
    }
}

fn get_dot(index: usize, lang: Translations) -> Html {
    let menus = vec![
        lang.home,
        lang.tokenomics,
        lang.use_cases,
        lang.buy,
        lang.stake,
        lang.road_map,
        lang.community,
    ];

    let dots = get_dots_icons();

    match index {
        0 => html! {
            <Tooltip content=get_text(&menus[0]) tooltip_position=Position::Right>
                {dots[0].clone()}
            </Tooltip>
        },
        1 => html! {
                <Tooltip content=get_text(&menus[1]) tooltip_position=Position::Right>
                    {dots[1].clone()}
                </Tooltip>
        },
        2 => html! {
            <Tooltip content=get_text(&menus[2]) tooltip_position=Position::Right>
                {dots[2].clone()}
            </Tooltip>
        },
        3 => html! {
            <Tooltip content=get_text(&menus[3]) tooltip_position=Position::Right>
                {dots[3].clone()}
            </Tooltip>
        },
        4 => html! {
            <Tooltip content=get_text(&menus[4]) tooltip_position=Position::Right>
                {dots[4].clone()}
            </Tooltip>
        },
        5 => html! {
            <Tooltip content=get_text(&menus[5]) tooltip_position=Position::Right>
                {dots[5].clone()}
            </Tooltip>
        },
        6 => html! {
            <Tooltip content=get_text(&menus[6]) tooltip_position=Position::Right>
                {dots[6].clone()}
            </Tooltip>
        },
        _ => html! {
            <Tooltip content=get_text(&menus[0]) tooltip_position=Position::Right>
                {dots[7].clone()}
            </Tooltip>
        },
    }
}

fn get_screen_index(screen: &str) -> usize {
    match screen {
        "/info" => 1,
        "/use-cases" => 2,
        "/buy" => 3,
        "/stake" => 4,
        "/roadmap" => 5,
        "/community" => 6,
        &_ => 0,
    }
}

fn get_dots(items: Vec<bool>, link: ComponentLink<App>, lang: Translations) -> Html {
    let mut dot = vec![];

    for (i, _) in items.clone().into_iter().enumerate() {
        dot.push(html! {
            <CarouselDot active=items[i] onclick_signal = link.callback(move |_| Msg::ChangeNavbarItem(i))>
                {get_dot(i, lang.clone())}
            </CarouselDot>
        })
    }

    dot.into_iter().collect::<Html>()
}

fn check_scroll_leave_div_screen_up() -> bool {
    let window_scroll_y = window().scroll_y().unwrap();

    window_scroll_y == 0.0
}

fn check_scroll_leave_div_screen_down() -> bool {
    let body_scroll_height = document().body().unwrap().scroll_height();
    let window_inner_height = window().inner_height().unwrap();
    let window_page_y_offset = window().page_y_offset().unwrap();

    body_scroll_height as f64 - window_page_y_offset - window_inner_height.as_f64().unwrap() <= 1.0
}

fn get_screen_id(index: usize) -> String {
    let mut screen_id = get_route(index).replace("/", "");

    if screen_id.is_empty() {
        screen_id = String::from("home");
    }

    screen_id
}
