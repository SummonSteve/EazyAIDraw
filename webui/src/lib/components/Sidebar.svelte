

<script>
    import MdApps from 'svelte-icons/md/MdApps.svelte';
    import MdAutorenew from 'svelte-icons/md/MdAutorenew.svelte';
    import MdBurstMode from 'svelte-icons/md/MdBurstMode.svelte';
    import MdSettings from 'svelte-icons/md/MdSettings.svelte';
    import MdAccountCircle from 'svelte-icons/md/MdAccountCircle.svelte';
    import MdInsertChart from 'svelte-icons/md/MdInsertChart.svelte'

</script>

<div class="fixed top-0 left-0 h-screen w-16 m-0 flex flex-col bg-blue-300 text-gray-600 shadow-lg">
    <i class="sidebar-icon group">
        <div class="m-2">
            <span class="sidebar-tooltip group-hover:scale-100">
                Workbench
            </span>
            <MdApps />
        </div>
    </i>
    <i class="sidebar-icon group">
        <div class="m-2">
            <span class="sidebar-tooltip group-hover:scale-100">
                Gallary
            </span>
            <MdBurstMode />
        </div>
    </i>
    <i class="sidebar-icon group">
        <div class="m-2">
            <span class="sidebar-tooltip group-hover:scale-100">
                Settings
            </span>
            <MdSettings />
        </div>
    </i>
    <i class="sidebar-icon group">
        <div class="m-2">
            <span class="sidebar-tooltip group-hover:scale-100">
                User
            </span>
            <MdAccountCircle />
        </div>
    </i>
    <i class="sidebar-icon group">
        <div class="m-2">
            <span class="sidebar-tooltip group-hover:scale-100">
                Sync
            </span>
            <MdAutorenew />
        </div>
    </i>
</div>