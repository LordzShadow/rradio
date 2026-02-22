<script>
    import * as ToggleGroup from "$lib/components/ui/toggle-group/index";
    import { getAppTheme, setAppTheme } from "$lib/hooks/settings.svelte";
    import MoonIcon from "@lucide/svelte/icons/moon";
    import SunIcon from "@lucide/svelte/icons/sun";
</script>

<h1>Settings</h1>

<ToggleGroup.Root
    type="single"
    bind:value={getAppTheme, setAppTheme}
    variant="outline"
>
    <ToggleGroup.Item value="dark">
        <MoonIcon
            aria-label="Dark Mode"
            class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0"
        />
    </ToggleGroup.Item>
    <ToggleGroup.Item value="light">
        <SunIcon
            aria-label="Light Mode"
            class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0"
        />
    </ToggleGroup.Item>
    <ToggleGroup.Item value="system">Auto</ToggleGroup.Item>
</ToggleGroup.Root>
