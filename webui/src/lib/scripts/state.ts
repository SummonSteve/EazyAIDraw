import { writable, get, type Writable } from "svelte/store";
 
class State {
    
}

export enum Usage {
    img2img_positive,
    img2img_negative,
    text2img_positive,
    text2img_negative,
}

class FadeParam {
    old: string;
    _new: string;
    steps: number;
    constructor(old: string, _new: string, steps: number) {
        this.old = old;
        this._new = _new;
        this.steps = steps;
    }
}

export enum TagType {
    NAI,
    SD,
}

export class Tag {
    public name: string;
    public value: string[];
    public name_is_value: boolean;
    // true for positive, false for negative
    public polarity: boolean;
    public order: number;
    public weights: number[];
    public weight_multiplier: number;
    public fadeparam: FadeParam;
    public tag_type: TagType;
    public raw: string;
    constructor(name: string, value: string, name_is_value: boolean, weight: number, order: number){
        this.name = name;
        this.polarity = true;
        this.value = value === "" ? [] : [value];
        this.name_is_value = name_is_value;
        this.weights = weight === 1 ? [] : [weight];
        this.weight_multiplier = 0;
        this.order = order;
        this.fadeparam = new FadeParam("", "", 0);
        this.tag_type = TagType.SD;
        this.raw = "";
    }
}
export const img2img_positive_tags_input = writable("");
export const img2img_negative_tags_input = writable("");

export const text2img_positive_tags_input = writable("");
export const text2img_negative_tags_input = writable("");

export const img2img_positive_tags = writable([]);
export const img2img_negative_tags = writable([]);

export const text2img_positive_tags = writable([]);
export const text2img_negative_tags = writable([]);

export const sync_event_notifier = writable(0);

export function syncTags(curr: Usage) {
    if (curr === Usage.img2img_positive) {
        let tags = JSON.parse(JSON.stringify(get(text2img_positive_tags)));
        img2img_positive_tags.set(tags);
    } else if(curr === Usage.img2img_negative) {
        let tags = JSON.parse(JSON.stringify(get(text2img_negative_tags)));
        img2img_negative_tags.set(tags);
    } else if(curr === Usage.text2img_positive) {
        let tags = JSON.parse(JSON.stringify(get(img2img_positive_tags)));
        text2img_positive_tags.set(tags);
    } else if(curr === Usage.text2img_negative) {
        let tags = JSON.parse(JSON.stringify(get(img2img_negative_tags)));
        text2img_negative_tags.set(tags);
    }
    sync_event_notifier.set(Math.random());
}

export enum SamplingMethods {
    euler_a = "Euler a",
    euler = "Euler",
    lms = "LMS",
    heun = "Heun",
    dpm2 = "DPM2",
    dpm2a = "DPM2a",
    dpm_fast = "DPM fast",
    dpm_adaptive = "DPM adaptive",
    lms_karras = "LMS Karras",
    dpm2_karras = "DPM2 Karras",
    dpm2a_karras = "DPM2a Karras",
    ddim = "DDIM",
    plms = "PLMS",
}

export enum FaceRestoration {
    CodeFormer = "CodeFormer",
    GFPGAN = "GFPGAN",
    None = "None",
}

export class Configuration {
    public steps: number;
    public sampling_method: SamplingMethods;
    public width: number;
    public height: number;
    public cfg_scale: number;
    public seed: number;
    public restore_face: boolean;
    public face_restoration: FaceRestoration;
    public tiling: boolean;
    public highres_fix: boolean;
    public clip_skip: number;
    public denoising_strength: number;
    public model_hash: string;
    public raw: string;

    constructor() {
        this.steps = 20;
        this.sampling_method = SamplingMethods.euler;
        this.width = 512;
        this.height = 512;
        this.cfg_scale = 8;
        this.seed = -1;
        this.restore_face = false;
        this.tiling = false;
        this.highres_fix = false;
        this.clip_skip = 0;
        this.denoising_strength = 0;
        this.model_hash = "";
        this.raw = "";
    }
}

export const sync_config = writable(new Configuration());
