import type { Int } from "$/int/int";
import type { UpToFifty } from "$/int/upToFifty";
import type { UpToForty } from "$/int/upToForty";
import type { UpToSixty } from "$/int/upToSixty";
import type { UpToThirty } from "$/int/upToThirty";
import type { UpToTwenty } from "$/int/upToTwenty";

export type MinutesTypeInt =
  | Int
  | UpToTwenty
  | UpToThirty
  | UpToForty
  | UpToFifty
  | UpToSixty;
