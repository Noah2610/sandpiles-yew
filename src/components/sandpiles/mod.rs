use std::convert::{TryFrom, TryInto};
use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use yew::web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Sandpiles {
    props:      Props,
    canvas_ref: NodeRef,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(600)]
    pub width:  u32,
    #[prop_or(600)]
    pub height: u32,
}

pub enum Msg {}

impl Component for Sandpiles {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
            canvas_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                if let Some(ctx) = canvas
                    .get_context("2d")
                    .ok()
                    .and_then(|ctx_opt| ctx_opt)
                    .and_then(|ctx| {
                        ctx.dyn_into::<CanvasRenderingContext2d>().ok()
                    })
                {
                    ctx.set_fill_style(&JsValue::from("#fff"));
                    ctx.fill_rect(0.0, 0.0, 300.0, 300.0);
                }
            }
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div
                class=(
                    "flex",
                    "justify-center",
                    "flex-basis",
                )
                style=format!(
                    "flex-basis: {}px; height: {}px;",
                    self.props.width,
                    self.props.height,
                )
            >
                <canvas
                    ref=self.canvas_ref.clone()
                    class=(
                        "block",
                        "bg-black",
                        "w-full",
                        "h-full",
                    )
                    width=self.props.width,
                    height=self.props.height,
                >
                </canvas>
            </div>
        }
    }
}
