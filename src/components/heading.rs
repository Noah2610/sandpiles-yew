use crate::props::{self, ToClass};
use yew::prelude::*;

pub struct Heading {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub size:     props::Size,
}

pub enum Msg {}

impl Component for Heading {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
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
            <p class=(&self.props.size.to_class(), "text-gray-100")>
                { self.props.children.clone() }
            </p>
        }
    }
}
