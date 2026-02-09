import type { InvokeArgs } from "@tauri-apps/api/core";
import type { Station } from "./stations";

type CommandDefinition = { payload: InvokeArgs; return: unknown };

interface CommandRegistry {
  play: { payload: { uuid: string }; return: string };
  pause: { payload: never; return: void };
  set_volume: { payload: { volume: number }; return: void };
  get_volume: { payload: never; return: number };
  stations: { payload: never; return: Station[] };
}

// Validation Guard: If CommandRegistry doesn't match the shape,
// this helper will highlight the error.
type ValidateRegistry<T extends Record<keyof T, CommandDefinition>> = T;
type Verified = ValidateRegistry<CommandRegistry>;

export type CommandType = keyof CommandRegistry;

export type CommandPayload<T extends CommandType> =
  CommandRegistry[T]["payload"];

export type CommandResponse<T extends CommandType> =
  CommandRegistry[T]["return"];
