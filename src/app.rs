use crate::components::{Heading, Page};
use crate::props;
use yew::prelude::*;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Page>
                <Heading size=props::Size::Xl>
                    { "Hello World!" }
                </Heading>
                <div class=("h-16", "bg-black")>
                    { "Tailwind?" }
                </div>
            </Page>
        }
    }
}
