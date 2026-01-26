//! HTTP 4xx Client Error Status Codes.
//!
//! Indicates that the client seems to have made an error in the request.

/// Server cannot or will not process the request due to a client error.
pub const BAD_REQUEST: u16 = 400;

/// Request requires user authentication.
pub const UNAUTHORIZED: u16 = 401;

/// Payment is required for access to the resource.
pub const PAYMENT_REQUIRED: u16 = 402;

/// Server understood the request but refuses to authorize it.
pub const FORBIDDEN: u16 = 403;

/// Server cannot find the requested resource.
pub const NOT_FOUND: u16 = 404;

/// Method specified in the request is not allowed for the resource.
pub const METHOD_NOT_ALLOWED: u16 = 405;

/// Resource cannot generate content according to the Accept headers.
pub const NOT_ACCEPTABLE: u16 = 406;

/// Client must first authenticate itself with the proxy.
pub const PROXY_AUTHENTICATION_REQUIRED: u16 = 407;

/// Server timed out waiting for the request.
pub const REQUEST_TIMEOUT: u16 = 408;

/// Request conflicts with the current state of the server.
pub const CONFLICT: u16 = 409;

/// Resource requested is no longer available and will not be available again.
pub const GONE: u16 = 410;

/// Server requires request to have Content-Length header.
pub const LENGTH_REQUIRED: u16 = 411;

/// Server does not meet one of the preconditions specified in request headers.
pub const PRECONDITION_FAILED: u16 = 412;

/// Request entity is larger than limits defined by server.
pub const PAYLOAD_TOO_LARGE: u16 = 413;

/// URI requested by the client is longer than the server is willing to interpret.
pub const URI_TOO_LONG: u16 = 414;

/// Media format of the requested data is not supported by the server.
pub const UNSUPPORTED_MEDIA_TYPE: u16 = 415;

/// Range specified by the Range header field cannot be fulfilled.
pub const REQUESTED_RANGE_NOT_SATISFIABLE: u16 = 416;

/// Expectation indicated by the Expect request header field cannot be met.
pub const EXPECTATION_FAILED: u16 = 417;

/// Server refuses to brew coffee because it is a teapot (April Fools' joke).
pub const I_AM_A_TEAPOT: u16 = 418;

/// Request was directed at a server that is not able to produce a response.
pub const MISDIRECTED: u16 = 421;

/// Request was well-formed but unable to be followed due to semantic errors.
pub const UNPROCESSABLE_ENTITY: u16 = 422;

/// Request failed due to failure of a previous request.
pub const FAILED_DEPENDENCY: u16 = 424;

/// Origin server requires the request to be conditional.
pub const PRECONDITION_REQUIRED: u16 = 428;

/// User has sent too many requests in a given amount of time.
pub const TOO_MANY_REQUESTS: u16 = 429;

/// HTTP 4xx Client Error status codes as a module-like struct.
pub struct HttpClientErrorStatus;

impl HttpClientErrorStatus {
    /// Server cannot or will not process the request due to a client error.
    pub const BAD_REQUEST: u16 = BAD_REQUEST;

    /// Request requires user authentication.
    pub const UNAUTHORIZED: u16 = UNAUTHORIZED;

    /// Payment is required for access to the resource.
    pub const PAYMENT_REQUIRED: u16 = PAYMENT_REQUIRED;

    /// Server understood the request but refuses to authorize it.
    pub const FORBIDDEN: u16 = FORBIDDEN;

    /// Server cannot find the requested resource.
    pub const NOT_FOUND: u16 = NOT_FOUND;

    /// Method specified in the request is not allowed for the resource.
    pub const METHOD_NOT_ALLOWED: u16 = METHOD_NOT_ALLOWED;

    /// Resource cannot generate content according to the Accept headers.
    pub const NOT_ACCEPTABLE: u16 = NOT_ACCEPTABLE;

    /// Client must first authenticate itself with the proxy.
    pub const PROXY_AUTHENTICATION_REQUIRED: u16 = PROXY_AUTHENTICATION_REQUIRED;

    /// Server timed out waiting for the request.
    pub const REQUEST_TIMEOUT: u16 = REQUEST_TIMEOUT;

    /// Request conflicts with the current state of the server.
    pub const CONFLICT: u16 = CONFLICT;

    /// Resource requested is no longer available and will not be available again.
    pub const GONE: u16 = GONE;

    /// Server requires request to have Content-Length header.
    pub const LENGTH_REQUIRED: u16 = LENGTH_REQUIRED;

    /// Server does not meet one of the preconditions specified in request headers.
    pub const PRECONDITION_FAILED: u16 = PRECONDITION_FAILED;

    /// Request entity is larger than limits defined by server.
    pub const PAYLOAD_TOO_LARGE: u16 = PAYLOAD_TOO_LARGE;

    /// URI requested by the client is longer than the server is willing to interpret.
    pub const URI_TOO_LONG: u16 = URI_TOO_LONG;

    /// Media format of the requested data is not supported by the server.
    pub const UNSUPPORTED_MEDIA_TYPE: u16 = UNSUPPORTED_MEDIA_TYPE;

    /// Range specified by the Range header field cannot be fulfilled.
    pub const REQUESTED_RANGE_NOT_SATISFIABLE: u16 = REQUESTED_RANGE_NOT_SATISFIABLE;

    /// Expectation indicated by the Expect request header field cannot be met.
    pub const EXPECTATION_FAILED: u16 = EXPECTATION_FAILED;

    /// Server refuses to brew coffee because it is a teapot (April Fools' joke).
    pub const I_AM_A_TEAPOT: u16 = I_AM_A_TEAPOT;

    /// Request was directed at a server that is not able to produce a response.
    pub const MISDIRECTED: u16 = MISDIRECTED;

    /// Request was well-formed but unable to be followed due to semantic errors.
    pub const UNPROCESSABLE_ENTITY: u16 = UNPROCESSABLE_ENTITY;

    /// Request failed due to failure of a previous request.
    pub const FAILED_DEPENDENCY: u16 = FAILED_DEPENDENCY;

    /// Origin server requires the request to be conditional.
    pub const PRECONDITION_REQUIRED: u16 = PRECONDITION_REQUIRED;

    /// User has sent too many requests in a given amount of time.
    pub const TOO_MANY_REQUESTS: u16 = TOO_MANY_REQUESTS;
}
