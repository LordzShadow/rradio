<script>
    import { Button } from "$lib/components/ui/button";
    import * as ToggleGroup from "$lib/components/ui/toggle-group/index";
    import { getAppTheme, setAppTheme } from "$lib/hooks/settings.svelte";
    import MoonIcon from "@lucide/svelte/icons/moon";
    import SunIcon from "@lucide/svelte/icons/sun";
    import { toggleMode } from "mode-watcher";
</script>

<h1>Settings</h1>

<ToggleGroup.Root
    type="single"
    bind:value={getAppTheme, setAppTheme}
    variant="outline"
>
    <ToggleGroup.Item value="dark">
        <MoonIcon class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0" />
    </ToggleGroup.Item>
    <ToggleGroup.Item value="light">
        <SunIcon class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0" />
    </ToggleGroup.Item>
    <ToggleGroup.Item value="system">Auto</ToggleGroup.Item>
</ToggleGroup.Root>
