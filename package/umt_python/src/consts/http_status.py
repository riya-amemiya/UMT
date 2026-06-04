"""
HTTP status code constants.
"""


class HttpInformationalStatus:
    """HTTP 1xx Informational Status Codes."""

    CONTINUE = 100
    SWITCHING_PROTOCOLS = 101
    PROCESSING = 102
    EARLYHINTS = 103


class HttpSuccessStatus:
    """HTTP 2xx Success Status Codes."""

    OK = 200
    CREATED = 201
    ACCEPTED = 202
    NON_AUTHORITATIVE_INFORMATION = 203
    NO_CONTENT = 204
    RESET_CONTENT = 205
    PARTIAL_CONTENT = 206


class HttpRedirectionStatus:
    """HTTP 3xx Redirection Status Codes."""

    AMBIGUOUS = 300
    MOVED_PERMANENTLY = 301
    FOUND = 302
    SEE_OTHER = 303
    NOT_MODIFIED = 304
    TEMPORARY_REDIRECT = 307
    PERMANENT_REDIRECT = 308


class HttpClientErrorStatus:
    """HTTP 4xx Client Error Status Codes."""

    BAD_REQUEST = 400
    UNAUTHORIZED = 401
    PAYMENT_REQUIRED = 402
    FORBIDDEN = 403
    NOT_FOUND = 404
    METHOD_NOT_ALLOWED = 405
    NOT_ACCEPTABLE = 406
    PROXY_AUTHENTICATION_REQUIRED = 407
    REQUEST_TIMEOUT = 408
    CONFLICT = 409
    GONE = 410
    LENGTH_REQUIRED = 411
    PRECONDITION_FAILED = 412
    PAYLOAD_TOO_LARGE = 413
    URI_TOO_LONG = 414
    UNSUPPORTED_MEDIA_TYPE = 415
    REQUESTED_RANGE_NOT_SATISFIABLE = 416
    EXPECTATION_FAILED = 417
    I_AM_A_TEAPOT = 418
    MISDIRECTED = 421
    UNPROCESSABLE_ENTITY = 422
    FAILED_DEPENDENCY = 424
    PRECONDITION_REQUIRED = 428
    TOO_MANY_REQUESTS = 429


class HttpServerErrorStatus:
    """HTTP 5xx Server Error Status Codes."""

    INTERNAL_SERVER_ERROR = 500
    NOT_IMPLEMENTED = 501
    BAD_GATEWAY = 502
    SERVICE_UNAVAILABLE = 503
    GATEWAY_TIMEOUT = 504
    HTTP_VERSION_NOT_SUPPORTED = 505


