import ReconnectingWebSocket from 'reconnecting-websocket';
import { get, writable } from 'svelte/store';
import { addMessage, Message, messageType } from './message';
import { parseBlobImageToBase64 } from './tools'
 
const ws_url = 'ws://127.0.0.1:2339';
export const images_store = writable([{id: 0, b64: ""}]);

class WsHandler {
    public ws: ReconnectingWebSocket;
    public outgoing_event_queue:Array<any>;
    constructor() {
        this.outgoing_event_queue = [];
        this.ws = new ReconnectingWebSocket(ws_url);
        this.ws.onopen = () => {
            setTimeout(() => {
                addMessage("Websocket connected", messageType.info, 3000);
            }, 1000);
        }

        this.ws.onmessage = async (e: MessageEvent) => {
            if (e.data instanceof Blob) {
                console.log("new image");
                let pic = await parseBlobImageToBase64(e.data)
                images_store.update((images) => {
                    images.push(pic);
                    return images;
                });
            }
        }

        setInterval(() => {
            if (this.outgoing_event_queue.length > 0) {
                console.log(this.outgoing_event_queue);
                this.ws.send(this.outgoing_event_queue.pop());
            }
        }, 100)
    }

    public send(event: any) {
        console.log(event);
        this.outgoing_event_queue.push(event);
    }
}

const handler = new WsHandler();

export function send(event) {
    handler.send(event);
}