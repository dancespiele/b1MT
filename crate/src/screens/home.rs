use crate::config::Config;
use crate::lang::Translations;
use yew::prelude::*;
use yew_styles::layouts::{
    container::{AlignContent, AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
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
                justify_content=JustifyContent::FlexStart(Mode::NoMode)
                align_items=AlignItems::FlexStart(Mode::NoMode)
                align_content=AlignContent::FlexStart(Mode::NoMode)
                id="home">
                <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::Center>
                    <h2 class="slogan">{self.lang.slogan.clone()}</h2>
                    <p class="slogan-description">
                        {self.lang.slogan_description.clone()}
                    </p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(8)) align_self=AlignSelf::FlexStart class_name="moon-target">
                    <img src="/moon_target.png"/>
                </Item>

                <Container
                    direction=Direction::Row wrap=Wrap::Wrap
                    justify_content=JustifyContent::FlexStart(Mode::NoMode)
                    align_items=AlignItems::FlexEnd(Mode::NoMode)
                    class_name="home-description"
                >
                    <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(6)) align_self=AlignSelf::FlexStart>
                        <Container
                            direction=Direction::Row wrap=Wrap::Wrap
                            justify_content=JustifyContent::Center(Mode::NoMode)
                            class_name="right-line"
                        >
                            <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(4))>
                                <img class="logo-main-page" src="/b1mt.svg" />
                            </Item>
                            <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(7)) align_self=AlignSelf::FlexStart class_name="home-parragraph">
                                <p>{self.lang.description.clone()}</p>
                            </Item>
                        </Container>
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(6)) align_self=AlignSelf::FlexStart>
                        <Container
                            direction=Direction::Row wrap=Wrap::Wrap
                            justify_content=JustifyContent::Center(Mode::NoMode)
                        >
                            <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(4))>
                                <img class="logo-space" src="/P1MTMoon2b.png" />
                            </Item>
                            <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(7)) align_self=AlignSelf::FlexStart class_name="home-parragraph">
                                <p>{self.lang.community_project.clone()}</p>
                            </Item>
                        </Container>
                    </Item>
                </Container>
            </Container>
        }
    }
}
