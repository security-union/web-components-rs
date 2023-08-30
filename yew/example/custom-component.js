class CustomComponent extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });

    // Call the JavaScript wrapper to initialize the Yew component
    let div = document.createElement('div');
    div.innerHTML = "<h2>YOLO</h2>";
    this.shadowRoot.appendChild(div);

    // You can add further logic here to interact with the Yew component
  }

  // Add methods, event listeners, and other component logic here
}

customElements.define('custom-component', CustomComponent);

