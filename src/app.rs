use num_bigint::BigUint;
use yew::prelude::{function_component, html, use_state, ContextProvider, Html, UseStateHandle};

use crate::components::index::{IndexBusinessCard, IndexClipCard};

#[derive(Clone, PartialEq)]
pub struct RClips {
    pub clips: UseStateHandle<BigUint>,
    pub money: UseStateHandle<BigUint>,
}

#[function_component]
pub fn App() -> Html {
    let clips = use_state(|| BigUint::from(0u32));
    let money = use_state(|| BigUint::from(0u32));

    let rclips = RClips {
        clips: clips.clone(),
        money: money.clone(),
    };

    html! {
        <ContextProvider<RClips> context={rclips}>
            <div class="w-screen h-screen flex flex-col items-center justify-center">
                <IndexClipCard />
                <IndexBusinessCard />
            </div>
        </ContextProvider<RClips>>
    }
}
