// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { invoke, isTauri } from "@tauri-apps/api/core";
import { createSignal, Show } from "solid-js";
import "./ParametersSection.css";

interface SParameters {
  s11: [number, number];
  s21: [number, number];
  s12: [number, number];
  s22: [number, number];
}

interface SimulationResults {
  waveImpedance: number;
  differentialImpedance: number;
  effectiveDielectricConstant: number;
  crosstalk: number;
  skew: number;
  standingWaveRatio: number;
  sParameters: SParameters;
}

export function ParametersSection() {
  const [parameters, setParameters] = createSignal<SimulationResults | null>(
    null,
  );
  const [error, setError] = createSignal<string | null>(null);

  const getSimulationParameters = async (): Promise<SimulationResults> => {
    if (isTauri()) {
      const [
        waveImpedance,
        differentialImpedance,
        effectiveDielectricConstant,
        crosstalk,
        skew,
        standingWaveRatio,
        sParameters,
      ] = await Promise.all([
        invoke<number>("get_wave_impedance"),
        invoke<number>("get_differential_impedance"),
        invoke<number>("get_effective_dielectric_constant"),
        invoke<number>("get_crosstalk"),
        invoke<number>("get_skew"),
        invoke<number>("get_standing_wave_ratio"),
        invoke<SimulationResults["sParameters"]>("get_s_parameters"),
      ]);

      return {
        waveImpedance,
        differentialImpedance,
        effectiveDielectricConstant,
        crosstalk,
        skew,
        standingWaveRatio,
        sParameters,
      };
    } else {
      return Promise.reject(new Error("WASM version yet to be implemented"));
    }
  };

  async function onSimulationFinished() {
    setParameters(null);
    setError(null);

    try {
      const data = await getSimulationParameters();
      setParameters(data);
    } catch (err) {
      setError(String(err));
    }
  }

  return (
    <section>
      <h2>Parameters</h2>
      <button id="parameters-button" onClick={onSimulationFinished}>
        Click to test communication
      </button>
      <div>
        <Show when={parameters()}>
          <div>{parameters()?.waveImpedance}</div>
          <div>{parameters()?.differentialImpedance}</div>
          <div>{parameters()?.effectiveDielectricConstant}</div>
          <div>{parameters()?.crosstalk}</div>
          <div>{parameters()?.skew}</div>
          <div>{parameters()?.standingWaveRatio}</div>
          <div>{JSON.stringify(parameters()?.sParameters)}</div>
        </Show>
        <Show when={error()}>
          <div>{error()}</div>
        </Show>
      </div>
    </section>
  );
}
