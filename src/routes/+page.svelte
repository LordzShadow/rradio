<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let url = $state("");
    let playerState = $state("");

    let stations = $state<{ name: string; url: string }[]>();
    invoke("stations").then((m) => (stations = m as any));

    async function play(url: string) {
        playerState = await invoke("play", { url });
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
            <span>{station.name}</span>
            <button
                onclick={() => play(station.url)}
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
</main>

<style lang="css">
</style>
