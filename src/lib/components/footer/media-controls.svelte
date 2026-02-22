<script lang="ts">
    import { pause, play, playerState } from "$lib/hooks/player-state.svelte";
    import PauseIcon from "@lucide/svelte/icons/pause";
    import PlayIcon from "@lucide/svelte/icons/play";
    import { Button } from "../ui/button";
    import { Spinner } from "../ui/spinner";

    let playing = $derived(playerState.playing);
    let loading = $derived(playerState.loading);
</script>

{#if loading}
    <Button size="icon" variant="ghost" disabled aria-label="Loading">
        <Spinner />
    </Button>
{:else if playing}
    <Button
        size="icon"
        variant="ghost"
        onclick={() => pause()}
        aria-label="Pause playback"
    >
        <PauseIcon class="w-6 h-6" />
    </Button>
{:else}
    <Button
        size="icon"
        variant="ghost"
        onclick={() => play()}
        aria-label="Start playback"
    >
        <PlayIcon />
    </Button>
{/if}

<style lang="postcss">
</style>
