use num_bigint::BigUint;
use yew::prelude::{function_component, html, Html};

use super::{IndexClipCounter, IndexMakeClipButton};

use crate::utils::format_currency;

#[function_component]
pub fn IndexBusinessCard() -> Html {
    let formatted_value = format_currency(BigUint::from(0u32));
    html! {
      <div class="h-44 w-full m-10 flex flex-col bg-yellow-600 rounded-xl gap-5 p-6 text-white justify-between">
        { format!("Available funds: {formatted_value}") }
      </div>
    }
}
