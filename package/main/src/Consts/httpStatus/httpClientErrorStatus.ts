import type { GetEnumValues } from "$/enum/getEnumValues";

/**
 * HTTP 4xx Client Error Status Codes
 * Indicates that the client seems to have made an error in the request
 */
export const HttpClientErrorStatus = {
  /** Server cannot or will not process the request due to a client error */
  BAD_REQUEST: 400,
  /** Request requires user authentication */
  UNAUTHORIZED: 401,
  /** Payment is required for access to the resource */
  PAYMENT_REQUIRED: 402,
  /** Server understood the request but refuses to authorize it */
  FORBIDDEN: 403,
  /** Server cannot find the requested resource */
  NOT_FOUND: 404,
  /** Method specified in the request is not allowed for the resource */
  METHOD_NOT_ALLOWED: 405,
  /** Resource cannot generate content according to the Accept headers */
  NOT_ACCEPTABLE: 406,
  /** Client must first authenticate itself with the proxy */
  PROXY_AUTHENTICATION_REQUIRED: 407,
  /** Server timed out waiting for the request */
  REQUEST_TIMEOUT: 408,
  /** Request conflicts with the current state of the server */
  CONFLICT: 409,
  /** Resource requested is no longer available and will not be available again */
  GONE: 410,
  /** Server requires request to have Content-Length header */
  LENGTH_REQUIRED: 411,
  /** Server does not meet one of the preconditions specified in request headers */
  PRECONDITION_FAILED: 412,
  /** Request entity is larger than limits defined by server */
  PAYLOAD_TOO_LARGE: 413,
  /** URI requested by the client is longer than the server is willing to interpret */
  URI_TOO_LONG: 414,
  /** Media format of the requested data is not supported by the server */
  UNSUPPORTED_MEDIA_TYPE: 415,
  /** Range specified by the Range header field cannot be fulfilled */
  REQUESTED_RANGE_NOT_SATISFIABLE: 416,
  /** Expectation indicated by the Expect request header field cannot be met */
  EXPECTATION_FAILED: 417,
  /** Server refuses to brew coffee because it is a teapot (April Fools' joke) */
  I_AM_A_TEAPOT: 418,
  /** Request was directed at a server that is not able to produce a response */
  MISDIRECTED: 421,
  /** Request was well-formed but unable to be followed due to semantic errors */
  UNPROCESSABLE_ENTITY: 422,
  /** Request failed due to failure of a previous request */
  FAILED_DEPENDENCY: 424,
  /** Origin server requires the request to be conditional */
  PRECONDITION_REQUIRED: 428,
  /** User has sent too many requests in a given amount of time */
  TOO_MANY_REQUESTS: 429,
} as const;

export type HttpClientErrorStatus = GetEnumValues<typeof HttpClientErrorStatus>;
