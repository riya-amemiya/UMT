import { HttpServerErrorStatus } from "@/Consts/httpStatus";

describe("HttpServerErrorStatus", () => {
  describe("5xx Server Error", () => {
    it("should have correct server error status codes", () => {
      expect(HttpServerErrorStatus.INTERNAL_SERVER_ERROR).toBe(500);
      expect(HttpServerErrorStatus.NOT_IMPLEMENTED).toBe(501);
      expect(HttpServerErrorStatus.BAD_GATEWAY).toBe(502);
      expect(HttpServerErrorStatus.SERVICE_UNAVAILABLE).toBe(503);
      expect(HttpServerErrorStatus.GATEWAY_TIMEOUT).toBe(504);
      expect(HttpServerErrorStatus.HTTP_VERSION_NOT_SUPPORTED).toBe(505);
    });
  });
});
