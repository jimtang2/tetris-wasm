/* tslint:disable */
/* eslint-disable */
export class Tetris {
  free(): void;
  constructor(canvas_id: string);
  start(): void;
  pause(): void;
  unpause(): void;
  is_paused(): boolean;
  move_left(): void;
  move_right(): void;
  move_down(): boolean;
  drop(): void;
  rotate_left(): void;
  rotate_right(): void;
  update_clearing_animation(delta_time: number): void;
  draw(): void;
  draw_next(canvas_id: string): void;
  get_score(): number;
  is_game_over(): boolean;
  get_cleared_lanes(): number;
  get_tetris_count(): number;
  get_triple_count(): number;
  get_double_count(): number;
  get_single_count(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_tetris_free: (a: number, b: number) => void;
  readonly tetris_new: (a: number, b: number) => number;
  readonly tetris_start: (a: number) => void;
  readonly tetris_pause: (a: number) => void;
  readonly tetris_unpause: (a: number) => void;
  readonly tetris_is_paused: (a: number) => number;
  readonly tetris_move_left: (a: number) => void;
  readonly tetris_move_right: (a: number) => void;
  readonly tetris_move_down: (a: number) => number;
  readonly tetris_drop: (a: number) => void;
  readonly tetris_rotate_left: (a: number) => void;
  readonly tetris_rotate_right: (a: number) => void;
  readonly tetris_update_clearing_animation: (a: number, b: number) => void;
  readonly tetris_draw: (a: number) => void;
  readonly tetris_draw_next: (a: number, b: number, c: number) => void;
  readonly tetris_get_score: (a: number) => number;
  readonly tetris_is_game_over: (a: number) => number;
  readonly tetris_get_cleared_lanes: (a: number) => number;
  readonly tetris_get_tetris_count: (a: number) => number;
  readonly tetris_get_triple_count: (a: number) => number;
  readonly tetris_get_double_count: (a: number) => number;
  readonly tetris_get_single_count: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
