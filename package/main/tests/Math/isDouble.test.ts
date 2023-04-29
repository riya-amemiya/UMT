import { isDouble } from "../../module/Math/isDouble";
test("{isDouble}", () => {
	expect(isDouble(1)).toBeFalsy();
	expect(isDouble(1.1)).toBeTruthy();
	expect(isDouble(1.11)).toBeTruthy();
	expect(isDouble(1.111)).toBeTruthy();
	expect(isDouble("1.1111")).toBeTruthy();
	expect(isDouble("1.11111", false)).toBeFalsy();
});
