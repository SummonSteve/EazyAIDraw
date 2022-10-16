import { writable, get, type Writable } from "svelte/store";
 
class State {
    
}

export class Tag {
    public name: string;
    public value: string;
    public name_is_value: boolean;
    public order: number;
    public weight: number;
    constructor(name: string, value: string, name_is_value: boolean, weight: number, order: number){
        this.name = name;
        this.value = value;
        this.name_is_value = name_is_value;
        this.weight = weight;
        this.order = order;
    }
}

export const sync_event_notifier = writable(0);
export const img2img_tags = writable([]);
export const text2img_tags = writable([]);

export function syncTags(curr: string) {
    if (curr === "img2img") {
        let tags = JSON.parse(JSON.stringify(get(text2img_tags)));
        img2img_tags.set(tags);
    } else {
        let tags = JSON.parse(JSON.stringify(get(img2img_tags)));
        text2img_tags.set(tags);
    }
    sync_event_notifier.set(Math.random());
}
