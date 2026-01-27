package consts_test

import (
	"testing"

	"github.com/riya-amemiya/umt-go/src/consts"
)

// ---------------------------------------------------------------------------
// Basic time unit values
// ---------------------------------------------------------------------------

func TestOneSecondMs(t *testing.T) {
	if consts.OneSecondMs != 1000 {
		t.Errorf("expected 1000, got %d", consts.OneSecondMs)
	}
}

func TestOneMinuteMs(t *testing.T) {
	if consts.OneMinuteMs != 60_000 {
		t.Errorf("expected 60000, got %d", consts.OneMinuteMs)
	}
}

func TestOneHourMs(t *testing.T) {
	if consts.OneHourMs != 3_600_000 {
		t.Errorf("expected 3600000, got %d", consts.OneHourMs)
	}
}

func TestOneDayMs(t *testing.T) {
	if consts.OneDayMs != 86_400_000 {
		t.Errorf("expected 86400000, got %d", consts.OneDayMs)
	}
}

func TestOneWeekMs(t *testing.T) {
	if consts.OneWeekMs != 604_800_000 {
		t.Errorf("expected 604800000, got %d", consts.OneWeekMs)
	}
}

// ---------------------------------------------------------------------------
// Time unit relationships
// ---------------------------------------------------------------------------

func TestMinuteIsSecondTimes60(t *testing.T) {
	if consts.OneMinuteMs != consts.OneSecondMs*60 {
		t.Errorf("expected OneMinuteMs == OneSecondMs*60, got %d", consts.OneMinuteMs)
	}
}

func TestHourIsMinuteTimes60(t *testing.T) {
	if consts.OneHourMs != consts.OneMinuteMs*60 {
		t.Errorf("expected OneHourMs == OneMinuteMs*60, got %d", consts.OneHourMs)
	}
}

func TestDayIsHourTimes24(t *testing.T) {
	if consts.OneDayMs != consts.OneHourMs*24 {
		t.Errorf("expected OneDayMs == OneHourMs*24, got %d", consts.OneDayMs)
	}
}

func TestWeekIsDayTimes7(t *testing.T) {
	if consts.OneWeekMs != consts.OneDayMs*7 {
		t.Errorf("expected OneWeekMs == OneDayMs*7, got %d", consts.OneWeekMs)
	}
}

// ---------------------------------------------------------------------------
// Month variations
// ---------------------------------------------------------------------------

func TestOneMonthMs28(t *testing.T) {
	if consts.OneMonthMs28 != 2_419_200_000 {
		t.Errorf("expected 2419200000, got %d", consts.OneMonthMs28)
	}
	if consts.OneMonthMs28 != consts.OneDayMs*28 {
		t.Errorf("expected OneMonthMs28 == OneDayMs*28, got %d", consts.OneMonthMs28)
	}
}

func TestOneMonthMs29(t *testing.T) {
	if consts.OneMonthMs29 != 2_505_600_000 {
		t.Errorf("expected 2505600000, got %d", consts.OneMonthMs29)
	}
	if consts.OneMonthMs29 != consts.OneDayMs*29 {
		t.Errorf("expected OneMonthMs29 == OneDayMs*29, got %d", consts.OneMonthMs29)
	}
}

func TestOneMonthMs(t *testing.T) {
	if consts.OneMonthMs != 2_592_000_000 {
		t.Errorf("expected 2592000000, got %d", consts.OneMonthMs)
	}
	if consts.OneMonthMs != consts.OneDayMs*30 {
		t.Errorf("expected OneMonthMs == OneDayMs*30, got %d", consts.OneMonthMs)
	}
}

func TestOneMonthMs31(t *testing.T) {
	if consts.OneMonthMs31 != 2_678_400_000 {
		t.Errorf("expected 2678400000, got %d", consts.OneMonthMs31)
	}
	if consts.OneMonthMs31 != consts.OneDayMs*31 {
		t.Errorf("expected OneMonthMs31 == OneDayMs*31, got %d", consts.OneMonthMs31)
	}
}

func TestMonthLengthOrdering(t *testing.T) {
	if consts.OneMonthMs28 >= consts.OneMonthMs29 {
		t.Errorf("expected OneMonthMs28 < OneMonthMs29")
	}
	if consts.OneMonthMs29 >= consts.OneMonthMs {
		t.Errorf("expected OneMonthMs29 < OneMonthMs")
	}
	if consts.OneMonthMs >= consts.OneMonthMs31 {
		t.Errorf("expected OneMonthMs < OneMonthMs31")
	}
}

