use crate::config::Config;
use crate::lang::Translations;
use yew::prelude::*;
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{AlignSelf, Item, ItemLayout},
};

pub struct Home {
    lang: Translations,
}

impl Component for Home {
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
                id="home">
                <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItM(3))>
                    <img class="logo-main-page" src="./1MTlite2.png" />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItM(3)) align_self=AlignSelf::Center>
                    <h3>{self.lang.description.clone()}</h3>
                </Item>
            </Container>
        }
    }
}
