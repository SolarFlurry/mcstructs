/* tslint:disable */
/* eslint-disable */
export function main_js(): void;
export function blocktype_set_state(self_js: any, state_name: string, state_js: any): any;
export function blocktype_new(namespace: string): any;
export class WASM_MCStructure {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static new(size: Int32Array): WASM_MCStructure;
  as_bytes(): Uint8Array;
  setblock(loc: Int32Array, block: any): void;
}