// ---------------------------------------------------------------------------
// Year variations
// ---------------------------------------------------------------------------

func TestOneYearMs(t *testing.T) {
	if consts.OneYearMs != 31_536_000_000 {
		t.Errorf("expected 31536000000, got %d", consts.OneYearMs)
	}
	if consts.OneYearMs != consts.OneDayMs*365 {
		t.Errorf("expected OneYearMs == OneDayMs*365, got %d", consts.OneYearMs)
	}
}

func TestOneYearMs366(t *testing.T) {
	if consts.OneYearMs366 != 31_622_400_000 {
		t.Errorf("expected 31622400000, got %d", consts.OneYearMs366)
	}
	if consts.OneYearMs366 != consts.OneDayMs*366 {
		t.Errorf("expected OneYearMs366 == OneDayMs*366, got %d", consts.OneYearMs366)
	}
}

func TestLeapYearIsOneDayLonger(t *testing.T) {
	if consts.OneYearMs >= consts.OneYearMs366 {
		t.Errorf("expected OneYearMs < OneYearMs366")
	}
	if consts.OneYearMs366-consts.OneYearMs != consts.OneDayMs {
		t.Errorf("expected leap year difference to be one day (%d), got %d", consts.OneDayMs, consts.OneYearMs366-consts.OneYearMs)
	}
}

// ---------------------------------------------------------------------------
// HttpInformationalStatus (1xx)
// ---------------------------------------------------------------------------

func TestHttpInformationalStatusContinue(t *testing.T) {
	if consts.HttpInformationalStatus[100] != "CONTINUE" {
		t.Errorf("expected CONTINUE, got %q", consts.HttpInformationalStatus[100])
	}
}

func TestHttpInformationalStatusSwitchingProtocols(t *testing.T) {
	if consts.HttpInformationalStatus[101] != "SWITCHING_PROTOCOLS" {
		t.Errorf("expected SWITCHING_PROTOCOLS, got %q", consts.HttpInformationalStatus[101])
	}
}

func TestHttpInformationalStatusProcessing(t *testing.T) {
	if consts.HttpInformationalStatus[102] != "PROCESSING" {
		t.Errorf("expected PROCESSING, got %q", consts.HttpInformationalStatus[102])
	}
}

func TestHttpInformationalStatusEarlyHints(t *testing.T) {
	if consts.HttpInformationalStatus[103] != "EARLYHINTS" {
		t.Errorf("expected EARLYHINTS, got %q", consts.HttpInformationalStatus[103])
	}
}

func TestHttpInformationalStatusLength(t *testing.T) {
	if len(consts.HttpInformationalStatus) != 4 {
		t.Errorf("expected 4 entries, got %d", len(consts.HttpInformationalStatus))
	}
}

// ---------------------------------------------------------------------------
// HttpSuccessStatus (2xx)
// ---------------------------------------------------------------------------

func TestHttpSuccessStatusOK(t *testing.T) {
	if consts.HttpSuccessStatus[200] != "OK" {
		t.Errorf("expected OK, got %q", consts.HttpSuccessStatus[200])
	}
}

func TestHttpSuccessStatusCreated(t *testing.T) {
	if consts.HttpSuccessStatus[201] != "CREATED" {
		t.Errorf("expected CREATED, got %q", consts.HttpSuccessStatus[201])
	}
}

func TestHttpSuccessStatusAccepted(t *testing.T) {
	if consts.HttpSuccessStatus[202] != "ACCEPTED" {
		t.Errorf("expected ACCEPTED, got %q", consts.HttpSuccessStatus[202])
	}
}

func TestHttpSuccessStatusNonAuthoritativeInformation(t *testing.T) {
	if consts.HttpSuccessStatus[203] != "NON_AUTHORITATIVE_INFORMATION" {
		t.Errorf("expected NON_AUTHORITATIVE_INFORMATION, got %q", consts.HttpSuccessStatus[203])
	}
}

func TestHttpSuccessStatusNoContent(t *testing.T) {
	if consts.HttpSuccessStatus[204] != "NO_CONTENT" {
		t.Errorf("expected NO_CONTENT, got %q", consts.HttpSuccessStatus[204])
	}
}

