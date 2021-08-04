use crate::config::Config;
use crate::lang::Translations;
use crate::screens::{Buy, Community, Home, Info, RoadMap, Stake, UseCases};
use yew::prelude::*;
use yew::utils::{document, window};
use yew_assets::business_assets::{BusinessAssets, BusinessIcon};
use yew_assets::communication_assets::{CommunicationAssets, CommunicationIcon};
use yew_assets::ux_assets::{UxAssets, UxIcon};
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
    styles::{Palette, Position, Style},
    text::{Text, TextType},
    tooltip::Tooltip,
};

use gloo::timers::callback::Timeout;

pub struct App {
    navbar_items: Vec<bool>,
    link: ComponentLink<Self>,
    lang: Translations,
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
    ScreenUp(usize),
    ScreenDown(usize, usize),
    ScrollMenu(WheelEvent),
    UpdateRoute(Route<()>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route = Route::from("/".to_string());
        let callback_route = link.callback(Msg::UpdateRoute);
        let route_agent = RouteAgent::bridge(callback_route);

        App {
            navbar_items: vec![true, false, false, false, false, false, false],
            link,
            lang: Config::get_lang(),
            route,
            route_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
                }))
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
                    if wheel_event.delta_y() < 0.00 && check_scroll_leave_div_screen_up() {
                        let callback_screen_up = self.link.clone();
                        Timeout::new(500, move || {
                            callback_screen_up.send_message(Msg::ScreenUp(index))
                        })
                        .forget();
                    } else if check_scroll_leave_div_screen_down() {
                        let callback_screen_down = self.link.clone();
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
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="root">
                // <Navbar
                //     navbar_palette=Palette::Clean
                //     navbar_style=Style::Outline
                //     fixed=Fixed::Top
                //     branch=html!{<img src="./1MTlite2.png"/>}
                // >
                //     <NavbarContainer>
                //         {get_navbar(self.navbar_items.to_vec(), self.lang.clone(), self.link.clone())}
                //     </NavbarContainer>
                //     <NavbarContainer justify_content=JustifyContent::FlexEnd(Mode::NoMode)>
                //         <a class=classes!("marketing") href="https://1milliontoken.org/" target="_blank"><img src="/1MTp.png"/><span>{"1MT ETH"}</span></a>
                //     </NavbarContainer>
                // </Navbar>
                <Carousel class_name="carousel" id="screen" onwheel_signal= self.link.callback(Msg::ScrollMenu)>

                    <Container direction=Direction::Row wrap=Wrap::Wrap class_name="screen" justify_content=JustifyContent::FlexStart(Mode::NoMode)>
                        <Item layouts=vec!(ItemLayout::ItXs(1)) align_self=AlignSelf::Center class_name="content">
                            <Container
                                direction=Direction::Column wrap=Wrap::Wrap
                                justify_content=JustifyContent::FlexStart(Mode::NoMode)
                                align_items=AlignItems::FlexStart(Mode::NoMode)
                                align_content=AlignContent::FlexStart(Mode::NoMode)
                                class_name="dots">
                                {get_dots(self.navbar_items.to_vec(), self.link.clone(), self.lang.clone())}
                            </Container>
                        </Item>
                        <Item layouts=vec!(ItemLayout::ItXs(10)) align_self=AlignSelf::Center class_name="content">
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
                        </Item>
                    </Container>
                </Carousel>
            </div>
        }
    }
}

// fn get_navbar(items: Vec<bool>, lang: Translations, link: ComponentLink<App>) -> Html {
//     let menus = vec![
//         lang.home,
//         lang.tokenomics,
//         lang.use_cases,
//         lang.buy,
//         lang.stake,
//         lang.road_map,
//         lang.community,
//     ];
//
//     let mut navbar_items = vec![];
//
//     for (i, _) in items.clone().into_iter().enumerate() {
//         navbar_items.push(html! {
//             <NavbarItem
//                 active = items[i]
//                 onclick_signal = link.callback(move |_| Msg::ChangeNavbarItem(i))
//                 >
//                 {get_text(menus[i].as_str())}
//             </NavbarItem>
//         })
//     }
//
//     navbar_items.into_iter().collect::<Html>()
// }

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

    match index {
        0 => html! {
            <Tooltip content=get_text(&menus[0]) tooltip_position=Position::Right>
                <UxAssets icon=UxIcon::Home size=("40".to_string(), "40".to_string())/>
            </Tooltip>
        },
        1 => html! {
                <Tooltip content=get_text(&menus[1]) tooltip_position=Position::Right>
                    <BusinessAssets icon=BusinessIcon::TrendingUp size=("40".to_string(), "40".to_string())/>
                </Tooltip>
        },
        2 => html! {
            <Tooltip content=get_text(&menus[2]) tooltip_position=Position::Right>
                <UxAssets icon=UxIcon::Tool size=("40".to_string(), "40".to_string())/>
            </Tooltip>
        },
        3 => html! {
            <Tooltip content=get_text(&menus[3]) tooltip_position=Position::Right>
                <BusinessAssets icon=BusinessIcon::DollarSign size=("40".to_string(), "40".to_string())/>
            </Tooltip>
        },
        4 => html! {
            <Tooltip content=get_text(&menus[4]) tooltip_position=Position::Right>
                <UxAssets icon=UxIcon::Lock size=("40".to_string(), "40".to_string())/>
            </Tooltip>
        },
        5 => html! {
            <Tooltip content=get_text(&menus[5]) tooltip_position=Position::Right>
                <BusinessAssets icon=BusinessIcon::Target size=("40".to_string(), "40".to_string())/>
            </Tooltip>
        },
        6 => html! {
            <Tooltip content=get_text(&menus[6]) tooltip_position=Position::Right>
                <CommunicationAssets icon=CommunicationIcon::Smile size=("40".to_string(), "40".to_string())/>
            </Tooltip>
        },
        _ => html! {
            <Tooltip content=get_text(&menus[0]) tooltip_position=Position::Right>
                <UxAssets icon=UxIcon::Home size=("40".to_string(), "40".to_string())/>
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
