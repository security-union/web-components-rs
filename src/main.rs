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
pub fn init(root_id: &str) {
    let doc = window().unwrap().document().unwrap();
    let root = doc.get_element_by_id(root_id).unwrap_or_else(|| {
        let root = doc.create_element("div").unwrap();
        root.set_attribute("id", "custom-yew-component").unwrap();
        doc.body().unwrap().append_child(&root).unwrap();
        root
    });
    yew::Renderer::<App>::with_root(root).render();
}

fn main() {
}
