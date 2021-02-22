import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/sidebar";
import "@elements/entry/jig/edit/sidebar/header";
import "@elements/entry/jig/edit/pages/landing";
import {mapToString, arrayIndex} from "@utils/array";
import {Sidebar} from "../sidebar/sidebar";
import {ModuleSelection} from "../selection/selection";

export default {
    title: "Entry / Jig / Edit / Pages"
}

interface Args {
    nModules: number
}

const DEFAULT_ARGS:Args = {
    nModules: 10
}

export const Landing = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    
    const {nModules} = props;

    return `
        <jig-edit-page>
        ${Sidebar({
            nModules,
            slot: "sidebar"
        })}
        ${ModuleSelection({
            slot: "selection"
        })}
        </jig-edit-page>
    `;
}

Landing.args = DEFAULT_ARGS;
