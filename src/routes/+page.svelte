<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Slider } from "$lib/components/ui/slider";
    import { type Station } from "$lib/types/stations";
    import { executeCommand } from "$lib/utils/commands";
    import { listen } from "@tauri-apps/api/event";

    let playerState = $state("");
    let playing = $state("");
    let currentStationUuid = $state("");
    let volume = $state<number>();

    let stations = $state<Station[]>();
    executeCommand("stations").then((m) => (stations = m));
    executeCommand("get_volume").then((vol) => (volume = vol as number));

    listen("title", (event) => {
        playing = event.payload as string;
    });

    listen("volume_change", (event) => {
        volume = event.payload as number;
    });

    async function play(uuid: string) {
        currentStationUuid = uuid;
        playerState = await executeCommand("play", { uuid });
    }

    async function pause(event: Event) {
        event.preventDefault();

        await executeCommand("pause");
        playerState = "Paused";
    }

    $effect(() => {
        if (volume !== undefined) {
            setVolume(volume);
        }
    });

    async function setVolume(vol: number) {
        await executeCommand("set_volume", { volume: vol });
    }
</script>

<main
    class="w-screen h-screen flex flex-col items-center justify-center gap-4 p-12"
>
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
    <p>{playerState} <span>{volume}</span></p>
    <p>Playing: {playing}</p>
    <div class="w-full">
        <Slider type="single" bind:value={volume} max={100} step={1}></Slider>
    </div>
</main>

<style lang="css">
</style>
