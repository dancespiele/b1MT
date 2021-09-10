use yew::prelude::*;

use yew_styles::layouts::{
    container::{AlignContent, AlignItems, Container, Direction, Mode, Wrap},
    item::{Item, ItemLayout},
};

pub struct Info;

impl Component for Info {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
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
                direction=Direction::Column wrap=Wrap::Wrap
                align_content=AlignContent::Center(Mode::NoMode)
                align_items=AlignItems::FlexStart(Mode::NoMode)
                id="info">
                {get_cards()}
            </Container>
        }
    }
}

fn get_cards() -> Html {
    let info_url = vec![
        "https://bscscan.com/token/0x8d67448d4f6231abc070a42a8905084b79e09136",
        "https://www.coingecko.com/en/coins/1million-token",
        "https://dex.guru/token/0x8d67448d4f6231abc070a42a8905084b79e09136-bsc",
    ];

    let icons_src = vec!["/bscscan_logo.svg", "/coingecko_logo.svg", "/dex_guru.svg"];

    info_url
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, u)| {
            let tokenomics_class = classes!(if i == info_url.len() - 1 {
                "content-last-icon"
            } else {
                "content-icon"
            });

            html! {
                <Item layouts=vec![ItemLayout::ItL(4)]>
                    <div class=tokenomics_class>
                        <a href=u target="_blank">
                            <img src=icons_src[i] />
                        </a>
                    </div>
                </Item>
            }
        })
        .collect::<Html>()
}
