<script>
  import Button, { Group, Label } from "@smui/button";
  import Img2img from "./Img2img.svelte";
  import Text2img from "./Text2img.svelte";
  import statefulSwap from "../scripts/statefulSwap";
  import {fly, scale} from "svelte/transition"
  const {onOutro, transitionTo, state} = statefulSwap("second")
  let selected = "second"
</script>

<div class="ml-16 mt-20">
  {#if $state === "first"}
    <div
      in:fly={{y:-20}} out:fly={{y:-20}} on:outroend={onOutro}
    >
      <Img2img />
    </div>
  {:else if $state === "second"}
    <div class="relative"
      in:fly={{y:-20}} out:fly={{y:-20}} on:outroend={onOutro}
    >
      <Text2img />
    </div>
  {:else if $state === "third"}
    <div
      in:fly={{y:-20}} out:fly={{y:-20}} on:outroend={onOutro}
    >C</div>
  {/if}
</div>

<div
  class="fixed top-0 left-16 h-16 w-screen m-0 flex flex-row bg-blue-300 text-gray-600 shadow-lg"
>
  <Group variant="unelevated" class="flex m-3 w-1/2">
    <Button
      on:click={() => {
        transitionTo("first")
        selected = "first"
      }}
      variant="unelevated"
      color={selected === "first" ? "primary" : "secondary"}
      style={selected === "first"
        ? "width: 60%;"
        : "width: 20%; transition: width 0.2s;"}
    >
      <Label>图片转图片</Label>
    </Button>
    <Button
      on:click={() => {
        transitionTo("second")
        selected = "second"
      }}
      variant="unelevated"
      color={selected === "second" ? "primary" : "secondary"}
      style={selected === "second"
        ? "width: 60%;"
        : "width: 20%; transition: width 0.2s;"}
    >
      <Label>文字转图片</Label>
    </Button>
    <Button
      on:click={() => {
        transitionTo("third")
        selected = "third"
      }}
      variant="unelevated"
      color={selected === "third" ? "primary" : "secondary"}
      style={selected === "third"
        ? "width: 60%;"
        : "width: 20%; transition: width 0.2s;"}
    >
      <Label>图片放大</Label>
    </Button>
  </Group>
</div>