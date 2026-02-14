import {
  listen,
  type EventCallback,
  type UnlistenFn,
} from "@tauri-apps/api/event";

export class EventWrapper {
  private listenerPromise: Promise<UnlistenFn> | undefined;

  constructor(event: string, callback: EventCallback<unknown>) {
    this.listenerPromise = listen(event, callback);
  }

  async unlisten(): Promise<void> {
    if (!this.listenerPromise) return;
    const unlisten = await this.listenerPromise;
    return unlisten();
  }
}
