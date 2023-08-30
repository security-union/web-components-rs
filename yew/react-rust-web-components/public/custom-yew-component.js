import init from '/web-components-rs.js';

const yewComponent = async () => {
  let wasm = await init('web-components-rs_bg.wasm');
  wasm.init();
}

class CustomYewComponent extends HTMLElement {
  constructor() {
    super();
    // this.attachShadow({ mode: 'open' });
    yewComponent();
  }
}

customElements.define('custom-yew-component', CustomYewComponent);
