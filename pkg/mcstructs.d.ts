/* tslint:disable */
/* eslint-disable */
export class WASM_Block {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  set_item_slot(slot: number, item_type_id: string, count: number): void;
  static new(permutation: WASM_BlockType, index: number, structure: WASM_MCStructure): WASM_Block;
}
export class WASM_BlockType {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static new(namespace: string): WASM_BlockType;
  set_state(state_name: string, state_js: any): void;
}
export class WASM_MCStructure {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static new(size: Int32Array): WASM_MCStructure;
  as_bytes(): Uint8Array;
  setblock(loc: Int32Array, block: WASM_BlockType): WASM_Block;
}
