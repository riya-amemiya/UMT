package consts

// HttpInformationalStatus maps HTTP 1xx informational status codes to their names.
// These indicate a provisional response, consisting only of the Status-Line
// and optional headers.
var HttpInformationalStatus = map[int]string{
	100: "CONTINUE",
	101: "SWITCHING_PROTOCOLS",
	102: "PROCESSING",
	103: "EARLYHINTS",
}

// HttpSuccessStatus maps HTTP 2xx success status codes to their names.
// These indicate that the client's request was successfully received,
// understood, and accepted.
var HttpSuccessStatus = map[int]string{
	200: "OK",
	201: "CREATED",
	202: "ACCEPTED",
	203: "NON_AUTHORITATIVE_INFORMATION",
	204: "NO_CONTENT",
	205: "RESET_CONTENT",
	206: "PARTIAL_CONTENT",
}

// HttpRedirectionStatus maps HTTP 3xx redirection status codes to their names.
// These indicate that further action needs to be taken by the user agent
// in order to fulfill the request.
var HttpRedirectionStatus = map[int]string{
	300: "AMBIGUOUS",
	301: "MOVED_PERMANENTLY",
	302: "FOUND",
	303: "SEE_OTHER",
	304: "NOT_MODIFIED",
	307: "TEMPORARY_REDIRECT",
	308: "PERMANENT_REDIRECT",
}

// HttpClientErrorStatus maps HTTP 4xx client error status codes to their names.
// These indicate that the client seems to have made an error in the request.
var HttpClientErrorStatus = map[int]string{
	400: "BAD_REQUEST",
	401: "UNAUTHORIZED",
	402: "PAYMENT_REQUIRED",
	403: "FORBIDDEN",
	404: "NOT_FOUND",
	405: "METHOD_NOT_ALLOWED",
	406: "NOT_ACCEPTABLE",
	407: "PROXY_AUTHENTICATION_REQUIRED",
	408: "REQUEST_TIMEOUT",
	409: "CONFLICT",
	410: "GONE",
	411: "LENGTH_REQUIRED",
	412: "PRECONDITION_FAILED",
	413: "PAYLOAD_TOO_LARGE",
	414: "URI_TOO_LONG",
	415: "UNSUPPORTED_MEDIA_TYPE",
	416: "REQUESTED_RANGE_NOT_SATISFIABLE",
	417: "EXPECTATION_FAILED",
	418: "I_AM_A_TEAPOT",
	421: "MISDIRECTED",
	422: "UNPROCESSABLE_ENTITY",
	424: "FAILED_DEPENDENCY",
	428: "PRECONDITION_REQUIRED",
	429: "TOO_MANY_REQUESTS",
}

// HttpServerErrorStatus maps HTTP 5xx server error status codes to their names.
// These indicate that the server failed to fulfill a valid request.
var HttpServerErrorStatus = map[int]string{
	500: "INTERNAL_SERVER_ERROR",
	501: "NOT_IMPLEMENTED",
	502: "BAD_GATEWAY",
	503: "SERVICE_UNAVAILABLE",
	504: "GATEWAY_TIMEOUT",
	505: "HTTP_VERSION_NOT_SUPPORTED",
}
