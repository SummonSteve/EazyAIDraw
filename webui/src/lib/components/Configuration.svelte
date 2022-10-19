<script lang="ts">
    import type { Tag, Usage } from "../scripts/state";
    import Button, { Group, Label } from "@smui/button";
    import Slider from "@smui/slider";
    import Textfield from "@smui/textfield";
    import HelperText from "@smui/textfield/helper-text";
    import Switch from "@smui/switch";
    import Chip, { Set, Text } from "@smui/chips";

    enum sampling_methods {
        euler_a = "Euler a",
        euler = "Euler",
        lms = "LMS",
        heun = "Heun",
        dpm2 = "DPM2",
        dpm2a = "DPM2a",
        dpm_fast = "DPM fast",
        dpm_adaptive = "DPM adaptive",
        lms_karras = "LMS Karras",
        dpm2_karras = "DPM2 Karras",
        dpm2a_karras = "DPM2a Karras",
        ddim = "DDIM",
        plms = "PLMS",
    }

    let selected_sampling = sampling_methods.euler_a;
    let width: number = 0;
    let height: number = 0;
    let cfg_scale: number = 0;

    let seed: string = "-1";

    let choices_opt = ["restore_faces", "tiling", "highres_fix"];
    let selected = [];

    export let usage: Usage;
</script>

<div class=" bg-slate-200 flex flex-col ml-4 mr-3 mb-5">
    <div class="flex flex-col ml-5 mt-2 mb-3">
        <div class="eng text-sm">
            Sampling method: {selected_sampling}
        </div>
        <Group variant="outlined" class="mt-1">
            {#each Object.keys(sampling_methods) as method}
                <Button
                    on:click={() => {
                        selected_sampling = sampling_methods[method];
                    }}
                    variant="outlined"
                    color={selected_sampling === sampling_methods[method]
                        ? "primary"
                        : "secondary"}
                    style={selected_sampling === sampling_methods[method]
                        ? "background-color: #cb52ff; color: #ffffff;"
                        : "background-color: #f4d9ff;"}
                >
                    <Label>{sampling_methods[method]}</Label>
                </Button>
            {/each}
        </Group>
    </div>
    <div class="flex flex-row mb-4">
        <div class="basis-2/3  bg-slate-300 rounded-sm mt-2 ml-4">
            <div class="flex flex-row mt-2 ml-2">
                <div>
                    <Textfield
                        class="shaped-filled w-full ml-2"
                        variant="filled"
                        bind:value={width}
                        label="Width"
                    >
                    </Textfield>
                </div>
                <div class="basis-3/4 mt-1">
                    <Slider bind:value={width} min={0} max={2048} step={1} />
                </div>
            </div>
            <div class="flex flex-row mt-2 ml-2">
                <div>
                    <Textfield
                        class="shaped-filled w-full ml-2"
                        variant="filled"
                        bind:value={height}
                        label="Height"
                    >
                    </Textfield>
                </div>
                <div class="basis-3/4 mt-1">
                    <Slider bind:value={height} min={0} max={2048} step={1} />
                </div>
            </div>
            <div class="flex flex-row mt-2 ml-2">
                <div>
                    <Textfield
                        class="shaped-filled w-full ml-2"
                        variant="filled"
                        bind:value={cfg_scale}
                        label="Cfg scale"
                    >
                    </Textfield>
                </div>
                <div class="basis-3/4 mt-1 mb-3">
                    <Slider
                        bind:value={cfg_scale}
                        min={0}
                        max={30}
                        step={0.5}
                    />
                </div>
            </div>
        </div>
        <div class="flex flex-col basis-1/3">
            <div
                class="basis-1/4 flex flex-row ml-2 mt-2 mr-4 bg-slate-300 items-center"
            >
                <Set chips={choices_opt} let:chip filter bind:selected>
                    <Chip {chip} touch>
                        <Text>{chip}</Text>
                    </Chip>
                </Set>
            </div>
            <div
                class="basis-3/4 flex flex-row ml-2 mt-2 mr-4 bg-slate-300 items-center"
            >
                <div class="ml-2">
                    <Textfield
                        class="shaped-filled w-full ml-2"
                        variant="filled"
                        bind:value={seed}
                        label="Seed"
                    />
                </div>
                <div class="ml-4">
                    <Button
                        on:click={() => {
                            seed = Math.floor(
                                Math.random() * 1000000
                            ).toString();
                        }}
                        variant="raised"
                    >
                        <Label>Random</Label>
                    </Button>
                </div>
            </div>
        </div>
    </div>
</div>

<style>

    .title {
        font-family: "Jost", sans-serif;
        font-size: xx-large;
    }

    .eng {
        font-family: "Jost", sans-serif;
    }
</style>
