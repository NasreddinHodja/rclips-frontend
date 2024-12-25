use crate::app::RClips;
use num_bigint::BigUint;
use yew::{
    prelude::{function_component, html, Html},
    use_context, Callback,
};

#[function_component]
pub fn IndexMakeClipButton() -> Html {
    let rclips = use_context::<RClips>().expect("No context found");
    let make_clip = Callback::from(move |_| {
        let curr_value = &*(rclips.clips.clone());
        rclips.clips.set(curr_value + BigUint::from(1u32));
    });

    html! {
        <div class="flex justify-end">
            <button
                onclick={make_clip}
                class="rounded px-5 py-1 bg-green-500"
            >{ "Make clip" }</button>
        </div>
    }
}
