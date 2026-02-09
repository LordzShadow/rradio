<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    let playerState = $state("");
    let playing = $state("");
    let currentStationUuid = $state("");

    let stations = $state<{ name: string; url: string; uuid: string }[]>();
    invoke("stations").then((m) => (stations = m as any));

    listen("title", (event) => {
        playing = event.payload as string;
    });

    async function play(uuid: string) {
        currentStationUuid = uuid;
        playerState = await invoke("play", { uuid });
    }

    async function pause(event: Event) {
        event.preventDefault();

        await invoke("pause");
        playerState = "Paused";
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
    <p>{playerState}</p>
    <p>Playing: {playing}</p>
</main>

<style lang="css">
</style>
