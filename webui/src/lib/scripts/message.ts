import { writable } from "svelte/store";

enum messageType{
    info,
    error,
    warning
}

class Message {
    id: number;
    type: messageType;
    text: string[];
    timeout: number;
    show: boolean;
    constructor(type: messageType, text: string, timeout: number){
        this.id = Math.floor(Math.random() * 1000000);
        this.type = type;
        this.text = [text];
        this.timeout = timeout;
        this.show = true;
    }
}

export const message = writable([new Message(messageType.info, 'init', 0)]);

function addMessage(msg: string, type: messageType, timeout: number) {
    message.update((messages) => {
        messages.push(new Message(type, msg, timeout));
        return messages;
    });
}

function clearMessage() {
    message.set([]);
}


export {addMessage, clearMessage, Message, messageType};