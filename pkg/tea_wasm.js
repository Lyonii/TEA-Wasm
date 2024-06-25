import * as wasm from "./tea_wasm_bg.wasm";
import { __wbg_set_wasm } from "./tea_wasm_bg.js";
__wbg_set_wasm(wasm);
export * from "./tea_wasm_bg.js";
