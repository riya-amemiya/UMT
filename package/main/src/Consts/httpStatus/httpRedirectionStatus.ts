import type { GetEnumValues } from "$/enum/getEnumValues";

export const HttpRedirectionStatus = {
  AMBIGUOUS: 300,
  MOVED_PERMANENTLY: 301,
  FOUND: 302,
  SEE_OTHER: 303,
  NOT_MODIFIED: 304,
  TEMPORARY_REDIRECT: 307,
  PERMANENT_REDIRECT: 308,
} as const;
export type HttpRedirectionStatus = GetEnumValues<typeof HttpRedirectionStatus>;
