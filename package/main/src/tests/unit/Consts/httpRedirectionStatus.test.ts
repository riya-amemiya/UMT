import { HttpRedirectionStatus } from "@/Consts/httpStatus";

describe("HttpRedirectionStatus", () => {
  describe("3xx Redirection", () => {
    it("should have correct redirection status codes", () => {
      expect(HttpRedirectionStatus.AMBIGUOUS).toBe(300);
      expect(HttpRedirectionStatus.MOVED_PERMANENTLY).toBe(301);
      expect(HttpRedirectionStatus.FOUND).toBe(302);
      expect(HttpRedirectionStatus.SEE_OTHER).toBe(303);
      expect(HttpRedirectionStatus.NOT_MODIFIED).toBe(304);
      expect(HttpRedirectionStatus.TEMPORARY_REDIRECT).toBe(307);
      expect(HttpRedirectionStatus.PERMANENT_REDIRECT).toBe(308);
    });
  });
});
