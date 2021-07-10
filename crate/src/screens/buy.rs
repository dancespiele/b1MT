use inflector::Inflector;
use yew::prelude::*;
use yew_styles::card::Card;
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Style;
use yew_styles::text::{Header, Text, TextType};

pub struct Buy;

impl Component for Buy {
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
                direction=Direction::Row wrap=Wrap::Wrap
                justify_content=JustifyContent::Center(Mode::NoMode)
                align_items=AlignItems::Center(Mode::NoMode)
                id="buy">
                {get_cards()}
            </Container>
        }
    }
}

fn get_cards() -> Html {
    let cards_title = vec!["Pancakeswap", "Windswap"];
    let cards_url = vec![
        "https://v1exchange.pancakeswap.finance/#/swap?outputCurrency=0x8d67448d4f6231ABc070a42A8905084b79E09136",
        "https://v1app.windswap.finance/#/swap?outputCurrency=0x8d67448d4f6231ABc070a42A8905084b79E09136",
    ];
    let card_src = vec!["/pancakeswap_logo.png", "/windswap_logo.png"];

    cards_title
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let cards_title = c.clone();
            html! {
                <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItM(6)]>
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