func TestHttpSuccessStatusResetContent(t *testing.T) {
	if consts.HttpSuccessStatus[205] != "RESET_CONTENT" {
		t.Errorf("expected RESET_CONTENT, got %q", consts.HttpSuccessStatus[205])
	}
}

func TestHttpSuccessStatusPartialContent(t *testing.T) {
	if consts.HttpSuccessStatus[206] != "PARTIAL_CONTENT" {
		t.Errorf("expected PARTIAL_CONTENT, got %q", consts.HttpSuccessStatus[206])
	}
}

func TestHttpSuccessStatusLength(t *testing.T) {
	if len(consts.HttpSuccessStatus) != 7 {
		t.Errorf("expected 7 entries, got %d", len(consts.HttpSuccessStatus))
	}
}

// ---------------------------------------------------------------------------
// HttpRedirectionStatus (3xx)
// ---------------------------------------------------------------------------

func TestHttpRedirectionStatusAmbiguous(t *testing.T) {
	if consts.HttpRedirectionStatus[300] != "AMBIGUOUS" {
		t.Errorf("expected AMBIGUOUS, got %q", consts.HttpRedirectionStatus[300])
	}
}

func TestHttpRedirectionStatusMovedPermanently(t *testing.T) {
	if consts.HttpRedirectionStatus[301] != "MOVED_PERMANENTLY" {
		t.Errorf("expected MOVED_PERMANENTLY, got %q", consts.HttpRedirectionStatus[301])
	}
}

func TestHttpRedirectionStatusFound(t *testing.T) {
	if consts.HttpRedirectionStatus[302] != "FOUND" {
		t.Errorf("expected FOUND, got %q", consts.HttpRedirectionStatus[302])
	}
}

func TestHttpRedirectionStatusSeeOther(t *testing.T) {
	if consts.HttpRedirectionStatus[303] != "SEE_OTHER" {
		t.Errorf("expected SEE_OTHER, got %q", consts.HttpRedirectionStatus[303])
	}
}

func TestHttpRedirectionStatusNotModified(t *testing.T) {
	if consts.HttpRedirectionStatus[304] != "NOT_MODIFIED" {
		t.Errorf("expected NOT_MODIFIED, got %q", consts.HttpRedirectionStatus[304])
	}
}

func TestHttpRedirectionStatusTemporaryRedirect(t *testing.T) {
	if consts.HttpRedirectionStatus[307] != "TEMPORARY_REDIRECT" {
		t.Errorf("expected TEMPORARY_REDIRECT, got %q", consts.HttpRedirectionStatus[307])
	}
}

func TestHttpRedirectionStatusPermanentRedirect(t *testing.T) {
	if consts.HttpRedirectionStatus[308] != "PERMANENT_REDIRECT" {
		t.Errorf("expected PERMANENT_REDIRECT, got %q", consts.HttpRedirectionStatus[308])
	}
}

func TestHttpRedirectionStatusLength(t *testing.T) {
	if len(consts.HttpRedirectionStatus) != 7 {
		t.Errorf("expected 7 entries, got %d", len(consts.HttpRedirectionStatus))
	}
}

// ---------------------------------------------------------------------------
// HttpClientErrorStatus (4xx)
// ---------------------------------------------------------------------------

func TestHttpClientErrorStatusBadRequest(t *testing.T) {
	if consts.HttpClientErrorStatus[400] != "BAD_REQUEST" {
		t.Errorf("expected BAD_REQUEST, got %q", consts.HttpClientErrorStatus[400])
	}
}

func TestHttpClientErrorStatusUnauthorized(t *testing.T) {
	if consts.HttpClientErrorStatus[401] != "UNAUTHORIZED" {
		t.Errorf("expected UNAUTHORIZED, got %q", consts.HttpClientErrorStatus[401])
	}
}

func TestHttpClientErrorStatusPaymentRequired(t *testing.T) {
	if consts.HttpClientErrorStatus[402] != "PAYMENT_REQUIRED" {
		t.Errorf("expected PAYMENT_REQUIRED, got %q", consts.HttpClientErrorStatus[402])
	}
}

func TestHttpClientErrorStatusForbidden(t *testing.T) {
	if consts.HttpClientErrorStatus[403] != "FORBIDDEN" {
		t.Errorf("expected FORBIDDEN, got %q", consts.HttpClientErrorStatus[403])
	}
}

