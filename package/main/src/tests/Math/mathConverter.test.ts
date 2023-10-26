import { mathConverter } from "@/Math/mathConverter";
test("{mathConverter}", () => {
  expect(mathConverter("1250*1250")).toBe("1500*1000+400*100+200*100+50*50");
  expect(mathConverter("1250^2")).toBe("1500*1000+400*100+200*100+50*50");
  expect(mathConverter("1250")).toBe("1250");
  expect(mathConverter("1250+1250")).toBe("1250+1250");
});
