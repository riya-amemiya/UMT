import type { GetEnumValues } from "$/enum/getEnumValues";

export const HttpInformationalStatus = {
  CONTINUE: 100,
  SWITCHING_PROTOCOLS: 101,
  PROCESSING: 102,
  EARLYHINTS: 103,
} as const;
export type HttpInformationalStatus = GetEnumValues<
  typeof HttpInformationalStatus
>;
