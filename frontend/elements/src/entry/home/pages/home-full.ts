 import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('homepage-full')
export class _ extends LitElement {
  static get styles() {
    return [css`

 
    `];
  }



  render() {

    const {} = this;

    return html`
    <main>
        <slot></slot>
   
    </main>
  `;
  }
}