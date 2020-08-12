use std::convert::{TryFrom, TryInto};
use std::time::Duration;
use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use yew::services::{IntervalService, Task};
use yew::web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Sandpiles {
    props:      Props,
    link:       ComponentLink<Self>,
    canvas_ref: NodeRef,
    tick_job:   Box<dyn Task>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(600)]
    pub width:            u32,
    #[prop_or(600)]
    pub height:           u32,
    #[prop_or(500)]
    pub tick_interval_ms: u64,
}

pub enum Msg {
    Tick,
}

impl Sandpiles {
    fn get_context(&self) -> Option<CanvasRenderingContext2d> {
        self.canvas_ref
            .cast::<HtmlCanvasElement>()
            .and_then(|canvas| {
                canvas
                    .get_context("2d")
                    .ok()
                    .and_then(|ctx_opt| ctx_opt)
                    .and_then(|ctx| {
                        ctx.dyn_into::<CanvasRenderingContext2d>().ok()
                    })
            })
    }
}

impl Component for Sandpiles {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::Tick);
        let handle = IntervalService::spawn(
            Duration::from_millis(props.tick_interval_ms),
            callback,
        );

        Self {
            props,
            link,
            canvas_ref: NodeRef::default(),
            tick_job: Box::new(handle),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(ctx) = self.get_context() {
                ctx.set_fill_style(&JsValue::from("#fff"));
                ctx.fill_rect(0.0, 0.0, 300.0, 300.0);
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                if let Some(ctx) = self.get_context() {
                    console::log_1(&JsValue::from("TICK"))
                }
            }
        }
        false
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