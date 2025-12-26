export class Vec3<T> {
	x: number;
	y: number;
	z: number;
	constructor(x: number, y: number, z: number);
}

export class BlockType {
	constructor(namespace: string);
	setState(stateName: string, state: BlockState): BlockType;
}

export class MCStructure {
	constructor(size: Vec3);
	setBlock(loc: Vec3, block: BlockType): void;
	asBytes(): Int8Array;
}

export class BlockState {
	private constructor();
	static String(string: string): BlockState;
	static Int(i32: number): BlockState;
	static Bool(bool: boolean): BlockState;
}