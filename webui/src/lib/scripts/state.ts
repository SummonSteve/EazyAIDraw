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
    public order: number;
    public weights: number[];
    public weight_multiplier: number;
    public fadeparam: FadeParam;
    public tag_type: TagType;
    public raw: string;
    constructor(name: string, value: string, name_is_value: boolean, weight: number, order: number){
        this.name = name;
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
