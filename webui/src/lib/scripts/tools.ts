import { get, writable } from "svelte/store";
import {
    img2img_positive_tags,
    text2img_positive_tags,
    img2img_positive_tags_input,
    text2img_positive_tags_input,
    img2img_negative_tags,
    text2img_negative_tags,
    img2img_negative_tags_input,
    text2img_negative_tags_input,
    sync_config,
    Usage, Tag, TagType, Configuration
} from "./state";
import { addMessage, messageType } from "./message";
import * as base64 from "byte-base64";

import { inflate } from "pako";

const cfgs = ['Steps:', 'Sampler:', 'CFG scale:', 'Seed:', 'Face restoration:', 'Size:', 'Model hash:', 'Clip skip:', 'ENSD:', 'Denoising strength:', 'Highres fix:', 'Tiling:']

export const refresh_event_notifier = writable(0);

export function parsePromptString(usage: Usage) {
    let positiveTagArray: Tag[] = [];
    let negativeTagArray: Tag[] = [];
    let prompt_str = "";
    if (usage === Usage.img2img_positive) {
        prompt_str = get(img2img_positive_tags_input);
    } else if (usage === Usage.text2img_positive) {
        prompt_str = get(text2img_positive_tags_input);
    } else if (usage === Usage.img2img_negative) {
        prompt_str = get(img2img_negative_tags_input);
    } else if (usage === Usage.text2img_negative) {
        prompt_str = get(text2img_negative_tags_input);
    }

    let positive = ""
    let negative = "";
    if (prompt_str.includes("Negative prompt:")) {
        let str_split = prompt_str.split("Negative prompt:");
        positive = str_split[0]
        negative = str_split[1]
        if ((usage === Usage.img2img_positive || usage === Usage.text2img_positive) && positive.length === 0) {
            addMessage("Can't put neagtive prompt in positive prompt", messageType.error, 5000);
            return;
        }

    } else {
        positive = prompt_str;
    }
    if (negative.includes("Steps:")) {
        negative = negative.split("Steps:")[0]
    }
    let cfg_dict = {};
    cfgs.forEach((cfg) => {
        if (prompt_str.includes(cfg)) {
            let _cfg = prompt_str.split(cfg)[1].split(",")[0].trim();
            cfg_dict[cfg] = _cfg;
        }
    })
    try {
        let cfg = paresConfiguration(cfg_dict);
        sync_config.set(cfg);
    } catch (e) { }

    let positive_tags = positive.split(",");
    let negative_tags = negative.split(",");

    if (usage === Usage.img2img_positive || usage === Usage.text2img_positive) {
        positive_tags.forEach((tag, i) => {
            positiveTagArray.push(parseTag(tag, i));
        });

        if (negative_tags.length > 1) {
            negative_tags.forEach((tag, i) => {
                negativeTagArray.push(parseTag(tag, i));
            });
        }

    } else if (usage === Usage.img2img_negative || usage === Usage.text2img_negative) {
        negative_tags.forEach((tag, i) => {
            negativeTagArray.push(parseTag(tag, i));
        });
    }

    if (usage === Usage.img2img_positive) {
        img2img_positive_tags.set(positiveTagArray);
        img2img_negative_tags.set(negativeTagArray);
    } else if (usage === Usage.text2img_positive) {
        text2img_positive_tags.set(positiveTagArray);
        text2img_negative_tags.set(negativeTagArray);
    } else if (usage === Usage.img2img_negative) {
        img2img_negative_tags.set(negativeTagArray);
    } else if (usage === Usage.text2img_negative) {
        text2img_negative_tags.set(negativeTagArray);
    }

    // remove tag name duplicates
    let name_list = [];
    positiveTagArray.forEach((tag, i) => {
        if (name_list.includes(tag.name)) {
            tag.name = tag.name + `#${i}`;
        } else {
            name_list.push(tag.name);
        }
    });
    negativeTagArray.forEach((tag, i) => {
        if (name_list.includes(tag.name)) {
            tag.name = tag.name + `#${i}`;
        } else {
            name_list.push(tag.name);
        }
    });

    refresh_event_notifier.set(Math.random());
}

// Match the three signed floating point numbers in a string with a regular expression.
// cat:1|happy:-0.2|cute:-0.3
// in this case, the weight of cat is 1, happy is -0.2, cute is -0.3
const re_match_number = /(-?\d+(?:\.\d+)?)/g;

