import {
  listen as tauriListen,
  type EventCallback,
  type UnlistenFn,
} from "@tauri-apps/api/event";

export class EventWrapper {
  private listenerPromise: Promise<UnlistenFn> | undefined;

  constructor(listenerPromise: Promise<UnlistenFn>) {
    this.listenerPromise = listenerPromise;
  }

  static fromEvent(event: string, callback: EventCallback<unknown>) {
    return new EventWrapper(tauriListen(event, callback));
  }

  async unlisten(): Promise<void> {
    if (!this.listenerPromise) return;
    try {
      const unlisten = await this.listenerPromise;
      unlisten();
      this.listenerPromise = undefined;
    } catch (error) {
      console.error("Failed to unlisten", error);
    }
  }
}
