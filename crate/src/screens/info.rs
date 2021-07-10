use crate::config::Config;
use crate::lang::Translations;
use inflector::Inflector;
use yew::prelude::*;
use yew_styles::card::Card;
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Style;
use yew_styles::text::{Header, Text, TextType};

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
    let cards_title = vec![lang.contract, "Coingeko".to_string(), "Chart".to_string()];
    let cards_url = vec![
        "https://bscscan.com/token/0x8d67448d4f6231abc070a42a8905084b79e09136",
        "https://www.coingecko.com/en/coins/1million-token",
        "https://app.windswap.finance/#/tokens/0x8d67448d4f6231abc070a42a8905084b79e09136",
    ];
    let card_src = vec![
        "/bscscan_logo.jpg",
        "/coingeko_logo.png",
        "/windswap_logo.png",
    ];

    cards_title
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let cards_title = c.clone();
            html! {
                <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItM(6), ItemLayout::ItL(4)]>
                    <a href=cards_url[i] target="_blank">
                        <Card
                            class_name="content-card"
                            card_style=Style::Outline
                            header=Some(html!{
                                <img class="content-image" src=card_src[i] alt=cards_title.to_title_case()/>
                            })
                            header_size=11
                            body_size=1
                            body=Some(html!{
                                <Text
                                    text_type=TextType::Title(Header::H3)
                                    plain_text=c.to_title_case()
                                />
                            })
                        />
                    </a>
                </Item>
            }
        })
        .collect::<Html>()
}
