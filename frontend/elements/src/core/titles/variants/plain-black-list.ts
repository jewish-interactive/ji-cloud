import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('plain-black-list')
export class _ extends LitElement {
  static get styles() {
    return [css`
    p{
        color: #4a4a4a;
        margin-top:0;
        

    }
    .bold {
      font-weight:500;
    }
    .wrapper {
        display:flex;
        flex-direction:column;
    }
    p{
        margin-bottom:0;
    }
    
   
    `];
  }

  @property()
  title:string = ""; 
  @property({type: Boolean})
  bold:boolean = false; 

  render() {

    const {title, bold} = this;

    return html`
    <div class="wrapper">
        <slot></slot>
    </div>
  `;
  }
}