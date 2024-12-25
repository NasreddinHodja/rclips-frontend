use num_bigint::BigUint;
use yew::{
    prelude::{function_component, html, Html},
    use_context,
};

use crate::{app::RClips, utils::format_currency};

#[function_component]
pub fn IndexBusinessCard() -> Html {
    let rclips = use_context::<RClips>().expect("No context found");
    let formatted_value = format_currency(&*rclips.money);
    let unsold_inventory = "0".to_owned();
    let price_per_clip = format_currency(&BigUint::from(0u32));

    html! {
      <div class="h-44 w-96 m-10 flex flex-col bg-yellow-600 rounded-xl gap-2 p-6 text-white">
        <span class="font-bold text-xl mb-2">{ "Business" }</span>
        <div>{ format!("Available funds: {formatted_value}") }</div>
        <div>{ format!("Unsold inventory: {unsold_inventory}") }</div>
        <div class="flex flex-row gap-1">
            <button class="rounded px-2 bg-red-500">{ "Lower" }</button>
            <button class="rounded px-2 bg-green-500">{ "Raise" }</button>
            { format!("Price per clip: {price_per_clip}") }
        </div>
      </div>
    }
}
