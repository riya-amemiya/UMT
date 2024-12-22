import { toBaseN } from "@/Math/toBaseN";
test("{toBaseN}", () => {
  //2進数
  expect(toBaseN(1)).toBe("1");
  expect(toBaseN(2)).toBe("10");
  expect(toBaseN(3)).toBe("11");
  expect(toBaseN(4)).toBe("100");
  expect(toBaseN(5)).toBe("101");
  expect(toBaseN(6)).toBe("110");
  expect(toBaseN(112)).toBe("1110000");
  //4進数
  expect(toBaseN(7, 4)).toBe("13");
  expect(toBaseN(8, 4)).toBe("20");
  expect(toBaseN(9, 4)).toBe("21");
  expect(toBaseN(112, 4)).toBe("1300");
  //8進数
  expect(toBaseN(7, 8)).toBe("7");
  expect(toBaseN(8, 8)).toBe("10");
  expect(toBaseN(9, 8)).toBe("11");
  expect(toBaseN(112, 8)).toBe("160");
});
