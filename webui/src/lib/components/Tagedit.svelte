<script lang="ts">
    import { Tag, TagType } from "../scripts/state";
    import { fly, fade } from "svelte/transition";
    import { onMount } from "svelte";
    import Slider from "@smui/slider";
    import Button, { Label } from "@smui/button";
    import Textfield from "@smui/textfield";
    import HelperText from "@smui/textfield/helper-text";
    import { getTranslation } from "../scripts/tagEdit";
    import SegmentedButton, { Segment } from "@smui/segmented-button";
    import Card, { PrimaryAction } from "@smui/card";

    export let tag: Tag;
    let local_tag = new Tag("", "", true, 1, tag.order);
    local_tag.weight_multiplier = tag.weight_multiplier;
    let zh_transition;
    console.log(tag);
    setTimeout(async () => {
        let translation_promise = getTranslation(tag.name);
        zh_transition = (await translation_promise.finally()).data[0][
            "translations"
        ][0]["text"];
    }, 500);

    // we can't know how many values there are in the tag at compile time
    // so, can't create the excat number of bindings for them
    // just limit the max number of values to 3
    let local_value_1 = "";
    let local_value_2 = "";
    let local_value_3 = "";
    let local_value = [local_value_1, local_value_2, local_value_3];

    if (tag.value.length > 3) {
        // todo
    }

    let tag_types = ["NovelAI", "StableDiffusion"];
    let selected = tag.tag_type === TagType.NAI ? "NovelAI" : "StableDiffusion";
</script>

<div class="flex flex-col m-6">
    <div class="mb-2">
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
    <div class="flex flex-row">
        <div class="basis-[85%]">
            <Textfield
                class="shaped-filled w-full"
                variant="filled"
                bind:value={local_tag.name}
                label={tag.name}
            >
                <HelperText slot="helper"
                    >Change tag name have no effect on the tag value itself. Plz
                    change value</HelperText
                >
            </Textfield>
        </div>
        <div class="basis-[15%]">
            <Button
                class="w-full m-2"
                on:click={() => {}}
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
            Replace [{tag.fadeparam.old}] by [{tag.fadeparam._new}] at step:[ {local_tag
                .fadeparam.steps} ]
        </div>
        <Slider
            bind:value={local_tag.fadeparam.steps}
            min={0}
            max={100}
            step={1}
        />
    {/if}

    {#each tag.value as value, i}
        <div class="flex flex-row">
            <div class="basis-3/4">
                <Textfield
                    class="shaped-filled w-full"
                    variant="filled"
                    bind:value={local_value[i]}
                    label={tag.value[i]}
                >
                    <HelperText slot="helper">Tag Value</HelperText>
                </Textfield>
            </div>
            <div class="basis-1/4">
                <Textfield
                    class="shaped-filled w-full ml-2"
                    variant="filled"
                    bind:value={local_value[i]}
                    label={tag.weights[i].toString()}
                >
                    <HelperText slot="helper"
                        >{tag.value[i]}'s Weight</HelperText
                    >
                </Textfield>
            </div>
        </div>
    {/each}
    <div class="flex flex-row">
        <div class="basis-[55%] mt-1">
            <Textfield
                class="shaped-filled w-full"
                variant="filled"
                bind:value={local_tag.order}
                label="Order in queue"
            >
                <HelperText slot="helper">Position in queue</HelperText>
            </Textfield>
        </div>
        <div class="basis-[45%] ml-4">
            <div class="eng text-sm h-fit">tag type</div>
            <SegmentedButton
                segments={tag_types}
                let:segment
                singleSelect
                bind:selected
            >
                <Segment {segment} on:click$preventDefault={() => {}}>
                    <Label>{segment}</Label>
                </Segment>
            </SegmentedButton>
        </div>
    </div>

    <div class="flex flex-row">
        <div class="basis-1/4">
            <Textfield
                class="shaped-filled w-full"
                variant="filled"
                bind:value={local_tag.weight_multiplier}
                label="Weight Multiplier"
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

    <div class="eng text-sm">
        Preview:
    </div>

    <div class="card-container">
        <Card>
            <PrimaryAction on:click={() => {}} padded>
                <div class="title text-center">
                    {tag.raw}
                </div>
            </PrimaryAction>
        </Card>
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
    }
</style>
