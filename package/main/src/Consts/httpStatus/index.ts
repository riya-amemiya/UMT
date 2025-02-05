import type { GetEnumValues } from "$/enum/getEnumValues";
import { HttpClientErrorStatus } from "@/Consts/httpStatus/httpClientErrorStatus";
import { HttpInformationalStatus } from "@/Consts/httpStatus/httpInformationalStatus";
import { HttpRedirectionStatus } from "@/Consts/httpStatus/httpRedirectionStatus";
import { HttpServerErrorStatus } from "@/Consts/httpStatus/httpServerErrorStatus";
import { HttpSuccessStatus } from "@/Consts/httpStatus/httpSuccessStatus";

export const HttpStatus = {
  ...HttpInformationalStatus,
  ...HttpSuccessStatus,
  ...HttpRedirectionStatus,
  ...HttpClientErrorStatus,
  ...HttpServerErrorStatus,
} as const;

export type HttpStatus = GetEnumValues<typeof HttpStatus>;

export { HttpClientErrorStatus } from "@/Consts/httpStatus/httpClientErrorStatus";
export { HttpInformationalStatus } from "@/Consts/httpStatus/httpInformationalStatus";
export { HttpRedirectionStatus } from "@/Consts/httpStatus/httpRedirectionStatus";
export { HttpServerErrorStatus } from "@/Consts/httpStatus/httpServerErrorStatus";
export { HttpSuccessStatus } from "@/Consts/httpStatus/httpSuccessStatus";
