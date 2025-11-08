import * as mcstructs from "./pkg/mcstructs.js";

export class Vector3 {
	#state;

	constructor (x, y, z) {
		this.#state = {e: [x, y, z]}
	}

	_getInternalState() {
		return this.#state;
	}
	get x() { return this.#state.e[0]; }
	get y() { return this.#state.e[1]; }
	get z() { return this.#state.e[2]; }
}

export class BlockType {
	#state;

	constructor (namespace) {
		this.#state = mcstructs.blocktype_new(namespace);
	}

	_getInternalState() {
		return this.#state;
	}

	setState(stateName, state) {
		this.#state = mcstructs.blocktype_set_state(this.#state, stateName, state);
		return this;
	}
}

export class MCStructure {
	#state;

	constructor (size) {
		this.#state = mcstructs.mcstructure_new(size._getInternalState());
	}
	setBlock(loc, block) {
		this.#state = mcstructs.mcstructure_setblock(this.#state, loc._getInternalState(), block._getInternalState());
	}
	asBytes() {
		return mcstructs.mcstructure_as_bytes(this.#state);
	}

}

export class BlockState {
	#state;

	_getInternalState() {return this.#state}
	static String(string) {
		return {
			tag: "String",
			contents: string
		}
	}
	static Int(int) {
		return {
			tag: "Int",
			contents: int
		}
	}
	static Bool(bool) {
		return {
			tag: "Bool",
			contents: typeof bool === "boolean" ? (bool ? 1 : 0) : bool
		}
	}
}