<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import { useSidebar } from "$lib/components/ui/sidebar/index.js";
    import HouseIcon from "@lucide/svelte/icons/house";
    import PanelLeftCloseIcon from "@lucide/svelte/icons/panel-left-close";
    import PanelLeftOpenIcon from "@lucide/svelte/icons/panel-left-open";
    import SearchIcon from "@lucide/svelte/icons/search";
    import SettingsIcon from "@lucide/svelte/icons/settings";
    import StarIcon from "@lucide/svelte/icons/star";

    // Menu items.
    const items = [
        {
            title: "Home",
            url: "/",
            icon: HouseIcon,
        },
        {
            title: "Search",
            url: "/search",
            icon: SearchIcon,
        },
        {
            title: "Favorites",
            url: "/favorites",
            icon: StarIcon,
        },
        {
            title: "Settings",
            url: "/settings",
            icon: SettingsIcon,
        },
    ];

    const sidebar = useSidebar();
</script>

<Sidebar.Root collapsible="icon">
    <Sidebar.Content>
        <Sidebar.Group>
            <Sidebar.GroupContent>
                <Sidebar.Menu>
                    <Sidebar.MenuItem>
                        <Sidebar.MenuButton>
                            {#snippet child({ props })}
                                <div
                                    on:click={() => sidebar.toggle()}
                                    {...props}
                                >
                                    {#if sidebar.open}
                                        <PanelLeftCloseIcon
                                        ></PanelLeftCloseIcon>
                                    {:else}
                                        <PanelLeftOpenIcon></PanelLeftOpenIcon>
                                    {/if}
                                    <span>Close sidebar</span>
                                </div>
                            {/snippet}
                        </Sidebar.MenuButton>
                    </Sidebar.MenuItem>
                    {#each items as item (item.title)}
                        <Sidebar.MenuItem>
                            <Sidebar.MenuButton>
                                {#snippet child({ props })}
                                    <a href={item.url} {...props}>
                                        <item.icon />
                                        <span>{item.title}</span>
                                    </a>
                                {/snippet}
                            </Sidebar.MenuButton>
                        </Sidebar.MenuItem>
                    {/each}
                </Sidebar.Menu>
            </Sidebar.GroupContent>
        </Sidebar.Group>
    </Sidebar.Content>
</Sidebar.Root>
