import type { GetEnumValues } from "$/enum/getEnumValues";

/**
 * HTTP 1xx Informational Status Codes
 * Indicates a provisional response, consisting only of the Status-Line and optional headers,
 * and is terminated by an empty line
 */
export const HttpInformationalStatus = {
  /** Server has received request headers and client should proceed with request */
  CONTINUE: 100,
  /** Server is switching protocols according to Upgrade header */
  SWITCHING_PROTOCOLS: 101,
  /** Server has received and is processing the request, but no response is available yet */
  PROCESSING: 102,
  /** Server is likely to send a final response with the header fields included in the informational response */
  EARLYHINTS: 103,
} as const;
export type HttpInformationalStatus = GetEnumValues<
  typeof HttpInformationalStatus
>;
