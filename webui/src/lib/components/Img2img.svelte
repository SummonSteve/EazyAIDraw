<script>
    import Taginput from "./Taginput.svelte";
    import {Usage} from "../scripts/state";
</script>

<Taginput usage={Usage.img2img_positive} />
<Taginput usage={Usage.img2img_negative} />