use yew::prelude::*;

pub struct Page {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

pub enum Msg {}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, new_props: Self::Properties) -> ShouldRender {
        self.props != new_props
    }

    fn view(&self) -> Html {
        html! {
            <div class="absolute top-0 bottom-0 left-0 right-0 flex justify-center bg-gray-900 text-gray-200">
                <div class="container px-8 py-4">
                    { self.props.children.clone() }
                </div>
            </div>
        }
    }
}
