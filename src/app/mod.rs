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
            <div class="absolute top-0 bottom-0 left-0 right-0 flex justify-center bg-gray-900 text-gray-200">
                <div class="container px-8 py-4">
                    <h1>
                        { "Hello World!" }
                    </h1>
                    <div class=("h-16", "bg-black")>
                        { "Tailwind?" }
                    </div>
                </div>
            </div>
        }
    }
}
