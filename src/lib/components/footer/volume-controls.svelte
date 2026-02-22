<script lang="ts">
    import * as Popover from "$lib/components/ui/popover";
    import { playerState, setVolume } from "$lib/hooks/player-state.svelte";
    import VolumeIcon from "@lucide/svelte/icons/volume-2";
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
        <VolumeIcon></VolumeIcon>
    </Popover.Trigger>
    <Popover.Content class="w-12" side="top">
        <div class="flex flex-col items-center gap-2">
            <Slider
                aria-labelledby="volume-label"
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
