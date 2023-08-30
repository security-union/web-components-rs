use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

#[wasm_bindgen]
pub fn init() {
    let ele = window()
        .unwrap()
        .document()
        .unwrap()
        .get_elements_by_tag_name("CUSTOM-YEW-COMPONENT");
    // TODO: How can we support more than one?
    let ele = ele.get_with_index(0).unwrap();
    // let ele = ele
    //     .dyn_into::<HtmlElement>()
    //     .expect("Ele is not an HtmlElement");
    yew::Renderer::<App>::with_root(ele).render();
}

fn main() {}
