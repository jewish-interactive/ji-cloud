import {getMediaUrl, getMediaUrl_UI, getMediaUrl_UPLOADS} from "@project-config";

const isDev = process.env["NODE_ENV"] === "development";
const deployTarget = process.env["DEPLOY_TARGET"] || process.env["STORYBOOK_DEPLOY_TARGET"];

export const MEDIA_BASE = getMediaUrl(isDev);
export const MEDIA_UI = getMediaUrl_UI(isDev);
export const MEDIA_UPLOADS = getMediaUrl_UPLOADS(deployTarget);

export const mediaUi = (path:string):string => `${MEDIA_UI}/${path}`;

const mediaUploads = (path:string):string => `${MEDIA_UPLOADS}/${path}`;

export type MediaLibOptions = "global" | "user" | "web" | "mock";
export type MediaSizeOptions = "original" | "full" | "thumb";

const imagePrefix = (lib:MediaLibOptions):string => {
    switch(lib) {
        case "mock": return "mock";
        case "global": return "media/global";
        case "user": return "media/user";
        case "web": return "media/web";
		default: return "";
    }
}

const audioPrefix = (lib:MediaLibOptions):string => {
    switch(lib) {
        case "global": return "audio/global";
        case "user": return "audio/user";
        case "web": return "audio/web";
		default: return "";
    }
}

const sizeVariant = (size:MediaSizeOptions):string => {
    switch(size) {
        case "original": return "original";
        case "full": return "resized";
        case "thumb": return "thumbnail";
		default: return "";
    }
}

export const imageLib = ({lib, size, id}:{lib: MediaLibOptions, size: MediaSizeOptions, id: string}) => {
    const prefix = imagePrefix(lib);
    const variant = sizeVariant(size);

    return lib === "mock" 
        ? mediaUi(`${prefix}/${variant}/${id}`)
        : mediaUploads(`${prefix}/${id}/${variant}.png`);
}

interface LegacyMedia {
    jigId: string,
    moduleId: string,
    path: string
}
export const legacyMock = ({jigId, moduleId, path}:LegacyMedia):string => 
    `${MEDIA_BASE}/legacy/examples/${jigId}/slides/${moduleId}/${path}`;

export const legacyMedia = (_props?:LegacyMedia):string => 
    `TODO`
