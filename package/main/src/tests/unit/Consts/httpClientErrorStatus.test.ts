import { HttpClientErrorStatus } from "@/Consts/httpStatus";

describe("HttpClientErrorStatus", () => {
  describe("4xx Client Error", () => {
    it("should have correct client error status codes", () => {
      expect(HttpClientErrorStatus.BAD_REQUEST).toBe(400);
      expect(HttpClientErrorStatus.UNAUTHORIZED).toBe(401);
      expect(HttpClientErrorStatus.PAYMENT_REQUIRED).toBe(402);
      expect(HttpClientErrorStatus.FORBIDDEN).toBe(403);
      expect(HttpClientErrorStatus.NOT_FOUND).toBe(404);
      expect(HttpClientErrorStatus.METHOD_NOT_ALLOWED).toBe(405);
      expect(HttpClientErrorStatus.NOT_ACCEPTABLE).toBe(406);
      expect(HttpClientErrorStatus.PROXY_AUTHENTICATION_REQUIRED).toBe(407);
      expect(HttpClientErrorStatus.REQUEST_TIMEOUT).toBe(408);
      expect(HttpClientErrorStatus.CONFLICT).toBe(409);
      expect(HttpClientErrorStatus.GONE).toBe(410);
      expect(HttpClientErrorStatus.LENGTH_REQUIRED).toBe(411);
      expect(HttpClientErrorStatus.PRECONDITION_FAILED).toBe(412);
      expect(HttpClientErrorStatus.PAYLOAD_TOO_LARGE).toBe(413);
      expect(HttpClientErrorStatus.URI_TOO_LONG).toBe(414);
      expect(HttpClientErrorStatus.UNSUPPORTED_MEDIA_TYPE).toBe(415);
      expect(HttpClientErrorStatus.REQUESTED_RANGE_NOT_SATISFIABLE).toBe(416);
      expect(HttpClientErrorStatus.EXPECTATION_FAILED).toBe(417);
      expect(HttpClientErrorStatus.I_AM_A_TEAPOT).toBe(418);
      expect(HttpClientErrorStatus.MISDIRECTED).toBe(421);
      expect(HttpClientErrorStatus.UNPROCESSABLE_ENTITY).toBe(422);
      expect(HttpClientErrorStatus.FAILED_DEPENDENCY).toBe(424);
      expect(HttpClientErrorStatus.PRECONDITION_REQUIRED).toBe(428);
      expect(HttpClientErrorStatus.TOO_MANY_REQUESTS).toBe(429);
    });
  });
});
