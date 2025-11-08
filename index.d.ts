export class Vec3 {
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