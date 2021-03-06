import { LitElement, html, css, customElement, property } from "lit-element";
import { imageLib, MediaLibOptions, MediaSizeOptions } from "@utils/path";
import {sameOrigin} from "@utils/image";

@customElement("img-ji")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            img {
                display: inherit;
                width: inherit;
                height: inherit;
                max-height: 100%;
                max-width: 100%;
                object-fit: inherit;
            }
            `,
        ];
    }

      @property({type: Boolean})
      draggable: boolean = true; 

    @property({type: Boolean})
    cacheBust:boolean = false;

    @property()
    lib: MediaLibOptions = "global";

    @property()
    size: MediaSizeOptions = "full";

    //use with cacheBust true to force reloading when id changes to the same thing
    @property({hasChanged: () => true})
    id: string = "";

    onLoad(evt: Event) {
        const img = evt.currentTarget as HTMLImageElement;
        const width = img.naturalWidth;
        const height = img.naturalHeight;

        this.dispatchEvent(
            new CustomEvent("image-load", {
                detail: { width, height },
                bubbles: true,
                composed: true,
            })
        );
    }


    render() {
        const { lib, size, id, cacheBust, draggable } = this;

        let src = imageLib({ lib, size, id });

        if(cacheBust) {
            src += `?cb=${Date.now()}`;
        }

        if (sameOrigin(src)) {
            return html`<img .draggable=${draggable} .src="${src}" @load="${this.onLoad}" ></img>`;
        } else {
            return html`<img .draggable=${draggable} .src="${src}" crossorigin="anonymous" @load="${this.onLoad}" ></img>`;
            }
    }
}
