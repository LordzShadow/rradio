<script lang="ts">
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

    async function setVolume(vol: number) {
        await executeCommand("set_volume", { volume: vol });
    }
</script>

<main class="w-screen h-screen flex flex-col items-center justify-center gap-4">
    {#each stations as station}
        <div class="station">
            <span
                class={currentStationUuid === station.uuid
                    ? "text-blue-500"
                    : ""}>{station.name}</span
            >
            <button
                onclick={() => play(station.uuid)}
                class="border px-2 border-stone-300 rounded bg-slate-300"
                >Play</button
            >
        </div>
    {/each}
    <form class="row" onsubmit={pause}>
        <button
            type="submit"
            class="border px-2 border-stone-300 rounded bg-slate-300"
            >Pause</button
        >
    </form>
    <p>{playerState} <span>{volume}</span></p>
    <p>Playing: {playing}</p>
    <div>
        <button onclick={() => setVolume((volume ?? 50) - 5)}>-</button>
        <button onclick={() => setVolume((volume ?? 50) + 5)}>+</button>
    </div>
</main>

<style lang="css">
</style>
