<script>
    // @ts-nocheck
    import { syncTags } from "../scripts/state";
    import Tags from "../thirdparty/svelte-tags-input/src/Tags.svelte";
    let tag = "";

    function handleTags(event) {
        tag = event.detail.tags;
    }
</script>

<div class="items-start justify-center m-4 mr-5 custom-tag max-w-5xl">
    <div class="flex flex-row">
        <Tags
            on:tags={handleTags}
            autoCompleteKey={"name"}
            use_way="text2img"
        />
        <div class="ml-2 translation">
            <button
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded h-max"
                on:click={() => {
                    syncTags("text2img");
                }}
            >
                Sync
            </button>
        </div>
    </div>
</div>

<style>
    .custom-tag :global(.svelte-tags-input-tag) {
        cursor: pointer;
        background-color: rgb(195, 195, 255);
        color: black;
        border-radius: 20px;
        padding-left: 15px;
        padding-right: 10px;
        border-color: rgb(0, 87, 87);
        border-width: 1px;
        transition-property: all;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 300ms;
    }
    .custom-tag :global(.svelte-tags-input-tag:hover) {
        background-color: rgb(121, 121, 241);
        color: aliceblue;
        border-radius: 5px;
        --tw-bg-opacity: 1;
        transition-property: all;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 300ms;
    }
</style>
