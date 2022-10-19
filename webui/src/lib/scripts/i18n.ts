import { writable, get } from "svelte/store";

enum Language {
    en,
    zhs,
    zht,
}

export const txt_map = writable(new Map());
export const curr_lang = writable(Language.en);

const zhHans = new Map([
    ["@test", "测试"],
    ['@warn-using-confusing-char', '注意：输入包含中文括号'],
]);

const en = new Map([
    ["@test", "测试"],
    ['@warn-using-confusing-char', 'Attention: input contains U+FF08 or U+FF09\n which are confusing with U+0028 and U+0029'],
]);

const zhHant = new Map([
    ["@test", "測試"],
    ['@warn-using-confusing-char', '注意：輸入包含中文括號'],
]);

const langMap = new Map<Language, Map<string, string>>([
    [Language.en, en],
    [Language.zhs, zhHans],
    [Language.zht, zhHant],
]);

export function toggle_language(lang: Language) {
    txt_map.set(langMap.get(lang));
}

export function get_text(key: string) {
    if (key.startsWith("@")) {
        let txt = get(txt_map).get(key);
        if (!txt) {
            let la
            console.warn(`No text found for key ${key} in language ${get(curr_lang)}`);
        }
    }
}
