<script lang="ts">
    import Taginput from "./Taginput.svelte";
    import statefulSwap from "../scripts/statefulSwap";
    import Tagedit from "./Tagedit.svelte";
    import { fly } from "svelte/transition";
    import Configuration from "./Configuration.svelte";
    import {
        Usage,
        text2img_negative_tags,
        text2img_positive_tags,
        Tag,
    } from "../scripts/state";

    const { onOutro, transitionTo, state } = statefulSwap("none");

    let tag: Tag;
    let _toggle = false;
    function handleTags(index: number, usage: Usage) {
        if (usage == Usage.text2img_positive) {
            tag = $text2img_positive_tags.find((t) => t.order == index);
            if (_toggle) {
                transitionTo("tag1");
                _toggle = !_toggle;
            } else {
                transitionTo("tag2");
                _toggle = !_toggle;
            }
        } else if (usage == Usage.text2img_negative) {
            tag = $text2img_negative_tags.find((t) => t.order == index);
            if (_toggle) {
                transitionTo("tag1");
                _toggle = !_toggle;
            } else {
                transitionTo("tag2");
                _toggle = !_toggle;
            }
        }
    }
</script>

<div class="flex flex-col">
    <div class="flex flex-row ">
        <div class="basis-1/2">
            <Taginput
                usage={Usage.text2img_positive}
                on:tagclick={(e) => {
                    handleTags(e.detail, Usage.text2img_positive);
                }}
            />
            <Taginput
                usage={Usage.text2img_negative}
                on:tagclick={(e) => {
                    handleTags(e.detail, Usage.text2img_negative);
                }}
            />
        </div>

        <div
            class="m-3 bg-slate-200 basis-1/2 hover:shadow-lg transition-all duration-300 ease-in-out"
        >
            {#if $state == "none"}
                <div
                    class="flex flex-col justify-center items-center h-full"
                    in:fly={{ y: -20 }}
                    out:fly={{ y: -20 }}
                    on:outroend={onOutro}
                >
                    Click on a tag to see its details
                </div>
            {:else if $state == "tag1"}
                <div
                    in:fly={{ y: -20 }}
                    out:fly={{ y: -20 }}
                    on:outroend={onOutro}
                >
                    <Tagedit {tag} />
                </div>
            {:else if $state == "tag2"}
                <div
                    in:fly={{ y: -20 }}
                    out:fly={{ y: -20 }}
                    on:outroend={onOutro}
                >
                    <Tagedit {tag} />
                </div>
            {/if}
        </div>
    </div>

    <div>
        <Configuration usage={Usage.text2img_positive} />
    </div>
</div>
