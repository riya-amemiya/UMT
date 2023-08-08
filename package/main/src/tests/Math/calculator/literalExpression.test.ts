import { literalExpression } from "../../../../module/Math/calculator/literalExpression";
test("literalExpression", () => {
  expect(literalExpression("x=1+1")).toBe("2");
  expect(literalExpression("x=1+1+1")).toBe("3");
  expect(literalExpression("x=1+1+1+1")).toBe("4");
  expect(literalExpression("x=1+1+1+1+1")).toBe("5");
});
