use yew::{
    prelude::{function_component, html, Html},
    use_context,
};

use crate::app::RClips;

#[function_component]
pub fn IndexClipCounter() -> Html {
    let rclips = use_context::<RClips>().expect("No context found");
    let clips: String = rclips.clips.to_string();

    html! {
        <div>
            <span class="text-xl font-bold">{ "Clips: " }</span>
            <span class="text-xl">{ clips }</span>
        </div>
    }
}
