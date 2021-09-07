use crate::config::Config;
use crate::lang::Translations;
use inflector::Inflector;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{AlignSelf, Item, ItemLayout},
};
use yew_styles::styles::Size;
use yew_styles::text::{Header, Text, TextType};

pub struct UseCases {
    partner_description_ref: NodeRef,
    lang: Translations,
}

impl Component for UseCases {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            lang: Config::get_lang(),
            partner_description_ref: NodeRef::default(),
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
            let partner_description_lang = self
                .lang
                .partner_with_1mt_description
                .replace("\n", "<br/>");

            if let Some(description) = self.partner_description_ref.cast::<HtmlElement>() {
                description.set_inner_html(&partner_description_lang);
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <Container
                direction=Direction::Row wrap=Wrap::Wrap
                justify_content=JustifyContent::Center(Mode::NoMode)
                align_items=AlignItems::Center(Mode::NoMode)
                id="use-cases">
                {get_cards(self.lang.clone(), self.partner_description_ref.clone())}
            </Container>
        }
    }
}

fn get_cards(lang: Translations, partner_description_ref: NodeRef) -> Html {
    let cards_title = vec!["1MT Vegas Casino", &lang.partner_with_1mt];

    let card_src = vec!["/vc1mt.svg", "/b1mt.svg"];

    cards_title
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let cards_title = c;
            html! {
                <Item layouts=vec![ItemLayout::ItXs(12)] class_name="use-cases-content" align_self=AlignSelf::FlexStart>
                    <Container
                        direction=Direction::Row wrap=Wrap::Wrap
                        justify_content=JustifyContent::Center(Mode::NoMode)
                        align_items=AlignItems::Center(Mode::NoMode)>
                        <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItL(1)] align_self=AlignSelf::FlexStart>
                            <img class="use-cases-image" src=card_src[i] alt=cards_title.to_title_case()/>
                        </Item>
                        <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItL(4)]>
                            <Text
                                text_type=TextType::Title(Header::H2)
                                plain_text=c.to_string()
                                class_name="use-cases-title"
                            />
                            <Text
                                text_type=TextType::Plain
                                text_size=Size::Medium
                                class_name="use-cases-description"
                                html_text=html!{
                                    if c == lang.partner_with_1mt {
                                        html!{
                                            <p ref=partner_description_ref.clone()></p>
                                        }
                                    } else {
                                        html!{<p>{lang.vegas_casino_description.clone()}</p>}
                                    }
                                }
                            />
                            </Item>
                    </Container>
                </Item>
            }
        })
        .collect::<Html>()
}
