use yew::prelude::*;

pub struct Tweet {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub message: String,
    #[prop_or(0)]
    pub interval: u64,
}

impl Component for Tweet {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{ &self.props.message }</p>
                <p>{ self.props.interval }</p>
            </div>
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}
