use leptos::*;
use leptos_meta::*;
use leptos_tea::Cmd;

mod component;

use component::counter::*;
use component::counter2::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <Counter /><Counter2 /> })
}
