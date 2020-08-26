use yew::prelude::*;

pub struct Tweet {
    link: ComponentLink<Self>,
    message: String,
    interval: u64
}

