import type { GetEnumValues } from "$/enum/getEnumValues";

/**
 * HTTP 2xx Success Status Codes
 * Indicates that the client's request was successfully received, understood, and accepted
 */
export const HttpSuccessStatus = {
  /** Request has succeeded */
  OK: 200,
  /** Request has succeeded and a new resource has been created */
  CREATED: 201,
  /** Request has been accepted for processing, but the processing has not been completed */
  ACCEPTED: 202,
  /** Server returned transformed information from origin server */
  NON_AUTHORITATIVE_INFORMATION: 203,
  /** Server has fulfilled the request but does not need to return any content */
  NO_CONTENT: 204,
  /** Server has fulfilled the request and the client should reset the document view */
  RESET_CONTENT: 205,
  /** Server has fulfilled the partial GET request for the resource */
  PARTIAL_CONTENT: 206,
} as const;
export type HttpSuccessStatus = GetEnumValues<typeof HttpSuccessStatus>;
