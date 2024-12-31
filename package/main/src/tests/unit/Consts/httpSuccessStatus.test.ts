import { HttpSuccessStatus } from "@/Consts/httpStatus";

describe("HttpSuccessStatus", () => {
  describe("2xx Success", () => {
    it("should have correct success status codes", () => {
      expect(HttpSuccessStatus.OK).toBe(200);
      expect(HttpSuccessStatus.CREATED).toBe(201);
      expect(HttpSuccessStatus.ACCEPTED).toBe(202);
      expect(HttpSuccessStatus.NON_AUTHORITATIVE_INFORMATION).toBe(203);
      expect(HttpSuccessStatus.NO_CONTENT).toBe(204);
      expect(HttpSuccessStatus.RESET_CONTENT).toBe(205);
      expect(HttpSuccessStatus.PARTIAL_CONTENT).toBe(206);
    });
  });
});
