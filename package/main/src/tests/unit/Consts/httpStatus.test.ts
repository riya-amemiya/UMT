import {
  HttpStatus,
  HttpClientErrorStatus,
  HttpInformationalStatus,
  HttpRedirectionStatus,
  HttpServerErrorStatus,
  HttpSuccessStatus,
} from "@/Consts/httpStatus";

describe("HttpStatus", () => {
  describe("Exports", () => {
    it("should export all HTTP status types", () => {
      expect(HttpClientErrorStatus).toBeDefined();
      expect(HttpInformationalStatus).toBeDefined();
      expect(HttpRedirectionStatus).toBeDefined();
      expect(HttpServerErrorStatus).toBeDefined();
      expect(HttpSuccessStatus).toBeDefined();
    });
  });

  describe("1xx Informational", () => {
    it("should have correct informational status codes", () => {
      expect(HttpStatus.CONTINUE).toBe(100);
      expect(HttpStatus.SWITCHING_PROTOCOLS).toBe(101);
      expect(HttpStatus.PROCESSING).toBe(102);
      expect(HttpStatus.EARLYHINTS).toBe(103);
    });
  });

  describe("2xx Success", () => {
    it("should have correct success status codes", () => {
      expect(HttpStatus.OK).toBe(200);
      expect(HttpStatus.CREATED).toBe(201);
      expect(HttpStatus.ACCEPTED).toBe(202);
      expect(HttpStatus.NON_AUTHORITATIVE_INFORMATION).toBe(203);
      expect(HttpStatus.NO_CONTENT).toBe(204);
      expect(HttpStatus.RESET_CONTENT).toBe(205);
      expect(HttpStatus.PARTIAL_CONTENT).toBe(206);
    });
  });

  describe("3xx Redirection", () => {
    it("should have correct redirection status codes", () => {
      expect(HttpStatus.AMBIGUOUS).toBe(300);
      expect(HttpStatus.MOVED_PERMANENTLY).toBe(301);
      expect(HttpStatus.FOUND).toBe(302);
      expect(HttpStatus.SEE_OTHER).toBe(303);
      expect(HttpStatus.NOT_MODIFIED).toBe(304);
      expect(HttpStatus.TEMPORARY_REDIRECT).toBe(307);
      expect(HttpStatus.PERMANENT_REDIRECT).toBe(308);
    });
  });

  describe("4xx Client Error", () => {
    it("should have correct client error status codes", () => {
      expect(HttpStatus.BAD_REQUEST).toBe(400);
      expect(HttpStatus.UNAUTHORIZED).toBe(401);
      expect(HttpStatus.PAYMENT_REQUIRED).toBe(402);
      expect(HttpStatus.FORBIDDEN).toBe(403);
      expect(HttpStatus.NOT_FOUND).toBe(404);
      expect(HttpStatus.METHOD_NOT_ALLOWED).toBe(405);
      expect(HttpStatus.NOT_ACCEPTABLE).toBe(406);
      expect(HttpStatus.PROXY_AUTHENTICATION_REQUIRED).toBe(407);
      expect(HttpStatus.REQUEST_TIMEOUT).toBe(408);
      expect(HttpStatus.CONFLICT).toBe(409);
      expect(HttpStatus.GONE).toBe(410);
      expect(HttpStatus.LENGTH_REQUIRED).toBe(411);
      expect(HttpStatus.PRECONDITION_FAILED).toBe(412);
      expect(HttpStatus.PAYLOAD_TOO_LARGE).toBe(413);
      expect(HttpStatus.URI_TOO_LONG).toBe(414);
      expect(HttpStatus.UNSUPPORTED_MEDIA_TYPE).toBe(415);
      expect(HttpStatus.REQUESTED_RANGE_NOT_SATISFIABLE).toBe(416);
      expect(HttpStatus.EXPECTATION_FAILED).toBe(417);
      expect(HttpStatus.I_AM_A_TEAPOT).toBe(418);
      expect(HttpStatus.MISDIRECTED).toBe(421);
      expect(HttpStatus.UNPROCESSABLE_ENTITY).toBe(422);
      expect(HttpStatus.FAILED_DEPENDENCY).toBe(424);
      expect(HttpStatus.PRECONDITION_REQUIRED).toBe(428);
      expect(HttpStatus.TOO_MANY_REQUESTS).toBe(429);
    });
  });

  describe("5xx Server Error", () => {
    it("should have correct server error status codes", () => {
      expect(HttpStatus.INTERNAL_SERVER_ERROR).toBe(500);
      expect(HttpStatus.NOT_IMPLEMENTED).toBe(501);
      expect(HttpStatus.BAD_GATEWAY).toBe(502);
      expect(HttpStatus.SERVICE_UNAVAILABLE).toBe(503);
      expect(HttpStatus.GATEWAY_TIMEOUT).toBe(504);
      expect(HttpStatus.HTTP_VERSION_NOT_SUPPORTED).toBe(505);
    });
  });
});
