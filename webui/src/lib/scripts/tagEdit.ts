import { writable } from "svelte/store";
import axios from 'axios';
import { v4 as uuidv4 } from 'uuid';

const translate_key = "f0b18acddb444aab8dd68b4c06877a15";
const translate_endpoint = "https://api.cognitive.microsofttranslator.com/translate?api-version=3.0&from=en&to=zh-Hans";

const instance = axios.create({
    baseURL: translate_endpoint,
    timeout: 10000,
    headers: {
        'Ocp-Apim-Subscription-Key': translate_key,
        'Content-Type': 'application/json',
        'X-ClientTraceId': uuidv4(),
        'Ocp-Apim-Subscription-Region': 'eastasia',
    }
});

export function getTranslation(text: string) {
    let data = [{
        "text": text
    }];
    return instance.post('', data);
}