import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/text-pencil";

export default {
    title: "Core / Inputs"
}

interface Args {
  value:string,
  editing: boolean,
  placeholder: string,
  width: number,
}

const DEFAULT_ARGS:Args = {
  value:"hello",
  placeholder: "blah blah",
  editing: false,
  width: 300
}

export const TextPencil = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {width, ...textProps} = props

    return `
        <div style="width:${width}px">
            <input-text-pencil ${argsToAttrs(textProps)}></input-text-pencil>
        </div>
    `;
}

TextPencil.args = DEFAULT_ARGS;