import * as mcstructs from "./pkg/mcstructs.js";

export class Vec3 {
	#e
	constructor(x, y, z) {
		this.#e = new Int32Array([x, y, z])
	}
	get x () {return this.#e[0]}
	get y () {return this.#e[1]}
	get z () {return this.#e[2]}
	set x (val) {this.#e[0] = val}
	set y (val) {this.#e[1] = val}
	set z (val) {this.#e[2] = val}
	_int32array() {return this.#e}
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
	#state
	constructor (size) {
		this.#state = mcstructs.WASM_MCStructure.new(size._int32array())
	}
	setBlock(loc, block) {
		this.#state.setblock(loc._int32array(), block._getInternalState());
	}
	asBytes() {
		return this.#state.as_bytes()
	}
}

export const BlockState = {
	String: (string) => {
		return {
			tag: "String",
			contents: string
		}
	},
	Int: (int) => {
		return {
			tag: "Int",
			contents: int
		}
	},
	Bool: (bool) => {
		return {
			tag: "Bool",
			contents: typeof bool === "boolean" ? (bool ? 1 : 0) : bool
		}
	}
}