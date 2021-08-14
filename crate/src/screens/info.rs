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

pub struct Info {
    lang: Translations,
}

impl Component for Info {
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
                id="info">
                {get_cards(self.lang.clone())}
            </Container>
        }
    }
}

fn get_cards(lang: Translations) -> Html {
    let tooltip_titles = vec!["Coingeko", &lang.contract, "Chart"];
    let info_url = vec![
        "https://bscscan.com/token/0x8d67448d4f6231abc070a42a8905084b79e09136",
        "https://www.coingecko.com/en/coins/1million-token",
        "https://dex.guru/token/0x8d67448d4f6231abc070a42a8905084b79e09136-bsc",
    ];
    let icons_src = vec!["/bscscan_logo.jpg", "/coingeko_logo.png", "/dex_guru.png"];

    tooltip_titles
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let tooltip_title = c;
            let tokenomics_class = classes!(if i == info_url.len() - 1 {
                "content-last-icon"
            } else {
                "content-icon"
            });

            html! {
                <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItM(6), ItemLayout::ItL(4)]>
                    <div class=tokenomics_class>
                        <Tooltip
                            content=html!{<span>{tooltip_title}</span>}
                            tooltip_position=Position::Below
                        >
                            <a href=info_url[i] target="_blank">
                                <img class="content-image" src=icons_src[i] alt=tooltip_title.to_title_case()/>
                            </a>
                        </Tooltip>
                    </div>
                </Item>
            }
        })
        .collect::<Html>()
}
