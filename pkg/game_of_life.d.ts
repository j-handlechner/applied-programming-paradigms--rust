/* tslint:disable */
/* eslint-disable */
export enum CellState {
  ALIVE = 0,
  ZOMBIE = 1,
  DEAD = 2,
}
export class GameOfLife {
  free(): void;
  constructor(width: number, height: number, cell_size: number);
  reset_cells(): void;
  life(iteration: number, canvas: HTMLCanvasElement): void;
  append_life_iteration(canvas: HTMLCanvasElement): void;
}
export class GameOfLifeCell {
  private constructor();
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_gameoflifecell_free: (a: number, b: number) => void;
  readonly __wbg_gameoflife_free: (a: number, b: number) => void;
  readonly gameoflife_new: (a: number, b: number, c: number) => number;
  readonly gameoflife_reset_cells: (a: number) => void;
  readonly gameoflife_life: (a: number, b: number, c: any) => void;
  readonly gameoflife_append_life_iteration: (a: number, b: any) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
