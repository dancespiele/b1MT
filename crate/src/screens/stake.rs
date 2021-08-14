use crate::config::Config;
use crate::lang::Translations;
use inflector::Inflector;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Size;
use yew_styles::text::{Header, Text, TextType};

pub struct Stake {
    lang: Translations,
    kennel_description_ref: NodeRef,
}

impl Component for Stake {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            lang: Config::get_lang(),
            kennel_description_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let kennel_description_lang = self.lang.the_kennel_token_description.clone();

            if let Some(description) = self.kennel_description_ref.cast::<HtmlElement>() {
                description.set_inner_html(&kennel_description_lang);
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <Container
                direction=Direction::Row wrap=Wrap::Wrap
                justify_content=JustifyContent::Center(Mode::NoMode)
                align_items=AlignItems::Center(Mode::NoMode)
                id="stake">
                {get_cards(vec![self.kennel_description_ref.clone()])}
            </Container>
        }
    }
}

fn get_cards(descriptions: Vec<NodeRef>) -> Html {
    let stake_titles = vec!["The Token Kennel"];
    let pools_url = vec!["https://thetokenkennel.com/boarding/#/"];
    let icon_src = vec!["/token_kennel_logo.jpg"];

    stake_titles
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let stake_title = c;
            html! {
                <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItM(6)]>
                    <Container
                        direction=Direction::Row wrap=Wrap::Wrap
                        justify_content=JustifyContent::Center(Mode::NoMode)
                        align_items=AlignItems::Center(Mode::NoMode)>
                        <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItM(3), ItemLayout::ItL(2)]>
                            <a href=pools_url[i] target="_blank">
                                <img class="kennel-image" src=icon_src[i] alt=stake_title.to_title_case()/>
                            </a>
                        </Item>
                        <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItM(6), ItemLayout::ItL(6)]>
                            <Text
                                class_name="kennel-title"
                                text_type=TextType::Title(Header::H3)
                                plain_text=c.to_string()
                            />
                            <Text
                                class_name="kennel-description"
                                text_type=TextType::Plain
                                text_size=Size::Medium
                                html_text=html!{
                                    <p ref=descriptions[i].clone()></p>
                                }
                            />
                            </Item>
                    </Container>
                </Item>
            }
        })
        .collect::<Html>()
}
