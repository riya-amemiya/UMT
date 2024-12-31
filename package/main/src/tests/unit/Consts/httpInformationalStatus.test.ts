import { HttpInformationalStatus } from "@/Consts/httpStatus";

describe("HttpInformationalStatus", () => {
  describe("1xx Informational", () => {
    it("should have correct informational status codes", () => {
      expect(HttpInformationalStatus.CONTINUE).toBe(100);
      expect(HttpInformationalStatus.SWITCHING_PROTOCOLS).toBe(101);
      expect(HttpInformationalStatus.PROCESSING).toBe(102);
      expect(HttpInformationalStatus.EARLYHINTS).toBe(103);
    });
  });
});
