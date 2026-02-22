export interface PlayerState {
  currentStationUuid?: string;
  volume: number;
  playing: boolean;
  trackTitle?: string;

  loading?: boolean;
}
