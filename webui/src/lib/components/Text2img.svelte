<script>
    import Taginput from "./Taginput.svelte";
    import {Usage} from "../scripts/state";
</script>

<Taginput usage={Usage.text2img_positive} />
<Taginput usage={Usage.text2img_negative} />