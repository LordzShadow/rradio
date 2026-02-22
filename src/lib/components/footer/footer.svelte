<script lang="ts">
    import { playerState } from "$lib/hooks/player-state.svelte";
    import MediaControls from "./media-controls.svelte";
    import VolumeControls from "./volume-controls.svelte";
</script>

<div
    class="bg-sidebar sticky bottom-0 w-full h-16 flex justify-between items-center p-2"
>
    <div class="flex items-center gap-2">
        <MediaControls></MediaControls>
        <span class="truncate">{playerState.trackTitle}</span>
    </div>
    <VolumeControls></VolumeControls>
</div>
