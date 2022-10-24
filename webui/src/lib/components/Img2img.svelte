<script lang="ts">
    import Taginput from "./Taginput.svelte";
    import {Usage} from "../scripts/state";
    import { images_store } from "../scripts/main"; 

    let base64: string;
    $: base64;

    images_store.subscribe((images) => {
        base64 = images[images.length - 1].b64;
        console.log(base64);
    });
</script>

<Taginput usage={Usage.img2img_positive} />
<Taginput usage={Usage.img2img_negative} />


<img class="m-20" src="{base64}" alt="ok" />