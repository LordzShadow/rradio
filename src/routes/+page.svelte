<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { play, playerState } from "$lib/hooks/player-state.svelte";
    import { type Station } from "$lib/types/stations";
    import { executeCommand } from "$lib/utils/executeCommand";

    let stations = $state<Station[]>([]);
    executeCommand("stations").then((m) => (stations = m));
</script>

<div class="flex justify-center gap-4 p-12">
    <div class="flex-1 flex flex-col items-center gap-4">
        {#each stations as station}
            <div class="station">
                <span
                    class={playerState.currentStationUuid === station.uuid
                        ? "text-primary"
                        : ""}>{station.name}</span
                >
                <Button onclick={() => play(station.uuid)} variant="outline">
                    Play
                </Button>
            </div>
        {/each}
    </div>
</div>

<style lang="css">
</style>
