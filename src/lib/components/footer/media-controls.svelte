<script lang="ts">
    import { pause, play, playerState } from "$lib/hooks/player-state.svelte";
    import PauseIcon from "@lucide/svelte/icons/pause";
    import PlayIcon from "@lucide/svelte/icons/play";
    import { Button } from "../ui/button";

    let playing = $derived(playerState.playing);
</script>

{#if playing}
    <Button size="icon" variant="ghost" onclick={() => pause()}>
        <PauseIcon class="w-6 h-6" />
    </Button>
{:else}
    <Button
        size="icon"
        variant="ghost"
        onclick={() => play(playerState.currentStationUuid)}
    >
        <PlayIcon />
    </Button>
{/if}

<style lang="postcss">
</style>