class HttpStatus:
    """All HTTP status codes (1xx through 5xx) merged into a single container.

    Combines :class:`HttpInformationalStatus`, :class:`HttpSuccessStatus`,
    :class:`HttpRedirectionStatus`, :class:`HttpClientErrorStatus`, and
    :class:`HttpServerErrorStatus`, mirroring the TypeScript ``HttpStatus``
    object that spreads every status group together.

    Example:
        >>> HttpStatus.CONTINUE
        100
        >>> HttpStatus.OK
        200
        >>> HttpStatus.MOVED_PERMANENTLY
        301
        >>> HttpStatus.NOT_FOUND
        404
        >>> HttpStatus.INTERNAL_SERVER_ERROR
        500
    """

    # 1xx Informational
    CONTINUE = HttpInformationalStatus.CONTINUE
    SWITCHING_PROTOCOLS = HttpInformationalStatus.SWITCHING_PROTOCOLS
    PROCESSING = HttpInformationalStatus.PROCESSING
    EARLYHINTS = HttpInformationalStatus.EARLYHINTS

    # 2xx Success
    OK = HttpSuccessStatus.OK
    CREATED = HttpSuccessStatus.CREATED
    ACCEPTED = HttpSuccessStatus.ACCEPTED
    NON_AUTHORITATIVE_INFORMATION = HttpSuccessStatus.NON_AUTHORITATIVE_INFORMATION
    NO_CONTENT = HttpSuccessStatus.NO_CONTENT
    RESET_CONTENT = HttpSuccessStatus.RESET_CONTENT
    PARTIAL_CONTENT = HttpSuccessStatus.PARTIAL_CONTENT

    # 3xx Redirection
    AMBIGUOUS = HttpRedirectionStatus.AMBIGUOUS
    MOVED_PERMANENTLY = HttpRedirectionStatus.MOVED_PERMANENTLY
    FOUND = HttpRedirectionStatus.FOUND
    SEE_OTHER = HttpRedirectionStatus.SEE_OTHER
    NOT_MODIFIED = HttpRedirectionStatus.NOT_MODIFIED
    TEMPORARY_REDIRECT = HttpRedirectionStatus.TEMPORARY_REDIRECT
    PERMANENT_REDIRECT = HttpRedirectionStatus.PERMANENT_REDIRECT

    # 4xx Client Error
    BAD_REQUEST = HttpClientErrorStatus.BAD_REQUEST
    UNAUTHORIZED = HttpClientErrorStatus.UNAUTHORIZED
    PAYMENT_REQUIRED = HttpClientErrorStatus.PAYMENT_REQUIRED
    FORBIDDEN = HttpClientErrorStatus.FORBIDDEN
    NOT_FOUND = HttpClientErrorStatus.NOT_FOUND
    METHOD_NOT_ALLOWED = HttpClientErrorStatus.METHOD_NOT_ALLOWED
    NOT_ACCEPTABLE = HttpClientErrorStatus.NOT_ACCEPTABLE
    PROXY_AUTHENTICATION_REQUIRED = HttpClientErrorStatus.PROXY_AUTHENTICATION_REQUIRED
    REQUEST_TIMEOUT = HttpClientErrorStatus.REQUEST_TIMEOUT
    CONFLICT = HttpClientErrorStatus.CONFLICT
    GONE = HttpClientErrorStatus.GONE
    LENGTH_REQUIRED = HttpClientErrorStatus.LENGTH_REQUIRED
    PRECONDITION_FAILED = HttpClientErrorStatus.PRECONDITION_FAILED
    PAYLOAD_TOO_LARGE = HttpClientErrorStatus.PAYLOAD_TOO_LARGE
    URI_TOO_LONG = HttpClientErrorStatus.URI_TOO_LONG
    UNSUPPORTED_MEDIA_TYPE = HttpClientErrorStatus.UNSUPPORTED_MEDIA_TYPE
    REQUESTED_RANGE_NOT_SATISFIABLE = (
        HttpClientErrorStatus.REQUESTED_RANGE_NOT_SATISFIABLE
    )
    EXPECTATION_FAILED = HttpClientErrorStatus.EXPECTATION_FAILED
    I_AM_A_TEAPOT = HttpClientErrorStatus.I_AM_A_TEAPOT
    MISDIRECTED = HttpClientErrorStatus.MISDIRECTED
    UNPROCESSABLE_ENTITY = HttpClientErrorStatus.UNPROCESSABLE_ENTITY
    FAILED_DEPENDENCY = HttpClientErrorStatus.FAILED_DEPENDENCY
    PRECONDITION_REQUIRED = HttpClientErrorStatus.PRECONDITION_REQUIRED
    TOO_MANY_REQUESTS = HttpClientErrorStatus.TOO_MANY_REQUESTS

    # 5xx Server Error
    INTERNAL_SERVER_ERROR = HttpServerErrorStatus.INTERNAL_SERVER_ERROR
    NOT_IMPLEMENTED = HttpServerErrorStatus.NOT_IMPLEMENTED
    BAD_GATEWAY = HttpServerErrorStatus.BAD_GATEWAY
    SERVICE_UNAVAILABLE = HttpServerErrorStatus.SERVICE_UNAVAILABLE
    GATEWAY_TIMEOUT = HttpServerErrorStatus.GATEWAY_TIMEOUT
    HTTP_VERSION_NOT_SUPPORTED = HttpServerErrorStatus.HTTP_VERSION_NOT_SUPPORTED
