import type { GetEnumValues } from "$/enum/getEnumValues";

/**
 * HTTP 3xx Redirection Status Codes
 * Indicates that further action needs to be taken by the user agent in order to fulfill the request
 */
export const HttpRedirectionStatus = {
  /** Multiple options for the resource from which the client may choose */
  AMBIGUOUS: 300,
  /** Resource has been permanently moved to another URI */
  MOVED_PERMANENTLY: 301,
  /** Resource temporarily resides under a different URI */
  FOUND: 302,
  /** Response to the request can be found under another URI using GET */
  SEE_OTHER: 303,
  /** Resource has not been modified since last requested */
  NOT_MODIFIED: 304,
  /** Resource temporarily moved to another URI, using same HTTP method */
  TEMPORARY_REDIRECT: 307,
  /** Resource has been permanently moved to another URI, using same HTTP method */
  PERMANENT_REDIRECT: 308,
} as const;
export type HttpRedirectionStatus = GetEnumValues<typeof HttpRedirectionStatus>;
