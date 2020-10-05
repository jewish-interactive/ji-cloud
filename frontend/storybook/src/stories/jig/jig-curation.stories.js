
import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jigtable from "@templates/admin/jig/jig-table.html";
import jigtooltip from "@templates/admin/jig/jig-copyemail.html";
import jigprivacy from "@templates/admin/jig/jig-privacy.html";
import jigfilter from "@templates/admin/jig/jig-filter.html";
import jiglanguage from "@templates/admin/jig/jig-language-filter.html";
import jigprivacyfilter from "@templates/admin/jig/privacy-filter.html";
import jigstatus from "@templates/admin/jig/jig-status.html";
import jigtofrom from "@templates/admin/jig/jig-tofrom.html";

export default {
  title: 'JIG/Curation',
}


export const JigTable = () =>
    tmpl(jigtable, {

});


export const JigTooltip = () =>  {
    const pageContainer = tmpl(jigtable);

    const pageContents = tmpl(jigtooltip);

    appendId(pageContainer, "tooltip", pageContents);

    return pageContainer;
}

export const JigPrivacy = () =>  {
    const pageContainer = tmpl(jigtable);

    const pageContents = tmpl(jigprivacy);

    appendId(pageContainer, "privacy-dropdown", pageContents);

    return pageContainer;
}

export const JigFilter = () =>  {
    const pageContainer = tmpl(jigtable);

    const pageContents = tmpl(jigfilter);

    appendId(pageContainer, "jig-filter", pageContents);

    return pageContainer;
}

export const JigLanguage = () =>  {
    const pageContainer = tmpl(jigtable);

    const pageContents = tmpl(jiglanguage);

    appendId(pageContainer, "language-filter", pageContents);

    return pageContainer;
}

export const JigPrivacyFilter = () =>  {
    const pageContainer = tmpl(jigtable);

    const pageContents = tmpl(jigprivacyfilter);

    appendId(pageContainer, "privacy-filter", pageContents);

    return pageContainer;
}

export const JigStatus = () =>  {
    const pageContainer = tmpl(jigtable);

    const pageContents = tmpl(jigstatus);

    appendId(pageContainer, "jig-status", pageContents);

    return pageContainer;
}

export const JigToFrom = () =>  {
    const pageContainer = tmpl(jigtable);

    const pageContents = tmpl(jigtofrom);

    appendId(pageContainer, "jig-tofrom", pageContents);

    return pageContainer;
}