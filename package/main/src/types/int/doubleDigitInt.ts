import type { UpToEighty } from "./upToEighty";
import type { UpToFifty } from "./upToFifty";
import type { UpToForty } from "./upToForty";
import type { UpToNinety } from "./upToNinety";
import type { UpToNinetyNine } from "./upToNinetyNine";
import type { UpToSeventy } from "./upToSeventy";
import type { UpToSixty } from "./upToSixty";
import type { UpToThirty } from "./upToThirty";
import type { UpToTwenty } from "./upToTwenty";

export type DoubleDigitInt =
  | UpToTwenty
  | UpToThirty
  | UpToForty
  | UpToFifty
  | UpToSixty
  | UpToSeventy
  | UpToEighty
  | UpToNinety
  | UpToNinetyNine;
