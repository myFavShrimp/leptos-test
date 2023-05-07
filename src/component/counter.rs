use lazy_static::lazy_static;
use leptos::{html::*, *};
// use leptos_meta::*;
use leptos_tea::Cmd;

use super::style;

static STYLE_SHEET: &str = grass::include!("src/component/counter.scss");

lazy_static! {
    static ref CLASS_NAME: String = style(STYLE_SHEET).get_class_name().to_string();
    static ref COMPILED_STYLE: String = style(STYLE_SHEET).get_style_str().to_string();
}

#[derive(Default, leptos_tea::Model)]
struct CounterModel {
    counter: u8,
}

#[derive(Default)]
enum Msg {
    Increment,
    Decrement,
    #[default]
    Init,
}

fn update(model: UpdateCounterModel, msg: &Msg, _: Cmd<Msg>) {
    match msg {
        Msg::Increment => model.counter.update(|c| *c += 1),
        Msg::Decrement => model.counter.update(|c| *c -= 1),
        Msg::Init => {}
    }
}

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    // let class_name: &str = &CLASS_NAME;
    // let compiled_style: &str = &COMPILED_STYLE;
    let (model, msg_dispatcher) = CounterModel::default().init(cx, update);

    view! {
        cx,
        // class = class_name,
        <style>
            {STYLE_SHEET}
        </style>
        // <div>{class_name}</div>
        // <div>{compiled_style}</div>
        <h1>{model.counter}</h1>
        <button on:click=move |_| msg_dispatcher.set(Msg::Decrement)>"-"</button>
        <button on:click=move |_| msg_dispatcher.set(Msg::Increment)>"+"</button>
    }
}
