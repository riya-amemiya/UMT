import { mathConverter } from "../../../module/Math/mathConverter";
test("{mathConverter}", () => {
  expect(mathConverter("1250*1250")).toBe("1500*1000+400*100+200*100+50*50");
});
