<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Slider } from "$lib/components/ui/slider";
    import { type Station } from "$lib/types/stations";
    import { executeCommand } from "$lib/utils/executeCommand";
    import { EventWrapper } from "$lib/utils/events";
    import { onDestroy } from "svelte";

    let playerState = $state("");
    let playing = $state("");
    let currentStationUuid = $state("");
    let volume = $state<number>(0);

    let stations = $state<Station[]>([]);
    executeCommand("stations").then((m) => (stations = m));
    executeCommand("get_volume").then((vol) => (volume = vol as number));

    let titleChangeEvent = $state<EventWrapper>(
        new EventWrapper("title", (event) => {
            playing = event.payload as string;
        }),
    );

    let volumeChangeEvent = $state<EventWrapper>(
        new EventWrapper("volume_change", (event) => {
            volume = event.payload as number;
        }),
    );

    async function play(uuid: string) {
        currentStationUuid = uuid;
        playerState = await executeCommand("play", { uuid });
    }

    async function pause(event: Event) {
        event.preventDefault();

        await executeCommand("pause");
        playerState = "Paused";
        playing = "";
    }

    async function setVolume(vol: number) {
        volume = vol;
        await executeCommand("set_volume", { volume: vol });
    }

    onDestroy(async () => {
        await titleChangeEvent.unlisten();
        await volumeChangeEvent.unlisten();
    });
</script>

<div class="flex justify-center gap-4 p-12">
    <div class="flex-1 flex flex-col items-center gap-4">
        {#each stations as station}
            <div class="station">
                <span
                    class={currentStationUuid === station.uuid
                        ? "text-blue-500"
                        : ""}>{station.name}</span
                >
                <Button onclick={() => play(station.uuid)} variant="outline">
                    Play
                </Button>
            </div>
        {/each}
        <form class="row" onsubmit={pause}>
            <Button type="submit" variant="outline">Pause</Button>
        </form>
        <p>{playerState}</p>
        <p>Playing: {playing || "-"}</p>
    </div>
    <div class="flex flex-col items-center w-12">
        <label for="volume">{volume}</label>
        <Slider
            id="volume"
            type="single"
            orientation="vertical"
            value={volume}
            onValueChange={setVolume}
            max={100}
            step={1}
        ></Slider>
    </div>
</div>

<style lang="css">
</style>
