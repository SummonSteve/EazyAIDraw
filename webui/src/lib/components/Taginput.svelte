<script>
    
    import { syncTags, Usage } from "../scripts/state";
    import { parsePromptString } from "../scripts/tools";
    import Tags from "./Tags.svelte";
    import {
        text2img_positive_tags,
        img2img_positive_tags_input,
        img2img_positive_tags,
        text2img_positive_tags_input,
        text2img_negative_tags_input,
        img2img_negative_tags_input,
        text2img_negative_tags,
        img2img_negative_tags,
    } from "../scripts/state";
    import { addMessage, messageType } from "../scripts/message";
    let tag = "";
    export let usage;
    function handleTags(event) {
        tag = event.detail.tags;
    }

    let button_enable_class =
        "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded";
    let button_disable_class =
        "bg-blue-500 text-white font-bold py-2 px-4 rounded opacity-50 cursor-not-allowed";

    $: sync_button_disable = () => {
        if (usage == Usage.text2img_positive) {
            return $img2img_positive_tags.length === 0;
        } else if (usage == Usage.img2img_positive) {
            return $text2img_positive_tags.length === 0;
        } else if (usage == Usage.text2img_negative) {
            return $img2img_negative_tags.length === 0;
        } else if (usage == Usage.img2img_negative) {
            return $text2img_negative_tags.length === 0;
        }
    };

    $: parse_button_disable = () => {
        if (usage == Usage.text2img_positive) {
            return $text2img_positive_tags_input.length === 0;
        } else if (usage == Usage.img2img_positive) {
            return $img2img_positive_tags_input.length === 0;
        } else if (usage == Usage.text2img_negative) {
            return $text2img_negative_tags_input.length === 0;
        } else if (usage == Usage.img2img_negative) {
            return $img2img_negative_tags_input.length === 0;
        }
    };
</script>

<div class="m-4 custom-tag max-w-2xl">
    <Tags on:tags={handleTags} autoCompleteKey={"name"} {usage} />
    <div class="mt-2 inline-block">
        <button
            disabled={sync_button_disable()}
            class={sync_button_disable()
                ? button_disable_class
                : button_enable_class}
            on:click={() => {
                syncTags(usage);
            }}
        >
            Sync
        </button>
        <button
            disabled={parse_button_disable()}
            class={parse_button_disable()
                ? button_disable_class
                : button_enable_class}
            on:click={() => {
                if (usage == Usage.text2img_positive) {
                    if ($text2img_positive_tags.length === 0) {
                        parsePromptString(usage);
                    } else {
                        addMessage(
                            "Can't parse input when tags is nonempty",
                            messageType.warning,
                            5000
                        );
                    }
                } else if (usage == Usage.img2img_positive) {
                    if ($img2img_positive_tags.length === 0) {
                        parsePromptString(usage);
                    } else {
                        addMessage(
                            "Can't parse input when tags is nonempty",
                            messageType.warning,
                            5000
                        );
                    }
                }
            }}
        >
            Parse
        </button>
    </div>
</div>

<style>
    .custom-tag :global(.svelte-tags-input-tag) {
        cursor: pointer;
        background-color: rgb(195, 195, 255);
        color: black;
        border-radius: 20px;
        padding-left: 15px;
        padding-right: 10px;
        border-color: rgb(0, 87, 87);
        border-width: 1px;
        transition-property: all;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 300ms;
    }
    .custom-tag :global(.svelte-tags-input-tag:hover) {
        background-color: rgb(121, 121, 241);
        color: aliceblue;
        border-radius: 5px;
        --tw-bg-opacity: 1;
        transition-property: all;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 300ms;
    }
</style>
