import { invoke } from "./typed_ipc.ts";
import { formatUrl } from "./domain_utils.ts";

export type ServerConfig = {
  url: string;
  alias: string;
  icon: string;
  zulip_version: string;
  zulip_feature_level: number;
};

export async function getServerSettings(domain: string): Promise<ServerConfig> {
  return invoke("get_server_settings", {
    domain: formatUrl(domain),
  });
}
