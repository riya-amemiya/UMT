import unittest

from src.consts import (
    HttpClientErrorStatus,
    HttpInformationalStatus,
    HttpRedirectionStatus,
    HttpServerErrorStatus,
    HttpStatus,
    HttpSuccessStatus,
)


class TestHttpStatus(unittest.TestCase):
    def test_exports_all_http_status_groups(self):
        self.assertIsNotNone(HttpClientErrorStatus)
        self.assertIsNotNone(HttpInformationalStatus)
        self.assertIsNotNone(HttpRedirectionStatus)
        self.assertIsNotNone(HttpServerErrorStatus)
        self.assertIsNotNone(HttpSuccessStatus)

    def test_informational_status_codes(self):
        self.assertEqual(HttpStatus.CONTINUE, 100)
        self.assertEqual(HttpStatus.SWITCHING_PROTOCOLS, 101)
        self.assertEqual(HttpStatus.PROCESSING, 102)
        self.assertEqual(HttpStatus.EARLYHINTS, 103)

    def test_success_status_codes(self):
        self.assertEqual(HttpStatus.OK, 200)
        self.assertEqual(HttpStatus.CREATED, 201)
        self.assertEqual(HttpStatus.ACCEPTED, 202)
        self.assertEqual(HttpStatus.NON_AUTHORITATIVE_INFORMATION, 203)
        self.assertEqual(HttpStatus.NO_CONTENT, 204)
        self.assertEqual(HttpStatus.RESET_CONTENT, 205)
        self.assertEqual(HttpStatus.PARTIAL_CONTENT, 206)

    def test_redirection_status_codes(self):
        self.assertEqual(HttpStatus.AMBIGUOUS, 300)
        self.assertEqual(HttpStatus.MOVED_PERMANENTLY, 301)
        self.assertEqual(HttpStatus.FOUND, 302)
        self.assertEqual(HttpStatus.SEE_OTHER, 303)
        self.assertEqual(HttpStatus.NOT_MODIFIED, 304)
        self.assertEqual(HttpStatus.TEMPORARY_REDIRECT, 307)
        self.assertEqual(HttpStatus.PERMANENT_REDIRECT, 308)

    def test_client_error_status_codes(self):
        self.assertEqual(HttpStatus.BAD_REQUEST, 400)
        self.assertEqual(HttpStatus.UNAUTHORIZED, 401)
        self.assertEqual(HttpStatus.PAYMENT_REQUIRED, 402)
        self.assertEqual(HttpStatus.FORBIDDEN, 403)
        self.assertEqual(HttpStatus.NOT_FOUND, 404)
        self.assertEqual(HttpStatus.METHOD_NOT_ALLOWED, 405)
        self.assertEqual(HttpStatus.NOT_ACCEPTABLE, 406)
        self.assertEqual(HttpStatus.PROXY_AUTHENTICATION_REQUIRED, 407)
        self.assertEqual(HttpStatus.REQUEST_TIMEOUT, 408)
        self.assertEqual(HttpStatus.CONFLICT, 409)
        self.assertEqual(HttpStatus.GONE, 410)
        self.assertEqual(HttpStatus.LENGTH_REQUIRED, 411)
        self.assertEqual(HttpStatus.PRECONDITION_FAILED, 412)
        self.assertEqual(HttpStatus.PAYLOAD_TOO_LARGE, 413)
        self.assertEqual(HttpStatus.URI_TOO_LONG, 414)
        self.assertEqual(HttpStatus.UNSUPPORTED_MEDIA_TYPE, 415)
        self.assertEqual(HttpStatus.REQUESTED_RANGE_NOT_SATISFIABLE, 416)
        self.assertEqual(HttpStatus.EXPECTATION_FAILED, 417)
        self.assertEqual(HttpStatus.I_AM_A_TEAPOT, 418)
        self.assertEqual(HttpStatus.MISDIRECTED, 421)
        self.assertEqual(HttpStatus.UNPROCESSABLE_ENTITY, 422)
        self.assertEqual(HttpStatus.FAILED_DEPENDENCY, 424)
        self.assertEqual(HttpStatus.PRECONDITION_REQUIRED, 428)
        self.assertEqual(HttpStatus.TOO_MANY_REQUESTS, 429)

    def test_server_error_status_codes(self):
        self.assertEqual(HttpStatus.INTERNAL_SERVER_ERROR, 500)
        self.assertEqual(HttpStatus.NOT_IMPLEMENTED, 501)
        self.assertEqual(HttpStatus.BAD_GATEWAY, 502)
        self.assertEqual(HttpStatus.SERVICE_UNAVAILABLE, 503)
        self.assertEqual(HttpStatus.GATEWAY_TIMEOUT, 504)
        self.assertEqual(HttpStatus.HTTP_VERSION_NOT_SUPPORTED, 505)


if __name__ == "__main__":
    unittest.main()