func TestHttpClientErrorStatusNotFound(t *testing.T) {
	if consts.HttpClientErrorStatus[404] != "NOT_FOUND" {
		t.Errorf("expected NOT_FOUND, got %q", consts.HttpClientErrorStatus[404])
	}
}

func TestHttpClientErrorStatusMethodNotAllowed(t *testing.T) {
	if consts.HttpClientErrorStatus[405] != "METHOD_NOT_ALLOWED" {
		t.Errorf("expected METHOD_NOT_ALLOWED, got %q", consts.HttpClientErrorStatus[405])
	}
}

func TestHttpClientErrorStatusNotAcceptable(t *testing.T) {
	if consts.HttpClientErrorStatus[406] != "NOT_ACCEPTABLE" {
		t.Errorf("expected NOT_ACCEPTABLE, got %q", consts.HttpClientErrorStatus[406])
	}
}

func TestHttpClientErrorStatusProxyAuthenticationRequired(t *testing.T) {
	if consts.HttpClientErrorStatus[407] != "PROXY_AUTHENTICATION_REQUIRED" {
		t.Errorf("expected PROXY_AUTHENTICATION_REQUIRED, got %q", consts.HttpClientErrorStatus[407])
	}
}

func TestHttpClientErrorStatusRequestTimeout(t *testing.T) {
	if consts.HttpClientErrorStatus[408] != "REQUEST_TIMEOUT" {
		t.Errorf("expected REQUEST_TIMEOUT, got %q", consts.HttpClientErrorStatus[408])
	}
}

func TestHttpClientErrorStatusConflict(t *testing.T) {
	if consts.HttpClientErrorStatus[409] != "CONFLICT" {
		t.Errorf("expected CONFLICT, got %q", consts.HttpClientErrorStatus[409])
	}
}

func TestHttpClientErrorStatusGone(t *testing.T) {
	if consts.HttpClientErrorStatus[410] != "GONE" {
		t.Errorf("expected GONE, got %q", consts.HttpClientErrorStatus[410])
	}
}

func TestHttpClientErrorStatusLengthRequired(t *testing.T) {
	if consts.HttpClientErrorStatus[411] != "LENGTH_REQUIRED" {
		t.Errorf("expected LENGTH_REQUIRED, got %q", consts.HttpClientErrorStatus[411])
	}
}

func TestHttpClientErrorStatusPreconditionFailed(t *testing.T) {
	if consts.HttpClientErrorStatus[412] != "PRECONDITION_FAILED" {
		t.Errorf("expected PRECONDITION_FAILED, got %q", consts.HttpClientErrorStatus[412])
	}
}

func TestHttpClientErrorStatusPayloadTooLarge(t *testing.T) {
	if consts.HttpClientErrorStatus[413] != "PAYLOAD_TOO_LARGE" {
		t.Errorf("expected PAYLOAD_TOO_LARGE, got %q", consts.HttpClientErrorStatus[413])
	}
}

func TestHttpClientErrorStatusURITooLong(t *testing.T) {
	if consts.HttpClientErrorStatus[414] != "URI_TOO_LONG" {
		t.Errorf("expected URI_TOO_LONG, got %q", consts.HttpClientErrorStatus[414])
	}
}

func TestHttpClientErrorStatusUnsupportedMediaType(t *testing.T) {
	if consts.HttpClientErrorStatus[415] != "UNSUPPORTED_MEDIA_TYPE" {
		t.Errorf("expected UNSUPPORTED_MEDIA_TYPE, got %q", consts.HttpClientErrorStatus[415])
	}
}

func TestHttpClientErrorStatusRequestedRangeNotSatisfiable(t *testing.T) {
	if consts.HttpClientErrorStatus[416] != "REQUESTED_RANGE_NOT_SATISFIABLE" {
		t.Errorf("expected REQUESTED_RANGE_NOT_SATISFIABLE, got %q", consts.HttpClientErrorStatus[416])
	}
}

func TestHttpClientErrorStatusExpectationFailed(t *testing.T) {
	if consts.HttpClientErrorStatus[417] != "EXPECTATION_FAILED" {
		t.Errorf("expected EXPECTATION_FAILED, got %q", consts.HttpClientErrorStatus[417])
	}
}

func TestHttpClientErrorStatusIAmATeapot(t *testing.T) {
	if consts.HttpClientErrorStatus[418] != "I_AM_A_TEAPOT" {
		t.Errorf("expected I_AM_A_TEAPOT, got %q", consts.HttpClientErrorStatus[418])
	}
}

