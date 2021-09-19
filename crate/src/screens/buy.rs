use inflector::Inflector;
use yew::prelude::*;
use yew_styles::layouts::{
    container::{AlignContent, AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Position;
use yew_styles::tooltip::Tooltip;

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
                align_items=AlignItems::FlexStart(Mode::NoMode)
                align_content=AlignContent::Center(Mode::NoMode)
                id="buy">
                <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItL(4)]>
                <Container
                    direction=Direction::Row wrap=Wrap::Wrap
                    justify_content=JustifyContent::FlexStart(Mode::NoMode)
                    align_items=AlignItems::Center(Mode::NoMode)
                >
                    {get_cards()}
                </Container>
                </Item>
            </Container>
        }
    }
}

fn get_cards() -> Html {
    let swaps_title = vec!["Pancakeswap", "Dex Guru"];
    let swaps_url = vec![
        "https://v1exchange.pancakeswap.finance/#/swap?outputCurrency=0x8d67448d4f6231ABc070a42A8905084b79E09136",
        "https://dex.guru/token/0x8d67448d4f6231abc070a42a8905084b79e09136-bsc"
    ];
    let icons_src = vec!["/pancakeswap_swap.svg", "/dex_guru.svg"];

    swaps_title
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let swap_title = c;
            let buy_class = classes!(if i == swaps_url.len() - 1 {
                "content-last-icon"
            } else {
                "content-icon"
            });

            html! {
                <Item layouts=vec![ItemLayout::ItXs(12)]>
                    <div class=buy_class>
                        <Tooltip
                            content=html!{<span>{swap_title}</span>}
                            tooltip_position=Position::Below
                        >
                            <a href=swaps_url[i] target="_blank">
                                <img class="content-image" src=icons_src[i] alt=swap_title.to_title_case()/>
                            </a>
                        </Tooltip>
                    </div>
                </Item>
            }
        })
        .collect::<Html>()
}
