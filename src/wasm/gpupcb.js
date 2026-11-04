/* @ts-self-types="./gpupcb.d.ts" */
import * as wasm from "./gpupcb_bg.wasm";
import { __wbg_set_wasm } from "./gpupcb_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    load_hyperlynx_with_wasm
} from "./gpupcb_bg.js";
