import type { InvokeArgs } from "@tauri-apps/api/core";
import type { Station } from "./stations";

type CommandDefinition = { payload: InvokeArgs; return: unknown };

interface CommandRegistry extends Record<string, CommandDefinition> {
  play: { payload: { uuid: string }; return: string };
  pause: { payload: never; return: void };
  set_volume: { payload: { volume: number }; return: void };
  get_volume: { payload: never; return: number };
  stations: { payload: never; return: Station[] };
}

export type CommandType = keyof CommandRegistry;

export type CommandPayload<T extends CommandType> =
  CommandRegistry[T]["payload"];

export type CommandResponse<T extends CommandType> =
  CommandRegistry[T]["return"];