export function parseTag(tag_str: string, id: number): Tag {
    let tag = new Tag("", "", true, 1, id);
    tag.raw = tag_str;
    if (tag_str.includes("{")) {
        tag.tag_type = TagType.NAI;
        tag.weight_multiplier = tag_str.split("{").length - 1;
        let colon_count = tag_str.split(":").length - 1;
        if (colon_count === 1) {
            console.warn("Tag " + tag_str + " is not a valid NAI tag. It should have at least two colons.");
        } else if (colon_count > 1) {
            [...tag_str]
                .filter(char => !["{", "}"]
                    .includes(char))
                .join("")
                .split("|")
                .forEach(elem => {
                    tag.name = elem.split(":")[0];
                    tag.value.push(elem.split(":")[0].trim());
                    tag.weights.push(elem.split(":")[1] ? parseFloat(elem.split(":")[1].trim()) : 1)
                })
        } else if (colon_count === 0) {
            tag.name = [...tag_str].filter(char => !["{", "}"].includes(char)).join("")
            tag.value.push(tag.name);
            tag.weights.push(1);
        }

    } else if (tag_str.includes("(")) {
        let bracket_count = tag_str.split("(").length - 1;
        tag.tag_type = TagType.SD;
        tag.weight_multiplier = bracket_count;
        if (bracket_count === 1) {
            [...tag_str]
                .filter(char => !["(", ")"]
                    .includes(char))
                .join("")
                .split("|")
                .forEach(elem => {
                    tag.name = elem.split(":")[0];
                    tag.value.push(elem.split(":")[0].trim());
                    tag.weights.push(elem.split(":")[1] ? parseFloat(elem.split(":")[1].trim()) : 1)
                })
        } else if (tag_str.includes(":") && bracket_count > 1) {
            console.warn("Use multiple bracket and number at the same time has no effect\n using number only");
        } else if (!tag_str.includes(":")) {
            tag.name = [...tag_str].filter(char => !["(", ")"].includes(char)).join("")
            tag.value.push(tag.name);
            tag.weights.push(1);
        }
    } else if (tag_str.includes("[")) {
        let square_bracket_count = tag_str.split("[").length - 1;
        let colon_count = tag_str.split(":").length - 1;
        if (square_bracket_count > 1) {
            console.warn("Nerf prompt only support one [bracket] at a time");
        }
        if (colon_count === 1 || colon_count === 2) {
            let elem = [...tag_str].filter(char => !["[", "]"].includes(char)).join("").split(":")
            tag.name = '[FADE TAG]'
            tag.fadeparam.old = elem.length === 2 ? "" : elem[0];
            tag.fadeparam._new = elem[elem.length - 2];
            tag.fadeparam.steps = parseFloat(elem[elem.length - 1].trim())
        }
        if (colon_count === 0) {
            tag.name = [...tag_str].filter(char => !["[", "]"].includes(char)).join("")
            tag.value.push(tag.name)
            tag.polarity = false;
            tag.weights.push(1);
        }
    } else {
        tag.name = tag_str.trim()
        tag.value.push(tag_str.trim());
        tag.weights.push(1);
    }

    if (tag.value.length !== 1) {
        tag.name_is_value = false;
    }

    if (tag.name.split("\\").length === 3) {
        tag.name = tag.name.replace(/\\/, "(");
        tag.name = tag.name.replace(/\\/, ")");
        if (tag.name_is_value) {
            tag.value[0] = tag.name;
        }
    }

    tag.name = tag.name.trim();
    return tag;
}

export function paresConfiguration(config_dict): Configuration {
    let cfg = new Configuration();
    cfg.steps = parseFloat(config_dict["Steps:"]);
    cfg.cfg_scale = parseFloat(config_dict["CFG scale:"]);
    cfg.height = parseFloat(config_dict["Size:"].split("x")[1]);
    cfg.width = parseFloat(config_dict["Size:"].split("x")[0]);
    cfg.highres_fix = config_dict["Highres fix:"] == 'true';
    cfg.clip_skip = parseFloat(config_dict["Clip skip:"]);
    cfg.denoising_strength = parseFloat(config_dict["Denoising strength:"]);
    cfg.face_restoration = config_dict["Face restoration:"];
    if (cfg.face_restoration) {
        cfg.restore_face = true;
    }
    cfg.model_hash = config_dict["Model hash:"];
    cfg.sampling_method = config_dict["Sampler:"];
    cfg.seed = parseFloat(config_dict["Seed:"]);
    cfg.tiling = config_dict["Tiling:"] == 'true';
    return cfg;
}

export async function parseBlobImageToBase64(blob: Blob): Promise<{id: number, b64: string}> {
    let base64String: string = "data:image/png;base64,";
    let inflated = inflate(await blob.arrayBuffer());
    let id_bytes = inflated.slice(-8);
    let image_bytes = inflated.slice(0, -8);
    let view = new DataView(id_bytes.buffer);
    let id = view.getUint32(0, true);
    base64String += base64.bytesToBase64(image_bytes);
    console.log(base64String);
    return {id: id, b64: base64String};
}