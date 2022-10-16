import ReconnectingWebSocket from 'reconnecting-websocket';
import { writable } from 'svelte/store';
import { addMessage, Message, messageType } from './message';

const ws_url = 'ws://127.0.0.1:2334'

enum EventType {

}

class Event {

}

class WsHandler {
    public ws: ReconnectingWebSocket;
    constructor() {
        this.ws = new ReconnectingWebSocket(ws_url)
        this.ws.onopen = () => {
            console.log('ws open')
        }

        this.ws.onmessage = (e) => {
            addMessage(e.data, messageType.info, 3000)
        }

        setInterval(() => {
            this.ws.send('"ping"')

        }, 5000)


    }
}


export { WsHandler }