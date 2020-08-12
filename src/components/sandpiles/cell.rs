use yew::prelude::*;

const CELL_SIZE: (i32, i32) = (16, 16);

pub struct Cell {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: i32,
    pub x:     i32,
    pub y:     i32,
}

pub enum Msg {}

impl Component for Cell {
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
            <div
                class=(
                    "inline",
                    "absolute",
                    "bg-red-900",
                    &format!("bg-opacity-{}", match self.props.value {
                        i32::MIN ..= 0 => "0",
                        1 => "25",
                        2 => "50",
                        3 => "75",
                        4 ..= i32::MAX => "100",
                    }),
                )
                style=format!(
                    "width: {}px; height: {}px; transform: translate({}px, {}px);",
                    CELL_SIZE.0,
                    CELL_SIZE.1,
                    self.props.x * CELL_SIZE.0,
                    self.props.y * CELL_SIZE.1,
                )
            >

            </div>
        }
    }
}
