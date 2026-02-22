// TODO: save settings

import { EventWrapper } from "$lib/utils/events";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { setMode } from "mode-watcher";

// THEME
let theme = $state<"light" | "dark" | "system">("system");
let themeEvent = $state<EventWrapper | undefined>();

export const getAppTheme = () => theme;
export const setAppTheme = async (value?: "light" | "dark" | "system") => {
  if (!value) return;
  theme = value;
  if (value === "system") {
    setMode((await getCurrentWindow().theme()) ?? "dark");
    themeEvent = new EventWrapper(
      getCurrentWindow().onThemeChanged((event) => {
        setMode(event.payload);
      }),
    );
  } else {
    setMode(value);
    themeEvent?.unlisten().catch(() => {
      console.error("Failed to unlisten theme event");
    });
  }
};
