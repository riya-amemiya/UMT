package consts

import "testing"

// ---------------------------------------------------------------------------
// HttpInformationalStatus (1xx)
// ---------------------------------------------------------------------------

func TestHttpInformationalStatusContinue(t *testing.T) {
	if HttpInformationalStatus[100] != "CONTINUE" {
		t.Errorf("expected CONTINUE, got %q", HttpInformationalStatus[100])
	}
}

func TestHttpInformationalStatusSwitchingProtocols(t *testing.T) {
	if HttpInformationalStatus[101] != "SWITCHING_PROTOCOLS" {
		t.Errorf("expected SWITCHING_PROTOCOLS, got %q", HttpInformationalStatus[101])
	}
}

func TestHttpInformationalStatusProcessing(t *testing.T) {
	if HttpInformationalStatus[102] != "PROCESSING" {
		t.Errorf("expected PROCESSING, got %q", HttpInformationalStatus[102])
	}
}

func TestHttpInformationalStatusEarlyHints(t *testing.T) {
	if HttpInformationalStatus[103] != "EARLYHINTS" {
		t.Errorf("expected EARLYHINTS, got %q", HttpInformationalStatus[103])
	}
}

func TestHttpInformationalStatusLength(t *testing.T) {
	if len(HttpInformationalStatus) != 4 {
		t.Errorf("expected 4 entries, got %d", len(HttpInformationalStatus))
	}
}

// ---------------------------------------------------------------------------
// HttpSuccessStatus (2xx)
// ---------------------------------------------------------------------------

func TestHttpSuccessStatusOK(t *testing.T) {
	if HttpSuccessStatus[200] != "OK" {
		t.Errorf("expected OK, got %q", HttpSuccessStatus[200])
	}
}

func TestHttpSuccessStatusCreated(t *testing.T) {
	if HttpSuccessStatus[201] != "CREATED" {
		t.Errorf("expected CREATED, got %q", HttpSuccessStatus[201])
	}
}

func TestHttpSuccessStatusAccepted(t *testing.T) {
	if HttpSuccessStatus[202] != "ACCEPTED" {
		t.Errorf("expected ACCEPTED, got %q", HttpSuccessStatus[202])
	}
}

func TestHttpSuccessStatusNonAuthoritativeInformation(t *testing.T) {
	if HttpSuccessStatus[203] != "NON_AUTHORITATIVE_INFORMATION" {
		t.Errorf("expected NON_AUTHORITATIVE_INFORMATION, got %q", HttpSuccessStatus[203])
	}
}

func TestHttpSuccessStatusNoContent(t *testing.T) {
	if HttpSuccessStatus[204] != "NO_CONTENT" {
		t.Errorf("expected NO_CONTENT, got %q", HttpSuccessStatus[204])
	}
}

func TestHttpSuccessStatusResetContent(t *testing.T) {
	if HttpSuccessStatus[205] != "RESET_CONTENT" {
		t.Errorf("expected RESET_CONTENT, got %q", HttpSuccessStatus[205])
	}
}

func TestHttpSuccessStatusPartialContent(t *testing.T) {
	if HttpSuccessStatus[206] != "PARTIAL_CONTENT" {
		t.Errorf("expected PARTIAL_CONTENT, got %q", HttpSuccessStatus[206])
	}
}

func TestHttpSuccessStatusLength(t *testing.T) {
	if len(HttpSuccessStatus) != 7 {
		t.Errorf("expected 7 entries, got %d", len(HttpSuccessStatus))
	}
}

// ---------------------------------------------------------------------------
// HttpRedirectionStatus (3xx)
// ---------------------------------------------------------------------------

func TestHttpRedirectionStatusAmbiguous(t *testing.T) {
	if HttpRedirectionStatus[300] != "AMBIGUOUS" {
		t.Errorf("expected AMBIGUOUS, got %q", HttpRedirectionStatus[300])
	}
}

