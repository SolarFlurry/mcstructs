export class Vector3 {
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
	constructor(size: Vector3);
	setBlock(loc: Vector3, block: BlockType);
	asBytes(): number[];
}

export class BlockState {
	static String(string: string);
	static Int(int: number);
	static Bool(bool: boolean | number);
}