func TestHttpClientErrorStatusMisdirected(t *testing.T) {
	if consts.HttpClientErrorStatus[421] != "MISDIRECTED" {
		t.Errorf("expected MISDIRECTED, got %q", consts.HttpClientErrorStatus[421])
	}
}

func TestHttpClientErrorStatusUnprocessableEntity(t *testing.T) {
	if consts.HttpClientErrorStatus[422] != "UNPROCESSABLE_ENTITY" {
		t.Errorf("expected UNPROCESSABLE_ENTITY, got %q", consts.HttpClientErrorStatus[422])
	}
}

func TestHttpClientErrorStatusFailedDependency(t *testing.T) {
	if consts.HttpClientErrorStatus[424] != "FAILED_DEPENDENCY" {
		t.Errorf("expected FAILED_DEPENDENCY, got %q", consts.HttpClientErrorStatus[424])
	}
}

func TestHttpClientErrorStatusPreconditionRequired(t *testing.T) {
	if consts.HttpClientErrorStatus[428] != "PRECONDITION_REQUIRED" {
		t.Errorf("expected PRECONDITION_REQUIRED, got %q", consts.HttpClientErrorStatus[428])
	}
}

func TestHttpClientErrorStatusTooManyRequests(t *testing.T) {
	if consts.HttpClientErrorStatus[429] != "TOO_MANY_REQUESTS" {
		t.Errorf("expected TOO_MANY_REQUESTS, got %q", consts.HttpClientErrorStatus[429])
	}
}

func TestHttpClientErrorStatusLength(t *testing.T) {
	if len(consts.HttpClientErrorStatus) != 24 {
		t.Errorf("expected 24 entries, got %d", len(consts.HttpClientErrorStatus))
	}
}

// ---------------------------------------------------------------------------
// HttpServerErrorStatus (5xx)
// ---------------------------------------------------------------------------

func TestHttpServerErrorStatusInternalServerError(t *testing.T) {
	if consts.HttpServerErrorStatus[500] != "INTERNAL_SERVER_ERROR" {
		t.Errorf("expected INTERNAL_SERVER_ERROR, got %q", consts.HttpServerErrorStatus[500])
	}
}

func TestHttpServerErrorStatusNotImplemented(t *testing.T) {
	if consts.HttpServerErrorStatus[501] != "NOT_IMPLEMENTED" {
		t.Errorf("expected NOT_IMPLEMENTED, got %q", consts.HttpServerErrorStatus[501])
	}
}

func TestHttpServerErrorStatusBadGateway(t *testing.T) {
	if consts.HttpServerErrorStatus[502] != "BAD_GATEWAY" {
		t.Errorf("expected BAD_GATEWAY, got %q", consts.HttpServerErrorStatus[502])
	}
}

func TestHttpServerErrorStatusServiceUnavailable(t *testing.T) {
	if consts.HttpServerErrorStatus[503] != "SERVICE_UNAVAILABLE" {
		t.Errorf("expected SERVICE_UNAVAILABLE, got %q", consts.HttpServerErrorStatus[503])
	}
}

func TestHttpServerErrorStatusGatewayTimeout(t *testing.T) {
	if consts.HttpServerErrorStatus[504] != "GATEWAY_TIMEOUT" {
		t.Errorf("expected GATEWAY_TIMEOUT, got %q", consts.HttpServerErrorStatus[504])
	}
}

func TestHttpServerErrorStatusHTTPVersionNotSupported(t *testing.T) {
	if consts.HttpServerErrorStatus[505] != "HTTP_VERSION_NOT_SUPPORTED" {
		t.Errorf("expected HTTP_VERSION_NOT_SUPPORTED, got %q", consts.HttpServerErrorStatus[505])
	}
}

func TestHttpServerErrorStatusLength(t *testing.T) {
	if len(consts.HttpServerErrorStatus) != 6 {
		t.Errorf("expected 6 entries, got %d", len(consts.HttpServerErrorStatus))
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
		{"Informational", consts.HttpInformationalStatus},
		{"Success", consts.HttpSuccessStatus},
		{"Redirection", consts.HttpRedirectionStatus},
		{"ClientError", consts.HttpClientErrorStatus},
		{"ServerError", consts.HttpServerErrorStatus},
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
