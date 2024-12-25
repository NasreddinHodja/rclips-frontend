use yew::prelude::{function_component, html, Html};

use super::{IndexClipCounter, IndexMakeClipButton};

#[function_component]
pub fn IndexClipCard() -> Html {
    html! {
      <div class="min-h h-44 w-96 m-10 flex flex-col bg-blue-500 rounded-xl gap-5 p-6 text-white justify-between">
        <IndexClipCounter />
        <IndexMakeClipButton />
      </div>
    }
}
