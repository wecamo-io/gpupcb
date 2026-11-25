/* @ts-self-types="./gpupcb.d.ts" */
import * as wasm from "./gpupcb_bg.wasm";
import { __wbg_set_wasm } from "./gpupcb_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    get_crosstalk_with_wasm, get_differential_impedance_with_wasm, get_effective_dielectric_constant_with_wasm, get_s_parameters_with_wasm, get_skew_with_wasm, get_standing_wave_ratio_with_wasm, get_wave_impedance_with_wasm, load_hyperlynx_with_wasm, set_frequency_with_wasm, set_timestep_with_wasm
} from "./gpupcb_bg.js";
