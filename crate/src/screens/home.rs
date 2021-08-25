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
                align_items=AlignItems::FlexEnd(Mode::NoMode)
                id="home">
                <Item layouts=vec!(ItemLayout::ItXs(5)) align_self=AlignSelf::FlexStart>
                    <h2 class="slogan">{"Lorem Ipsum dolor sit amet"}</h2>
                    <p class="slogan-description">
                        {"Perfecto principes a Illud discere quo et, sea eu aperiam praesent. Nec ne prima rebum voluptatibus."}
                    </p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(7)) class_name="moon-target">
                    <img src="/moon_target.png"/>
                </Item>

                <Container
                    direction=Direction::Row wrap=Wrap::Wrap
                    justify_content=JustifyContent::Center(Mode::NoMode)
                    align_items=AlignItems::FlexEnd(Mode::NoMode)
                    class_name="home-description"
                >
                    <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(2))>
                        <img class="logo-main-page" src="/1MTlite2.png" />
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(3)) align_self=AlignSelf::Center class_name="home-parragraph right-line">
                        <p>{self.lang.description.clone()}</p>
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(2))>
                        <img class="logo-main-page" src="/P1MTMoon2b.png" />
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(3)) align_self=AlignSelf::Center class_name="home-parragraph">
                        <p>{self.lang.community_project.clone()}</p>
                    </Item>
                </Container>
            </Container>
        }
    }
}
