use crate::config::Config;
use crate::lang::Translations;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use yew::utils::document;
use yew_styles::layouts::{
    container::{Container, Direction, JustifyContent, Mode, Wrap},
    item::{Item, ItemLayout},
};

pub struct RoadMapText {
    text: String,
    done: bool,
}

pub struct RoadMap {
    lang: Translations,
}

impl Component for RoadMap {
    type Properties = ();
    type Message = ();

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

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            draw_horizontal_line(get_text(self.lang.clone()));
        }
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap justify_content=JustifyContent::Center(Mode::NoMode)>
                <Item layouts=vec![ItemLayout::ItXs(12)]>
                    <canvas id="canvas" width="500" height="500">
                    </canvas>
                </Item>
            </Container>
        }
    }
}

fn get_context_2d() -> CanvasRenderingContext2d {
    let canvas = document().get_element_by_id("canvas").unwrap();

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

fn get_text(lang: Translations) -> Vec<RoadMapText> {
    vec![
        RoadMapText {
            text: lang.launch,
            done: true,
        },
        RoadMapText {
            text: lang.governance,
            done: true,
        },
        RoadMapText {
            text: lang.kennel_token_pool,
            done: true,
        },
        RoadMapText {
            text: lang.web,
            done: true,
        },
        RoadMapText {
            text: lang.swap_web,
            done: false,
        },
        RoadMapText {
            text: lang.automata_token,
            done: false,
        },
        RoadMapText {
            text: lang.cross_swap,
            done: false,
        },
        RoadMapText {
            text: lang.governance_web,
            done: false,
        },
    ]
}

fn draw_horizontal_line(road_map_text: Vec<RoadMapText>) {
    let context = get_context_2d();

    let x = 220.0;
    let y = 0.0;

    context.begin_path();

    context.move_to(x, y);
    context.line_to(x, 600.0);
    context.set_font("14px serif");

    road_map_text.into_iter().enumerate().for_each(|(i, t)| {
        let text_color = if t.done {
            JsValue::from_str("#2DBCC9")
        } else {
            JsValue::from_str("#0E6979")
        };

        context.set_fill_style(&text_color);

        let n = i + 1;
        let yh = n as f64 * 50.0;
        if n != 1 && n % 2 == 0 {
            let xh = 0.0;
            context.move_to(x, yh);
            context.line_to(xh, yh);
            context.fill_text(&t.text, xh, yh - 10.0).unwrap();
        } else {
            let xh = 400.0;
            context.move_to(x, yh);
            context.line_to(xh, yh);
            context.fill_text(&t.text, xh - 170.0, yh - 10.0).unwrap();
        }

        context.set_stroke_style(&text_color);
    });

    context.set_line_width(5.0);
    context.fill();
    context.stroke();
}
