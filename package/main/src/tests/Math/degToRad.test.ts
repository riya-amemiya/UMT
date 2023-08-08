import { degToRad } from "@/Math/degToRad";
test("{degToRad}", () => {
  expect(degToRad(0)).toBe(0);
  expect(degToRad(90)).toBe(Math.PI / 2);
  expect(degToRad(180)).toBe(Math.PI);
});
