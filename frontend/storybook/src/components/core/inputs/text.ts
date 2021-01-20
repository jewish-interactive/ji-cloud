import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/text";
import {Mode as InputTextMode} from "@elements/core/inputs/text";

export default {
    title: "Core / Inputs"
}

interface Args {
  mode: InputTextMode;
  error: string,
  label:string,
  help:string,
  value:string,
  width:number,
}

const DEFAULT_ARGS:Args = {
  mode: "text",
  error: "Wrong Password",
  label: "Title",
  help: "Minimum 8 digits, Must include a number",
  value:"hello",
  width:300
  
}

export const Text = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {width, ...textProps} = props

    return `
        <div style="width:${width}px">
            <input-text ${argsToAttrs(textProps)}></input-text>
        </div>
    `;
}

Text.args = DEFAULT_ARGS;

Text.argTypes = {
  mode: {
    control: {
      type: 'inline-radio',
      options: ["text", "passwordVisible", "passwordHidden"]
    }
  }
}