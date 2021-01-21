import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

@customElement("input-checkbox")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        label {
          display: flex;
          align-items: baseline;
        }
        input {
          margin-left: 2px;
          margin-right: 1px;
          display: inline-block;
        }
        span {
          margin-left: 12px;
        }
        li {
          list-style-type: none;
          padding: 2px 16px 2px 4px;
        }
        .errorwrapper {
          border: solid 1px #f00813;
          background-color: #fff4f4;
          border-radius: 14px;
          padding: 2px 16px 2px 4px;
          margin-right: 16px;
        }
        div {
          display: flex;
          align-items: center;
          height: 30px;
          margin-left: -2px;
        }
      `,
    ];
  }

  @property()
  label: string = "";

  @property()
  error: string = "";

  render() {
    const { label, error } = this;

    const isError: boolean = error !== "";

    const errorwrapper = isError ? "errorwrapper" : "";

    return html`
      <div>
        <li class="${errorwrapper}">
          <label class="">
            <input type="checkbox" />
            <span class=""> ${label} </span>
          </label>
        </li>
        ${isError ? html`<p class="error">${error}</p>` : nothing}
      </div>
    `;
  }
}
