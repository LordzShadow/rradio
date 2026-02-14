import type {
  CommandPayload,
  CommandResponse,
  CommandType,
} from "$lib/types/commands";
import { invoke } from "@tauri-apps/api/core";

export const executeCommand = async <T extends CommandType>(
  type: T,
  ...args: CommandPayload<T> extends never ? [] : [payload: CommandPayload<T>]
): Promise<CommandResponse<T>> => {
  return await invoke(type, args[0]);
};
