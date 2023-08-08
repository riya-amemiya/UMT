import { toBinary } from "@/Math/toBinary";
test("{toBinary}", () => {
  //2進数
  expect(toBinary(1)).toBe("1");
  expect(toBinary(2)).toBe("10");
  expect(toBinary(3)).toBe("11");
  expect(toBinary(4)).toBe("100");
  expect(toBinary(5)).toBe("101");
  expect(toBinary(6)).toBe("110");
  expect(toBinary(112)).toBe("1110000");
  //4進数
  expect(toBinary(7, 4)).toBe("13");
  expect(toBinary(8, 4)).toBe("20");
  expect(toBinary(9, 4)).toBe("21");
  expect(toBinary(112, 4)).toBe("1300");
  //8進数
  expect(toBinary(7, 8)).toBe("7");
  expect(toBinary(8, 8)).toBe("10");
  expect(toBinary(9, 8)).toBe("11");
  expect(toBinary(112, 8)).toBe("160");
});
