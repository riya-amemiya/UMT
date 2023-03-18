import { objectUnion } from "./objectUnion";

export { objectUnion };
export class UMTObjectClass {
	private localObjectUnion: typeof objectUnion;
	constructor() {
		this.localObjectUnion = objectUnion;
	}

	get objectUnion() {
		return this.localObjectUnion;
	}
}

export const UMT_Object = new UMTObjectClass();
