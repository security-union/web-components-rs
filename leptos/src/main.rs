use leptos::*;
use leptos_dom::Mountable;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, ShadowRoot};

#[wasm_bindgen]
pub fn init() {
    let ele = document().get_elements_by_tag_name("CUSTOM-LEPTOS-COMPONENT");
    // TODO: How can we support more than one?
    let ele = ele.get_with_index(0).unwrap();
    let ele = ele
        .dyn_into::<HtmlElement>()
        .expect("Ele is not an HtmlElement");
    let shadow_root = ele.shadow_root().unwrap();
    create_app(shadow_root, |cx| view! { cx, <App/> });
}

pub fn create_app<F, N>(shadow_root: ShadowRoot, f: F)
where
    F: FnOnce(Scope) -> N + 'static,
    N: IntoView,
{
    let runtime: RuntimeId = leptos_reactive::create_runtime();
    let disposer = leptos_reactive::create_scope(runtime, move |cx| {
        let node = f(cx).into_view(cx);

        shadow_root
            .append_child(&node.get_mountable_node())
            .unwrap();

        std::mem::forget(node);
    });

    std::mem::forget(disposer);
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    view! { cx,
        <button on:click=move |_| {
            set_count.update(|n| *n += 1);
        }>

            "Click me"
        </button>
        <p>
            <strong>"Reactive: "</strong>
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            {count}
        </p>
    }
}

fn main() {}
