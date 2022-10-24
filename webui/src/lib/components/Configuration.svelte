<script lang="ts">
    import {
        SamplingMethods,
        type Tag,
        Usage,
        sync_config,
    } from "../scripts/state";
    import Button, { Group, Label } from "@smui/button";
    import Slider from "@smui/slider";
    import Textfield from "@smui/textfield";
    import Genbutton from "./Genbutton.svelte";
    import Switch from "@smui/switch";
    import { onMount } from "svelte";
    import { send } from "../scripts/main";
    import {
        NovelAIDrawCall,
        NovelAISamplingMethods,
    } from "../scripts/novelai";
    import {
        img2img_negative_tags,
        img2img_positive_tags,
        text2img_negative_tags,
        text2img_positive_tags,
    } from "../scripts/state";

    let number_of_generation: number = 1;
    let selected_sampling = SamplingMethods.euler_a;
    let width: number = 0;
    let height: number = 0;
    let cfg_scale: number = 0;
    let steps: number = 0;

    let seed: number = -1;
    let restore_faces: boolean = false;
    let tiling: boolean = false;
    let highres_fix: boolean = false;

    const default_uc =
        "lowres, bad anatomy, bad hands, text, error, missing fingers, extra digit, fewer digits, cropped, worst quality, low quality, normal quality, jpeg artifacts, signature, watermark, username, blurry,";

    export let usage: Usage;

    sync_config.subscribe((cfg) => {
        selected_sampling = cfg.sampling_method;
        width = cfg.width;
        height = cfg.height;
        cfg_scale = cfg.cfg_scale;
        seed = cfg.seed;
        restore_faces = cfg.restore_face;
        tiling = cfg.tiling;
        highres_fix = cfg.highres_fix;
        steps = cfg.steps;
    });

    function submitTask() {
        if (usage == Usage.text2img_positive) {
            let prompt = $text2img_positive_tags
                .sort((a, b) => a.order - b.order)
                .map((tag: Tag) => tag.raw)
                .join(",");

            console.log(steps);
            let draw_call = new NovelAIDrawCall(
                height,
                width,
                number_of_generation,
                prompt,
                selected_sampling,
                cfg_scale,
                seed,
                steps,
                default_uc,
                0
            );
            send(draw_call.into_json());
        }
    }
</script>

<div class=" bg-slate-200 flex flex-col ml-4 mr-3 mb-5">
    <div class="flex flex-col ml-5 mt-2 mb-3">
        <div class="eng text-sm">
            Sampling method: {selected_sampling}
        </div>
        <div class="flex flex-row">
            <div>
                <Group variant="outlined" class="mt-1">
                    {#each Object.keys(SamplingMethods) as method}
                        <Button
                            on:click={() => {
                                selected_sampling = SamplingMethods[method];
                            }}
                            variant="outlined"
                            color={selected_sampling === SamplingMethods[method]
                                ? "primary"
                                : "secondary"}
                            style={selected_sampling === SamplingMethods[method]
                                ? "background-color: #cb52ff; color: #ffffff;"
                                : "background-color: #f4d9ff;"}
                        >
                            <Label>{SamplingMethods[method]}</Label>
                        </Button>
                    {/each}
                </Group>
            </div>
            <div class="flex flex-row">
                <div class="ml-10 eng mt-3">Seed</div>
                <div class="ml-2">
                    <Textfield
                        class="ml-2 h-10"
                        type="number"
                        bind:value={seed}
                    />
                </div>
                <div class="ml-4 mt-1">
                    <Button
                        on:click={() => {
                            seed = Math.floor(Math.random() * 1000000);
                        }}
                        variant="raised"
                    >
                        <Label>Random</Label>
                    </Button>
                </div>
            </div>
        </div>
    </div>
    <div class="flex flex-row mb-4">
        <div class="basis-2/3  bg-slate-300 rounded-sm mt-2 ml-4">
            <div class="flex flex-row">
                <div class="basis-1/2">
                    <div class="flex flex-row mt-2 ml-2">
                        <div class="basis-1/4">
                            <Textfield
                                type="number"
                                class="shaped-filled w-full ml-2"
                                variant="filled"
                                bind:value={width}
                                label="Width"
                            />
                        </div>
                        <div class="basis-3/4 mt-1">
                            <Slider bind:value={width} min={0} max={2048} step={64} />
                        </div>
                    </div>
                    <div class="flex flex-row mt-2 ml-2">
                        <div class="basis-1/4">
                            <Textfield
                                type="number"
                                class="shaped-filled w-full ml-2"
                                variant="filled"
                                bind:value={height}
                                label="Height"
                            />
                        </div>
                        <div class="basis-3/4 mt-1">
                            <Slider bind:value={height} min={0} max={2048} step={64} />
                        </div>
                    </div>
                </div>
                <div class="basis-1/2">
                    <div class="flex flex-row mt-2 ml-2">
                        <div class="basis-1/4">
                            <Textfield
                                type="number"
                                class="shaped-filled w-full ml-2"
                                variant="filled"
                                bind:value={cfg_scale}
                                label="Cfg scale"
                            />
                        </div>
                        <div class="basis-3/4 mt-1">
                            <Slider
                                bind:value={cfg_scale}
                                min={0}
                                max={30}
                                step={0.5}
                            />
                        </div>
                    </div>
                    <div class="flex flex-row mt-2 ml-2">
                        <div class="basis-1/4">
                            <Textfield
                                type="number"
                                class="shaped-filled w-full ml-2"
                                variant="filled"
                                bind:value={steps}
                                label="Steps"
                            />
                        </div>
                        <div class="basis-3/4 mt-1">
                            <Slider bind:value={steps} min={0} max={100} step={1} />
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div class="flex flex-col basis-1/3">
            <div
                class="basis-1/4 flex flex-row ml-2 mt-2 mr-4 bg-slate-300 items-center eng"
            >
                <div class="ml-3">
                    Restore faces
                    <Switch bind:checked={restore_faces} />
                </div>
                <div class="ml-3">
                    Tiling
                    <Switch bind:checked={tiling} />
                </div>
                <div class="ml-3">
                    Highres fix
                    <Switch bind:checked={highres_fix} />
                </div>
            </div>
            <div class="basis-3/4 flex flex-row ml-2 mt-2 mr-4 bg-slate-300">
                <div class="mt-3">
                    <Textfield
                        type="number"
                        class="shaped-filled ml-2"
                        variant="filled"
                        bind:value={number_of_generation}
                        label="Number of Images"
                    />
                    <Slider
                        class=" w-[85%]"
                        bind:value={number_of_generation}
                        min={1}
                        max={16}
                        step={1}
                    />
                </div>
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div on:click={submitTask}>
                    <Genbutton />
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .eng {
        font-family: "Jost", sans-serif;
    }
</style>
