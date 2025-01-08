import type { GetEnumValues } from "$/enum/getEnumValues";

export const HttpServerErrorStatus = {
  INTERNAL_SERVER_ERROR: 500,
  NOT_IMPLEMENTED: 501,
  BAD_GATEWAY: 502,
  SERVICE_UNAVAILABLE: 503,
  GATEWAY_TIMEOUT: 504,
  HTTP_VERSION_NOT_SUPPORTED: 505,
} as const;
export type HttpServerErrorStatus = GetEnumValues<typeof HttpServerErrorStatus>;
