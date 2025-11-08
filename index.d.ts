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
	setBlock(loc: Vector3, block: BlockType): void;
	asBytes(): number[];
}

export class BlockState {
	static String(string: string): any;
	static Int(int: number): any;
	static Bool(bool: boolean | number): any;
}