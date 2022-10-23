<script lang="ts">
    import { Tag, TagType, Usage } from "../scripts/state";
    import { fly, fade } from "svelte/transition";
    import { onMount } from "svelte";
    import Slider from "@smui/slider";
    import Button, { Label } from "@smui/button";
    import Textfield from "@smui/textfield";
    import HelperText from "@smui/textfield/helper-text";
    import { getTranslation } from "../scripts/tagEdit";
    import SegmentedButton, { Segment } from "@smui/segmented-button";
    import Card, { PrimaryAction } from "@smui/card";
    import {
        img2img_negative_tags,
        img2img_positive_tags,
        text2img_negative_tags,
        text2img_positive_tags,
    } from "../scripts/state";

    export let tag: Tag;
    export let usage: Usage;
    let preview_string: string = "";
    $: {
        preview_string = "";

        for (let i = 0; i < local_value.length; i++) {
            if(local_value[i].name != ""){
                if (local_value[i].weight == 1) {
                    preview_string += `${local_value[i].name}|`;
                } else {
                    preview_string += `${local_value[i].name}:${local_value[i].weight}|`;
                }
            }
        }
        preview_string = preview_string.slice(0, -1);
        preview_string.trim();

        for (let i = 0; i < local_tag.weight_multiplier; i++) {
            if (local_tag.tag_type === TagType.NAI) {
                preview_string = `{${preview_string}}`;
            } else {
                preview_string = `(${preview_string})`;
            }
        }
        if (!local_tag.polarity) {
            if (local_tag.weight_multiplier === 0) {
                preview_string = `[${preview_string}]`;
            }
            if (local_tag.tag_type === TagType.NAI) {
                preview_string = preview_string.replaceAll("{", "[");
                preview_string = preview_string.replaceAll("}", "]");
            } else {
                preview_string = preview_string.replaceAll("(", "[");
                preview_string = preview_string.replaceAll(")", "]");
            }
        }
        if (local_tag.fadeparam.steps != 0) {
            preview_string = `[${
                local_tag.fadeparam.old === ""
                    ? tag.fadeparam.old
                    : local_tag.fadeparam.old
            }:${
                local_tag.fadeparam._new === ""
                    ? tag.fadeparam._new
                    : local_tag.fadeparam._new
            }:${local_tag.fadeparam.steps}]`;
        }
    }
    let local_tag = new Tag("", "", true, 1, tag.order);
    local_tag.fadeparam = JSON.parse(JSON.stringify(tag.fadeparam));
    local_tag.polarity = tag.polarity;
    local_tag.weight_multiplier = tag.weight_multiplier;
    let zh_transition;
    setTimeout(async () => {
        let translation_promise = getTranslation(tag.name);
        zh_transition = (await translation_promise.finally()).data[0][
            "translations"
        ][0]["text"];
    }, 500);

    // we can't know how many values there are in the tag at compile time
    // so, can't create the excat number of bindings for them
    // just limit the max number of values to 3
    let local_value_1 = { name: "", weight: 1 };
    let local_value_2 = { name: "", weight: 1 };
    let local_value_3 = { name: "", weight: 1 };
    let local_value = [local_value_1, local_value_2, local_value_3];

    if (tag.value.length <= 3) {
        for (let i = 0; i < tag.value.length; i++) {
            local_value[i].name = tag.value[i];
            local_value[i].weight = tag.weights[i];
        }
    }

    let tag_types = ["NovelAI", "Stable Diffusion"];
    let tag_polarity = ["Positive", "Negative"];
    $: tag_type_selected =
        local_tag.tag_type === TagType.NAI ? "NovelAI" : "Stable Diffusion";
    $: tag_polarity_selected = local_tag.polarity ? "Positive" : "Negative";

    function saveTag() {
        if (local_tag.name === "") {
            local_tag.name = tag.name;
        }
        for (let i = 0; i < local_value.length; i++) {
            if (local_value[i].name !== "") {
                local_tag.value[i] = local_value[i].name;
                local_tag.weights[i] = local_value[i].weight;
            }
        }

        if (local_tag.name === local_value_1.name  && local_value_2.name === "" && local_value_3.name === "") {
            local_tag.name_is_value = true;
        } else {
            local_tag.name_is_value = false;
        }
        local_tag.raw = preview_string;
        
        if (usage === Usage.text2img_positive) {
            $text2img_positive_tags = $text2img_positive_tags.filter(
                (t) => t.name !== tag.name
            );
            $text2img_positive_tags.push(local_tag);
        } else if (usage === Usage.text2img_negative) {
            $text2img_negative_tags = $text2img_negative_tags.filter(
                (t) => t.name !== tag.name
            );
            $text2img_negative_tags.push(local_tag);
        } else if (usage === Usage.img2img_positive) {
            $img2img_positive_tags = $img2img_positive_tags.filter(
                (t) => t.name !== tag.name
            );
            $img2img_positive_tags.push(local_tag);
        } else if (usage === Usage.img2img_negative) {
            $img2img_negative_tags = $img2img_negative_tags.filter(
                (t) => t.name !== tag.name
            );
            $img2img_negative_tags.push(local_tag);
        }
    }
    console.log(tag.raw);
</script>

