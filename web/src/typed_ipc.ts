import { invoke as invokeTauri } from "@tauri-apps/api/core";
import { ServerConfig } from "./server_settings";

interface invokeAttr<A, R> {
  args: A;
  returns: R;
}

interface invokeCommands {
  // src/server_settings.rs/get_server_settings()
  get_server_settings: invokeAttr<Record<string, string>, ServerConfig>;
  get_domains: invokeAttr<Record<string, string>, Record<string,ServerConfig>>;
}

export function invoke<T extends keyof invokeCommands>(
  command: T,
  args: invokeCommands[T]["args"]
): Promise<invokeCommands[T]["returns"]> {
  return invokeTauri(command, args);
}
