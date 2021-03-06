import {argsToAttrs} from "@utils/attributes";
import "@elements/core/tooltips/info";
import {Placement} from "@elements/core/tooltips/base";

export default {
    title: "Core / Tooltips"
}

interface Args {
    title: string,
    body: string,
    placement: Placement,
    margin: number,
    arrowOffset: number,
}

const DEFAULT_ARGS:Args = {
    title: "Title here",
    body: "Body here",
    placement: "top",
    margin: 0,
    arrowOffset: 0,
    //margin: 10,
    //arrowOffset: 35,
}

export const Info = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
    <i>Note: the arrow positioning will only be driven at runtime</i>
    <div id="target" style="position: absolute; top: 50vh; left : 50vw; width: 100px; height: 100px; background-color: black; color: white">
    <div style="position: absolute; top: 0; left: 50px; width: 1px; height: 100px; background-color: yellow"></div>
    <div style="position: absolute; top: 50px; left: 0px; width: 100px; height: 1px; background-color: yellow"></div>
    </div>
    <tooltip-info ${argsToAttrs(props)} showId="debug" target="target" closeable></tooltip-info>
    `;
}

Info.args = DEFAULT_ARGS;
Info.argTypes = {
    placement: {
        control: {
            type: 'inline-radio',
            options: [  "top", "top-start", "top-end", 
                "bottom", "bottom-start" , "bottom-end", 
                "right", "right-start", "right-end", 
                "left", "left-start" , "left-end"
            ]
        }
    }
}
