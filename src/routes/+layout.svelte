<script lang="ts">
    import AppSidebar from "$lib/components/app-sidebar.svelte";
    import Footer from "$lib/components/footer/footer.svelte";
    import * as Sidebar from "$lib/components/ui/sidebar";
    import { initPlayerState } from "$lib/hooks/player-state.svelte";
    import { ModeWatcher } from "mode-watcher";
    import "./layout.css";
    import { setAppTheme } from "$lib/hooks/settings.svelte";
    import Spinner from "$lib/components/ui/spinner/spinner.svelte";

    let loadingPlayer = $state(true);
    initPlayerState().finally(() => (loadingPlayer = false));
    setAppTheme("dark"); // Remove once settings are loaded from storage

    const { children } = $props();
</script>

<!--Tracking is done through settings hook -->
<ModeWatcher track={false}></ModeWatcher>
<Sidebar.Provider>
    <AppSidebar />
    {#if loadingPlayer}
        <div class="m-auto flex flex-col gap-2">
            <Spinner class="w-full h-20"></Spinner>
            <span>Loading player...</span>
        </div>
    {:else}
        <main class="w-screen h-screen flex flex-col">
            <Sidebar.Trigger class="md:hidden" />
            <div class="flex-1">
                {@render children()}
            </div>
            <Footer></Footer>
        </main>
    {/if}
</Sidebar.Provider>
