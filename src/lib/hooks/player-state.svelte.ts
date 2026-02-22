import type { PlayerState } from "$lib/types/player";
import { EventWrapper } from "$lib/utils/events";
import { executeCommand } from "$lib/utils/executeCommand";

export const playerState = $state<PlayerState>({
  playing: false,
  volume: 0,
});

const titleChangeEvent = $state<EventWrapper>(
  EventWrapper.fromEvent("title", (event) => {
    playerState.trackTitle = event.payload as string;
  }),
);

const volumeChangeEvent = $state<EventWrapper>(
  EventWrapper.fromEvent("volume_change", (event) => {
    playerState.volume = event.payload as number;
  }),
);

export const initPlayerState = () => {
  executeCommand("get_player_state").then((state) => {
    playerState.currentStationUuid = state.currentStationUuid;
    playerState.playing = state.playing;
    playerState.volume = state.volume;
    playerState.trackTitle = state.trackTitle;
  });
};

export async function play(uuid?: string) {
  if (!uuid) return; // TODO: show alert
  playerState.currentStationUuid = uuid;
  playerState.trackTitle = undefined;
  await executeCommand("play", { uuid }).catch((error) => {
    console.error("Failed to play station:", error);
  });
  playerState.playing = true;
}

export async function pause() {
  await executeCommand("pause").catch((error) => {
    console.error("Failed to pause station:", error);
  });
  playerState.playing = false;
  playerState.trackTitle = undefined;
}

export async function setVolume(vol: number) {
  await executeCommand("set_volume", { volume: vol });
}