func TestHttpRedirectionStatusMovedPermanently(t *testing.T) {
	if HttpRedirectionStatus[301] != "MOVED_PERMANENTLY" {
		t.Errorf("expected MOVED_PERMANENTLY, got %q", HttpRedirectionStatus[301])
	}
}

func TestHttpRedirectionStatusFound(t *testing.T) {
	if HttpRedirectionStatus[302] != "FOUND" {
		t.Errorf("expected FOUND, got %q", HttpRedirectionStatus[302])
	}
}

func TestHttpRedirectionStatusSeeOther(t *testing.T) {
	if HttpRedirectionStatus[303] != "SEE_OTHER" {
		t.Errorf("expected SEE_OTHER, got %q", HttpRedirectionStatus[303])
	}
}

func TestHttpRedirectionStatusNotModified(t *testing.T) {
	if HttpRedirectionStatus[304] != "NOT_MODIFIED" {
		t.Errorf("expected NOT_MODIFIED, got %q", HttpRedirectionStatus[304])
	}
}

func TestHttpRedirectionStatusTemporaryRedirect(t *testing.T) {
	if HttpRedirectionStatus[307] != "TEMPORARY_REDIRECT" {
		t.Errorf("expected TEMPORARY_REDIRECT, got %q", HttpRedirectionStatus[307])
	}
}

func TestHttpRedirectionStatusPermanentRedirect(t *testing.T) {
	if HttpRedirectionStatus[308] != "PERMANENT_REDIRECT" {
		t.Errorf("expected PERMANENT_REDIRECT, got %q", HttpRedirectionStatus[308])
	}
}

func TestHttpRedirectionStatusLength(t *testing.T) {
	if len(HttpRedirectionStatus) != 7 {
		t.Errorf("expected 7 entries, got %d", len(HttpRedirectionStatus))
	}
}

// ---------------------------------------------------------------------------
// HttpClientErrorStatus (4xx)
// ---------------------------------------------------------------------------

func TestHttpClientErrorStatusBadRequest(t *testing.T) {
	if HttpClientErrorStatus[400] != "BAD_REQUEST" {
		t.Errorf("expected BAD_REQUEST, got %q", HttpClientErrorStatus[400])
	}
}

func TestHttpClientErrorStatusUnauthorized(t *testing.T) {
	if HttpClientErrorStatus[401] != "UNAUTHORIZED" {
		t.Errorf("expected UNAUTHORIZED, got %q", HttpClientErrorStatus[401])
	}
}

func TestHttpClientErrorStatusPaymentRequired(t *testing.T) {
	if HttpClientErrorStatus[402] != "PAYMENT_REQUIRED" {
		t.Errorf("expected PAYMENT_REQUIRED, got %q", HttpClientErrorStatus[402])
	}
}

func TestHttpClientErrorStatusForbidden(t *testing.T) {
	if HttpClientErrorStatus[403] != "FORBIDDEN" {
		t.Errorf("expected FORBIDDEN, got %q", HttpClientErrorStatus[403])
	}
}

func TestHttpClientErrorStatusNotFound(t *testing.T) {
	if HttpClientErrorStatus[404] != "NOT_FOUND" {
		t.Errorf("expected NOT_FOUND, got %q", HttpClientErrorStatus[404])
	}
}

func TestHttpClientErrorStatusMethodNotAllowed(t *testing.T) {
	if HttpClientErrorStatus[405] != "METHOD_NOT_ALLOWED" {
		t.Errorf("expected METHOD_NOT_ALLOWED, got %q", HttpClientErrorStatus[405])
	}
}

func TestHttpClientErrorStatusNotAcceptable(t *testing.T) {
	if HttpClientErrorStatus[406] != "NOT_ACCEPTABLE" {
		t.Errorf("expected NOT_ACCEPTABLE, got %q", HttpClientErrorStatus[406])
	}
}

