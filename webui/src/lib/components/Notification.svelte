<script lang="ts">
    import FaTimes from "svelte-icons/fa/FaTimes.svelte";
    import { each } from "svelte/internal";
    import { message, addMessage, Message } from "../scripts/message";

    let msg: Message;
    let initialized = false;
    let timeoutOutDict = {};
    $: messageArray = [];

    function hideMessage(id: number) {
        for (let i = 0; i < messageArray.length; i++) {
            if (messageArray[i].id === id) {
                messageArray[i].show = false;
                break;
            }
        }
    }

    function checkShowen() {
        messageArray = messageArray.filter((msg) => msg.show);
    }

    message.subscribe((arr) => {
        msg = arr.pop();
        if (!initialized) {
            messageArray = [];
            initialized = true;
            return;
        }
        msg.text = msg.text[0].split("\n");
        messageArray = [...messageArray, msg];
        timeoutOutDict[msg.id] = setTimeout(hideMessage, msg.timeout, msg.id);
        checkShowen();
    });

    function messageHover(id) {
        clearTimeout(timeoutOutDict[id]);
    }
</script>

{#each messageArray as msg, i}
    <!-- svelte-ignore a11y-mouse-events-have-key-events -->
    <div
        class="fixed bg-yellow-200 min-w-[20%] right-0 m-0 pl-10 pt-6 pb-6 pr-3 border-l-[8px] border-l-amber-300 rounded-lg flex flex-row shadow-lg alert {msg.show
            ? 'show'
            : 'hide'}
            hover:shadow-xl hover:bg-yellow-300
            transition-all duration-200 ease-in-out"
        style="top: {i * 80}px;"
        on:mouseover|once={() => messageHover(msg.id)}
    >
        <div class="flex flex-col mr-10">
            {#each msg.text as text, i}
                <div class="text">
                    {msg.text[i]}
                </div>
            {/each}
        </div>

        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <span
            class="absolute h-full top-0 right-0 bg-yellow-400 pt-6 pb-6 pl-4 pr-3 shadow-sm hover:bg-red-500 hover:text-white rounded-lg transition-all duration-200 ease-in-out cursor-pointer"
            on:click={() => {
                hideMessage(msg.id);
            }}
        >
            <FaTimes />
        </span>
    </div>
{/each}

<style>
    .alert.show {
        animation: show_slide 0.8s ease forwards;
    }
    @keyframes show_slide {
        0% {
            transform: translateX(100%);
        }
        40% {
            transform: translateX(-10%);
        }
        80% {
            transform: translateX(0%);
        }
        100% {
            transform: translateX(-10px);
        }
    }
    @keyframes hide_slide {
        0% {
            transform: translateX(-10px);
        }
        100% {
            transform: translateX(100%);
        }
    }

    .alert.hide {
        animation: hide_slide 0.8s ease-in-out forwards;
    }

    .text {
        font-size: 1.2rem;
        font-family: "Noto Sans SC", sans-serif;
    }
</style>
