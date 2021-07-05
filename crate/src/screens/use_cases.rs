use crate::config::Config;
use crate::lang::Translations;
use inflector::Inflector;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_styles::card::Card;
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::{Size, Style};
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
                align_items=AlignItems::Center(Mode::NoMode)>
                {get_cards(self.lang.clone(), self.partner_description_ref.clone())}
            </Container>
        }
    }
}

fn get_cards(lang: Translations, partner_description_ref: NodeRef) -> Html {
    let cards_title = vec!["1MT Vegas Casino", &lang.partner_with_1mt];

    let card_src = vec!["/bscscan_logo.jpg", "/coingeko_logo.png"];

    cards_title
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let cards_title = c.clone();
            html! {
                <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItM(6), ItemLayout::ItL(4)]>
                    <Card
                        class_name="content-card"
                        card_style=Style::Outline
                        header=Some(html!{
                            <img class="content-image" src=card_src[i] alt=cards_title.to_title_case()/>
                        })
                        header_size=11
                        body_size=1
                        body=Some(html!{
                            <>
                                <Text
                                    text_type=TextType::Title(Header::H3)
                                    plain_text=c.to_string()
                                />
                                <Text
                                    text_type=TextType::Plain
                                    text_size=Size::Medium
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
                            </>
                        })
                    />
                </Item>
            }
        })
        .collect::<Html>()
}
