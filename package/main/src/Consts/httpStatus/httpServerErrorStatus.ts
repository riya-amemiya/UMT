import type { GetEnumValues } from "$/enum/getEnumValues";

/**
 * HTTP 5xx Server Error Status Codes
 * Indicates that the server failed to fulfill a valid request
 */
export const HttpServerErrorStatus = {
  /** Server encountered an unexpected condition that prevented it from fulfilling the request */
  INTERNAL_SERVER_ERROR: 500,
  /** Server does not support the functionality required to fulfill the request */
  NOT_IMPLEMENTED: 501,
  /** Server received an invalid response from the upstream server */
  BAD_GATEWAY: 502,
  /** Server is temporarily unable to handle the request due to being overloaded or down for maintenance */
  SERVICE_UNAVAILABLE: 503,
  /** Server did not receive a timely response from the upstream server */
  GATEWAY_TIMEOUT: 504,
  /** Server does not support the HTTP protocol version used in the request */
  HTTP_VERSION_NOT_SUPPORTED: 505,
} as const;
export type HttpServerErrorStatus = GetEnumValues<typeof HttpServerErrorStatus>;