func TestHttpClientErrorStatusProxyAuthenticationRequired(t *testing.T) {
	if HttpClientErrorStatus[407] != "PROXY_AUTHENTICATION_REQUIRED" {
		t.Errorf("expected PROXY_AUTHENTICATION_REQUIRED, got %q", HttpClientErrorStatus[407])
	}
}

func TestHttpClientErrorStatusRequestTimeout(t *testing.T) {
	if HttpClientErrorStatus[408] != "REQUEST_TIMEOUT" {
		t.Errorf("expected REQUEST_TIMEOUT, got %q", HttpClientErrorStatus[408])
	}
}

func TestHttpClientErrorStatusConflict(t *testing.T) {
	if HttpClientErrorStatus[409] != "CONFLICT" {
		t.Errorf("expected CONFLICT, got %q", HttpClientErrorStatus[409])
	}
}

func TestHttpClientErrorStatusGone(t *testing.T) {
	if HttpClientErrorStatus[410] != "GONE" {
		t.Errorf("expected GONE, got %q", HttpClientErrorStatus[410])
	}
}

func TestHttpClientErrorStatusLengthRequired(t *testing.T) {
	if HttpClientErrorStatus[411] != "LENGTH_REQUIRED" {
		t.Errorf("expected LENGTH_REQUIRED, got %q", HttpClientErrorStatus[411])
	}
}

func TestHttpClientErrorStatusPreconditionFailed(t *testing.T) {
	if HttpClientErrorStatus[412] != "PRECONDITION_FAILED" {
		t.Errorf("expected PRECONDITION_FAILED, got %q", HttpClientErrorStatus[412])
	}
}

func TestHttpClientErrorStatusPayloadTooLarge(t *testing.T) {
	if HttpClientErrorStatus[413] != "PAYLOAD_TOO_LARGE" {
		t.Errorf("expected PAYLOAD_TOO_LARGE, got %q", HttpClientErrorStatus[413])
	}
}

func TestHttpClientErrorStatusURITooLong(t *testing.T) {
	if HttpClientErrorStatus[414] != "URI_TOO_LONG" {
		t.Errorf("expected URI_TOO_LONG, got %q", HttpClientErrorStatus[414])
	}
}

func TestHttpClientErrorStatusUnsupportedMediaType(t *testing.T) {
	if HttpClientErrorStatus[415] != "UNSUPPORTED_MEDIA_TYPE" {
		t.Errorf("expected UNSUPPORTED_MEDIA_TYPE, got %q", HttpClientErrorStatus[415])
	}
}

func TestHttpClientErrorStatusRequestedRangeNotSatisfiable(t *testing.T) {
	if HttpClientErrorStatus[416] != "REQUESTED_RANGE_NOT_SATISFIABLE" {
		t.Errorf("expected REQUESTED_RANGE_NOT_SATISFIABLE, got %q", HttpClientErrorStatus[416])
	}
}

func TestHttpClientErrorStatusExpectationFailed(t *testing.T) {
	if HttpClientErrorStatus[417] != "EXPECTATION_FAILED" {
		t.Errorf("expected EXPECTATION_FAILED, got %q", HttpClientErrorStatus[417])
	}
}

func TestHttpClientErrorStatusIAmATeapot(t *testing.T) {
	if HttpClientErrorStatus[418] != "I_AM_A_TEAPOT" {
		t.Errorf("expected I_AM_A_TEAPOT, got %q", HttpClientErrorStatus[418])
	}
}

func TestHttpClientErrorStatusMisdirected(t *testing.T) {
	if HttpClientErrorStatus[421] != "MISDIRECTED" {
		t.Errorf("expected MISDIRECTED, got %q", HttpClientErrorStatus[421])
	}
}

func TestHttpClientErrorStatusUnprocessableEntity(t *testing.T) {
	if HttpClientErrorStatus[422] != "UNPROCESSABLE_ENTITY" {
		t.Errorf("expected UNPROCESSABLE_ENTITY, got %q", HttpClientErrorStatus[422])
	}
}

