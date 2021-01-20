import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/register/pages/step3";
import "@elements/core/buttons/rectangle";
import "@elements/core/lists/list-vertical";
import "@elements/core/lists/list-horizontal";
import "@elements/core/inputs/checkbox";
import {Rectangle as RectangleButton} from "~/components/core/buttons/rectangle";
import {AFFILIATION_OPTIONS, AGE_OPTIONS} from "~/mock/meta";
import {mapToString} from "@utils/array";

export default {
  title: 'Entry / User / Register / Pages',
}

const STR_AGE_LABEL = "Which age group are you interested in?";
const STR_AFFILIATION_LABEL = "Content from which streams of Judaism do you want to see?";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Step3 = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-register-step3>
            <card-grey slot="main">
                <list-horizontal label="${STR_AGE_LABEL}">
                ${mapToString(AGE_OPTIONS, label => {
                    return `<input-checkbox label="${label}"></input-checkbox>`
                })}
                </list-horizontal>
            </card-grey>
            <card-grey slot="main">
                <list-vertical label="${STR_AFFILIATION_LABEL}">
                ${mapToString(AFFILIATION_OPTIONS, label => {
                    return `<input-checkbox label="${label}"></input-checkbox>`
                })}
                </list-vertical>
            </card-grey>

            <div slot="submit">${RectangleButton()}</div>
        </page-register-step3>

    `
}

Step3.args = DEFAULT_ARGS;