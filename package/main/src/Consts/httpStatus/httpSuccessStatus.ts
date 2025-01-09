import type { GetEnumValues } from "$/enum/getEnumValues";

export const HttpSuccessStatus = {
  OK: 200,
  CREATED: 201,
  ACCEPTED: 202,
  NON_AUTHORITATIVE_INFORMATION: 203,
  NO_CONTENT: 204,
  RESET_CONTENT: 205,
  PARTIAL_CONTENT: 206,
} as const;
export type HttpSuccessStatus = GetEnumValues<typeof HttpSuccessStatus>;
