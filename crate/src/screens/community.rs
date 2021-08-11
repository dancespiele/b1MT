use crate::config::Config;
use crate::lang::Translations;
use inflector::Inflector;
use yew::prelude::*;
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Position;
use yew_styles::tooltip::Tooltip;

pub struct Community {
    lang: Translations,
}

impl Component for Community {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            lang: Config::get_lang(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container
                direction=Direction::Row wrap=Wrap::Wrap
                justify_content=JustifyContent::Center(Mode::NoMode)
                align_items=AlignItems::Center(Mode::NoMode)
                id="community">
                {get_cards(self.lang.clone())}
            </Container>
        }
    }
}

fn get_cards(lang: Translations) -> Html {
    let tooltip_titles = vec!["Twitter", "Telegram", "Discord", "Medium", &lang.governance];
    let social_url = vec![
        "https://twitter.com/1MillionBsc",
        "https://t.me/MillionToken",
        "https://discord.gg/CW9qnCRq",
        "https://1millionposts.medium.com",
        "https://snapshot.org/#/b1mt.eth",
    ];
    let icon_src = vec![
        "/twitter_logo.png",
        "/telegram_logo.png",
        "/discord_logo.png",
        "/medium_logo.jpeg",
        "/snapshot.png",
    ];

    tooltip_titles
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let tooltip_title = c;
            html! {
                <Item class_name="social-icons" layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItM(6), ItemLayout::ItL(4)]>
                    <Tooltip
                        content=html!{<span>{tooltip_title}</span>}
                        tooltip_position=Position::Below
                    >
                        <a href=social_url[i] target="_blank">
                            <img class="content-image" src=icon_src[i] alt=tooltip_title.to_title_case()/>
                        </a>
                    </Tooltip>
                </Item>
            }
        })
        .collect::<Html>()
}
