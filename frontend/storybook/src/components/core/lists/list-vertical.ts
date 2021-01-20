import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/lists/list-vertical";
import "@elements/core/lists/li-check";

export default {
    title: "Core / Lists"
}

interface Args {
    label: string,
    count: number,
}

const DEFAULT_ARGS:Args = {
    label: "hello",
    count: 4
}

export const ListVertical = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {count, ...listProps} = props

    return `
    <list-vertical ${argsToAttrs(listProps)}>
        ${mapToString(arrayCount(count), i => {
            return `<li-check>item ${i}</li-check>`;
        })}
    </list-vertical>`;
}

ListVertical.args = DEFAULT_ARGS;