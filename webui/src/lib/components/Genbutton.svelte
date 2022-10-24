<script lang="ts">
    let tasks_in_queue = 0;
    let estimated_time = 0;
</script>

<div class="m-3 title">
    <button
        class="text-transparent hover:bg-blue-400 hover:text-white
        rounded-3xl hover:rounded-xl
        transition-all duration-300 ease-in-out
        cursor-pointer h-[5rem] w-80 bg-slate-400 group active:bg-violet-500"
    >
        <div
            class="transition-all duration-100 text-black group-hover:text-white mt-2 group-hover: text-2xl"
        >
            GO!
        </div>
    </button>
</div>

<style>
    .title {
        font-family: "Jost", sans-serif;
        font-size: large;
    }
</style>
