import type { SimplifiedUserAgentInfoBrowser } from "./simplifiedUserAgentInfoBrowser";
import type { SimplifiedUserAgentInfoDevice } from "./simplifiedUserAgentInfoDevice";
import type { SimplifiedUserAgentInfoOs } from "./simplifiedUserAgentInfoOs";

export interface SimplifiedUserAgentInfo {
  os: SimplifiedUserAgentInfoOs;
  browser: SimplifiedUserAgentInfoBrowser;
  device: SimplifiedUserAgentInfoDevice;
}