func TestHttpClientErrorStatusFailedDependency(t *testing.T) {
	if HttpClientErrorStatus[424] != "FAILED_DEPENDENCY" {
		t.Errorf("expected FAILED_DEPENDENCY, got %q", HttpClientErrorStatus[424])
	}
}

func TestHttpClientErrorStatusPreconditionRequired(t *testing.T) {
	if HttpClientErrorStatus[428] != "PRECONDITION_REQUIRED" {
		t.Errorf("expected PRECONDITION_REQUIRED, got %q", HttpClientErrorStatus[428])
	}
}

func TestHttpClientErrorStatusTooManyRequests(t *testing.T) {
	if HttpClientErrorStatus[429] != "TOO_MANY_REQUESTS" {
		t.Errorf("expected TOO_MANY_REQUESTS, got %q", HttpClientErrorStatus[429])
	}
}

func TestHttpClientErrorStatusLength(t *testing.T) {
	if len(HttpClientErrorStatus) != 24 {
		t.Errorf("expected 24 entries, got %d", len(HttpClientErrorStatus))
	}
}

// ---------------------------------------------------------------------------
// HttpServerErrorStatus (5xx)
// ---------------------------------------------------------------------------

func TestHttpServerErrorStatusInternalServerError(t *testing.T) {
	if HttpServerErrorStatus[500] != "INTERNAL_SERVER_ERROR" {
		t.Errorf("expected INTERNAL_SERVER_ERROR, got %q", HttpServerErrorStatus[500])
	}
}

func TestHttpServerErrorStatusNotImplemented(t *testing.T) {
	if HttpServerErrorStatus[501] != "NOT_IMPLEMENTED" {
		t.Errorf("expected NOT_IMPLEMENTED, got %q", HttpServerErrorStatus[501])
	}
}

func TestHttpServerErrorStatusBadGateway(t *testing.T) {
	if HttpServerErrorStatus[502] != "BAD_GATEWAY" {
		t.Errorf("expected BAD_GATEWAY, got %q", HttpServerErrorStatus[502])
	}
}

func TestHttpServerErrorStatusServiceUnavailable(t *testing.T) {
	if HttpServerErrorStatus[503] != "SERVICE_UNAVAILABLE" {
		t.Errorf("expected SERVICE_UNAVAILABLE, got %q", HttpServerErrorStatus[503])
	}
}

func TestHttpServerErrorStatusGatewayTimeout(t *testing.T) {
	if HttpServerErrorStatus[504] != "GATEWAY_TIMEOUT" {
		t.Errorf("expected GATEWAY_TIMEOUT, got %q", HttpServerErrorStatus[504])
	}
}

func TestHttpServerErrorStatusHTTPVersionNotSupported(t *testing.T) {
	if HttpServerErrorStatus[505] != "HTTP_VERSION_NOT_SUPPORTED" {
		t.Errorf("expected HTTP_VERSION_NOT_SUPPORTED, got %q", HttpServerErrorStatus[505])
	}
}

func TestHttpServerErrorStatusLength(t *testing.T) {
	if len(HttpServerErrorStatus) != 6 {
		t.Errorf("expected 6 entries, got %d", len(HttpServerErrorStatus))
	}
}

// ---------------------------------------------------------------------------
// Cross-category: ensure no overlapping codes
// ---------------------------------------------------------------------------

func TestHttpStatusCodesAreUniqueBetweenCategories(t *testing.T) {
	allCodes := make(map[int]string)
	categories := []struct {
		name   string
		status map[int]string
	}{
		{"Informational", HttpInformationalStatus},
		{"Success", HttpSuccessStatus},
		{"Redirection", HttpRedirectionStatus},
		{"ClientError", HttpClientErrorStatus},
		{"ServerError", HttpServerErrorStatus},
	}

	for _, cat := range categories {
		for code, name := range cat.status {
			if existing, found := allCodes[code]; found {
				t.Errorf("duplicate status code %d: %q (already mapped as %q)", code, name, existing)
			}
			allCodes[code] = name
		}
	}
}
