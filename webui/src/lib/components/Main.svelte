<script>
  import Button, { Group, Label } from "@smui/button";
  import Img2img from "./Img2img.svelte";
  import Text2img from "./Text2img.svelte";
  let clicked = 0;
  let selected = "B";
</script>

<div
  class="fixed top-0 left-16 h-16 w-screen m-0 flex flex-row bg-blue-300 text-gray-600 shadow-lg"
>
  <Group variant="unelevated" class="flex m-3 w-1/2">
    <Button
      on:click={() => (selected = "A")}
      variant="unelevated"
      color={selected === "A" ? "primary" : "secondary"}
      style={selected === "A"
        ? "width: 60%;"
        : "width: 20%; transition: width 0.2s;"}
    >
      <Label>图片转图片</Label>
    </Button>
    <Button
      on:click={() => (selected = "B")}
      variant="unelevated"
      color={selected === "B" ? "primary" : "secondary"}
      style={selected === "B"
        ? "width: 60%;"
        : "width: 20%; transition: width 0.2s;"}
    >
      <Label>文字转图片</Label>
    </Button>
    <Button
      on:click={() => (selected = "C")}
      variant="unelevated"
      color={selected === "C" ? "primary" : "secondary"}
      style={selected === "C"
        ? "width: 60%;"
        : "width: 20%; transition: width 0.2s;"}
    >
      <Label>图片放大</Label>
    </Button>
  </Group>
</div>

<div class="fixed top-16 left-16">
  {#if selected === "A"}
    <div>
      <Img2img />
    </div>
  {:else if selected === "B"}
    <div>
      <Text2img />
    </div>
  {:else if selected === "C"}
    <div>C</div>
  {/if}
</div>
