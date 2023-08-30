import init from '/web-components-rs.js';

const leptosComponent = async () => {
  let wasm = await init('web-components-rs_bg.wasm');
  wasm.init();
}

class CustomLeptosComponent extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    leptosComponent();
  }
}

customElements.define('custom-leptos-component', CustomLeptosComponent);
