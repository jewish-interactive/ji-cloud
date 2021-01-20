import {argsToAttrs} from "@utils/attributes";
import "@elements/core/buttons/circle";

export default {
  title: 'Core / Buttons',
}
interface Args {
    active: boolean,
    disabled: boolean,
    label: string,
    contents: string
}

const DEFAULT_ARGS:Args = {
    active: false,
    disabled: false,
    label: "label here",
    contents: "1"
}

export const Circle = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {contents, ...buttonProps} = props;
    return `<button-circle ${argsToAttrs(buttonProps)}>${contents}</button-circle>`;
}

Circle.args = DEFAULT_ARGS;