<div class="flex flex-col m-6">
    <div class="mb-2 flex flex-row">
        <div class="">
            <div class="title">
                {tag.name}
            </div>
            <div class="ml-2 mt-2">
                {#if zh_transition}
                    {zh_transition}
                {:else}
                    Loading...
                {/if}
            </div>
        </div>
    </div>
    <div class=" bg-slate-300 p-4 rounded-xl">
        <div class="flex flex-row">
            <div class="basis-[85%]">
                <Textfield
                    class="shaped-filled w-full mt-2"
                    variant="filled"
                    bind:value={local_tag.name}
                    label={tag.name}
                >
                    <HelperText slot="helper"
                        >Change tag name have no effect on the tag value itself.
                        Plz change value</HelperText
                    >
                </Textfield>
            </div>
            <div class="basis-[15%]">
                <Button
                    class="w-full ml-2 h-14"
                    on:click={saveTag}
                    touch
                    variant="raised"
                >
                    <Label>Save</Label>
                </Button>
            </div>
        </div>

        {#if tag.fadeparam.steps !== 0}
            <Textfield
                class="shaped-filled w-full"
                variant="filled"
                bind:value={local_tag.fadeparam._new}
                label={tag.fadeparam._new}
            >
                <HelperText slot="helper">New prompt</HelperText>
            </Textfield>
            <Textfield
                class="shaped-filled w-full"
                variant="filled"
                bind:value={local_tag.fadeparam.old}
                label={tag.fadeparam.old}
            >
                <HelperText slot="helper">Prompt to replace</HelperText>
            </Textfield>
            <div class="filled">
                Replace [{tag.fadeparam.old}] by [{tag.fadeparam._new}] at
                step:[ {local_tag.fadeparam.steps} ]
            </div>
            <Slider
                bind:value={local_tag.fadeparam.steps}
                min={1}
                max={100}
                step={1}
            />
        {/if}

        {#each tag.value as value, i}
            <div class="flex flex-row mt-2">
                <div class="basis-3/4">
                    <Textfield
                        class="shaped-filled w-full"
                        variant="filled"
                        bind:value={local_value[i].name}
                        label={tag.value[i]}
                    >
                        <HelperText slot="helper">Tag value</HelperText>
                    </Textfield>
                </div>
                <div class="basis-1/4">
                    <Textfield
                        class="shaped-filled w-full ml-2"
                        variant="filled"
                        type="number"
                        step="0.1"
                        bind:value={local_value[i].weight}
                        label={tag.weights[i].toString()}
                    >
                        <HelperText slot="helper"
                            >Prompt weight</HelperText
                        >
                    </Textfield>
                </div>
            </div>
        {/each}
        <div class="flex flex-row">
            <div class="basis-[35%] mt-3">
                <Textfield
                    class="shaped-filled w-full"
                    variant="filled"
                    type="number"
                    bind:value={local_tag.order}
                    label="Order in queue"
                >
                    <HelperText slot="helper">Position in queue</HelperText>
                </Textfield>
            </div>
            <div class="basis-[65%] ml-4">
                <div class="flex flex-row">
                    <div>
                        <div class="eng">Tag type</div>
                        <SegmentedButton
                            segments={tag_types}
                            let:segment
                            singleSelect
                            bind:selected={tag_type_selected}
                        >
                            <Segment
                                {segment}
                                on:click$preventDefault={() => {
                                    local_tag.tag_type =
                                        segment === "NovelAI"
                                            ? TagType.NAI
                                            : TagType.SD;
                                }}
                            >
                                <Label>{segment}</Label>
                            </Segment>
                        </SegmentedButton>
                    </div>
                    <div class="ml-2">
                        <div class="eng">Tag polarity</div>
                        <SegmentedButton
                            segments={tag_polarity}
                            let:segment
                            singleSelect
                            bind:selected={tag_polarity_selected}
                        >
                            <Segment
                                {segment}
                                on:click$preventDefault={() => {
                                    local_tag.polarity =
                                        segment === "Positive" ? true : false;
                                }}
                            >
                                <Label>{segment}</Label>
                            </Segment>
                        </SegmentedButton>
                    </div>
                </div>
            </div>
        </div>

        <div class="flex flex-row">
            <div class="basis-1/4">
                <Textfield
                    class="shaped-filled w-full"
                    type="number"
                    variant="filled"
                    bind:value={local_tag.weight_multiplier}
                    label="Weight multiplier"
                >
                    <HelperText slot="helper" />
                </Textfield>
            </div>
            <div class="basis-3/4 mt-1">
                <Slider
                    bind:value={local_tag.weight_multiplier}
                    min={0}
                    max={10}
                    step={1}
                />
            </div>
        </div>

        <div class="eng text-sm">Preview:</div>

        <div class="card-container">
            <Card>
                <PrimaryAction on:click={() => {}} padded>
                    <div class="title text-center">
                        {preview_string}
                    </div>
                </PrimaryAction>
            </Card>
        </div>
    </div>
</div>

<style>
    .title {
        font-family: "Jost", sans-serif;
        font-size: xx-large;
    }
    .filled {
        font-family: "Montserrat", sans-serif;
    }
    .eng {
        font-family: "Jost", sans-serif;
        font-size: small;
    }
</style>
