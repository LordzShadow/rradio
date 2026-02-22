<script lang="ts">
    import * as Popover from "$lib/components/ui/popover";
    import { playerState, setVolume } from "$lib/hooks/player-state.svelte";
    import Volume2Icon from "@lucide/svelte/icons/volume-2";
    import Volume1Icon from "@lucide/svelte/icons/volume-1";
    import VolumeOffIcon from "@lucide/svelte/icons/volume-off";
    import { buttonVariants } from "../ui/button";
    import { Slider } from "../ui/slider";
</script>

<Popover.Root>
    <Popover.Trigger
        class={buttonVariants({ variant: "ghost" })}
        openOnHover
        openDelay={200}
        closeDelay={200}
    >
        {#if playerState.volume === 0}
            <VolumeOffIcon></VolumeOffIcon>
        {:else if playerState.volume > 50}
            <Volume2Icon></Volume2Icon>
        {:else}
            <Volume1Icon></Volume1Icon>
        {/if}
    </Popover.Trigger>
    <Popover.Content class="w-12" side="top">
        <div class="flex flex-col items-center gap-2">
            <Slider
                aria-label="Volume"
                type="single"
                orientation="vertical"
                value={playerState.volume}
                onValueChange={setVolume}
                max={100}
                step={1}
            ></Slider>
        </div>
    </Popover.Content>
</Popover.Root>
