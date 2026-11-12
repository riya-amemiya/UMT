import { inverseLerp } from "./inverseLerp";
import { lerp } from "./lerp";

/**
Maps a value from one range to another without clamping
@param {number} value Value in the input range
@param {number} inMin Start of the input range
@param {number} inMax End of the input range
@param {number} outMin Start of the output range
@param {number} outMax End of the output range
@returns {number} The mapped value, or outMin when inMin equals inMax
@example mapRange(5, 0, 10, 0, 100); // 50
@example mapRange(0.5, 0, 1, 10, 20); // 15
@example mapRange(15, 0, 10, 0, 100); // 150
*/
export const mapRange = (
  value: number,
  inMin: number,
  inMax: number,
  outMin: number,
  outMax: number,
): number => lerp(outMin, outMax, inverseLerp(inMin, inMax, value));
