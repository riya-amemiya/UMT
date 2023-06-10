import { factorize } from "../../module/Math/factorize";
test("{factorize}", () => {
  expect(factorize(12)).toEqual(expect.arrayContaining([2, 2, 3]));
});
