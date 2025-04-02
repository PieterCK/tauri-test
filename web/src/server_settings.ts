import { invoke } from "@tauri-apps/api/core";
import { executableDir } from "@tauri-apps/api/path";
import { z } from "zod";

export type ServerConfig = {
  url: string;
  alias: string;
  icon: string;
  zulipVersion: string;
  zulipFeatureLevel: number;
};

const serverConfigSchema = z.object({
  url: z.string().url(),
  alias: z.string(),
  icon: z.string(),
  zulipVersion: z.string().default("unknown"),
  zulipFeatureLevel: z.number().default(0),
});

async function getServerSettings(domain: string): Promise<ServerConfig> {
  try{
    return serverConfigSchema.parse(
        await invoke("get_server_settings", {
          url: domain,
        })
    )
  } except {
    
  };
};
