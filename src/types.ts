// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

export type Units = "english" | "metric";

export type LayerType =
  | { dielectric: { dielectricConstant: number } }
  | { signal: { resistivity: number; platingThickness: number } }
  | { plane: { resistivity: number } };

export type LayerMaterial =
  | "SolderMask"
  | "COPPER"
  | "FR4"
  | "PR1080"
  | "PR2116"
  | "Core009"
  | "PP006"
  | "PP014"
  | "Core018"
  | "CompositeDielectric"
  | "Polyimide"
  | { unknownMaterial: string };

export type PadShape = "Oval" | "Roundrect" | "Rectangle";

export type PadType =
  | "M"
  | "A"
  | {
      t: {
        shape: PadShape;
        sizeX: number;
        sizeY: number;
        angle: number;
      };
    };

export type PolygonType = "Pour" | "COPPER";

export type PerimeterType =
  "segment" | { arc: { xc: number; yc: number; radius: number } };

export interface Perimeter {
  x1: number;
  x2: number;
  y1: number;
  y2: number;
  type: PerimeterType;
}

export interface Layer {
  layerType: LayerType;
  thickness: number;
  layerName: string;
  material: LayerMaterial;
}

export interface Device {
  reference: string;
  layer: string;
}

export interface Pad {
  layer: string;
  shape: PadShape;
  sizeX: number;
  sizeY: number;
  angle: number;
  type: PadType;
}

export interface Padstack {
  id: number;
  drill: number;
  pads: Pad[];
}

export interface Pin {
  x: number;
  y: number;
  componentReference: string;
  padstackId: number;
}

export interface Via {
  x: number;
  y: number;
  padstackId: number;
}

export interface Segment {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
  width: number;
  layer: string;
}

export interface UnroutedSegment {
  x1: number;
  y1: number;
  layer1: string;
  x2: number;
  y2: number;
  layer2: string;
  z: number;
  delay: number;
  resistance: number;
}

export interface Arc {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
  xc: number;
  yc: number;
  radius: number;
  width: number;
  layer: string;
}

export interface Curve {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
  xc: number;
  yc: number;
  radius: number;
}

export interface Polygon {
  polygonType: PolygonType;
  layer: string;
  id: number;
  x: number;
  y: number;
  lines: [number, number][];
  curves: Curve[];
}

export interface Polyvoid {
  id: number;
  x: number;
  y: number;
  lines: [number, number][];
  curves: Curve[];
}

export interface Polyline {
  layer: string;
  width: number;
  id: number;
  x: number;
  y: number;
  lines: [number, number][];
  curves: Curve[];
}

export interface Net {
  name: string;
  pins: Pin[];
  vias: Via[];
  segments: Segment[];
  unroutedSegments: UnroutedSegment[];
  arcs: Arc[];
  polygons: Polygon[];
  polyvoids: Polyvoid[];
  polylines: Polyline[];
}

export interface Board {
  version: string;
  units: Units;
  outline: Perimeter[];
  stackup: Layer[];
  nets: Net[];
  devices: Device[];
  padstacks: Padstack[];
}
