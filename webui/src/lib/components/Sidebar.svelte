

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
                菜单
            </span>
            <MdApps />
        </div>
    </i>
    <i class="sidebar-icon group">
        <div class="m-2">
            <span class="sidebar-tooltip group-hover:scale-100">
                服务状态
            </span>
            <MdInsertChart />
        </div>
    </i>
    <i class="sidebar-icon group">
        <div class="m-2">
            <span class="sidebar-tooltip group-hover:scale-100">
                图库
            </span>
            <MdBurstMode />
        </div>
    </i>
    <i class="sidebar-icon group">
        <div class="m-2">
            <span class="sidebar-tooltip group-hover:scale-100">
                设置
            </span>
            <MdSettings />
        </div>
    </i>
    <i class="sidebar-icon group">
        <div class="m-2">
            <span class="sidebar-tooltip group-hover:scale-100">
                用户
            </span>
            <MdAccountCircle />
        </div>
    </i>
    <i class="sidebar-icon group">
        <div class="m-2">
            <span class="sidebar-tooltip group-hover:scale-100">
                强制同步
            </span>
            <MdAutorenew />
        </div>
    </i>
</div>