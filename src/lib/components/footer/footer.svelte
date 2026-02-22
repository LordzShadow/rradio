<script lang="ts">
    import { playerState } from "$lib/hooks/player-state.svelte";
    import MediaControls from "./media-controls.svelte";
    import VolumeControls from "./volume-controls.svelte";
</script>

<div
    class="bg-sidebar sticky bottom-0 w-full h-16 flex justify-between items-center p-2"
>
    <div class="flex-1 flex items-center gap-2">
        <MediaControls></MediaControls>
        {#if playerState.trackTitle && !playerState.loading}
            <span class="truncate max-w-1/2 hidden xs:block">
                {playerState.trackTitle}
            </span>
        {/if}
    </div>
    <VolumeControls></VolumeControls>
</div>
