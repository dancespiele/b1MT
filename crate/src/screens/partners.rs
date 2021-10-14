use yew::prelude::*;
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Position;
use yew_styles::tooltip::Tooltip;

pub struct Partners;

impl Component for Partners {
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
                id="partners">
                    <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItL(5)] class_name="partners-content">
                        <Container
                            direction=Direction::Row wrap=Wrap::Wrap
                            justify_content=JustifyContent::Center(Mode::NoMode)
                            align_items=AlignItems::Center(Mode::NoMode)>
                            {get_cards()}
                        </Container>
                    </Item>
            </Container>
        }
    }
}

fn get_cards() -> Html {
    let partners_titles = vec!["The Token Kennel", "Yield Field" , "Money Time"];
    let partners_url = vec![
        "https://thetokenkennel.com/boarding/#/",
        "https://yieldfields.finance",
        "https://moneytime.finance/",
    ];
    let icon_src = vec!["/token_kennel.png", "/yieldfields.png", "/money_time.png"];

    partners_titles
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let partner_title = c;
            html! {
                <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItL(1)]
                    class_name="social-icon">
                    <Tooltip
                        content=html!{<span>{partner_title}</span>}
                        tooltip_position=Position::Below
                    >
                        <a href=partners_url[i] target="_blank">
                            <img src=icon_src[i] alt=partner_title/>
                        </a>
                    </Tooltip>
                </Item>
            }
        })
        .collect::<Html>()
}
