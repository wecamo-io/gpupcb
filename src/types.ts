// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

export enum Units {
  English = "English",
  Metric = "Metric",
}

export type LayerType =
  | {
      Dielectric: {
        dielectric_constant: number;
      };
    }
  | {
      Signal: {
        resistivity: number;
        plating_thickness: number;
      };
    }
  | {
      Plane: {
        resistivity: number;
      };
    };

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
  | { UnknownMaterial: string };

export enum PadShape {
  Oval = "Oval",
  Roundrect = "Roundrect",
  Rectangle = "Rectangle",
}

export type PadType =
  | "M"
  | "A"
  | {
      T: {
        shape: PadShape;
        size_x: number;
        size_y: number;
        angle: number;
      };
    };

export type PolygonType = "Pour" | "COPPER";

export type PerimeterType =
  | "Segment"
  | {
      Arc: {
        xc: number;
        yc: number;
        radius: number;
      };
    };

export interface Perimeter {
  x1: number;
  x2: number;
  y1: number;
  y2: number;
  type: PerimeterType;
}

export interface Layer {
  layer_type: LayerType;
  thickness: number;
  layer_name: string;
  material: LayerMaterial;
}

export interface Device {
  reference: string;
  layer: string;
}

export interface Pad {
  layer: string;
  shape: PadShape;
  size_x: number;
  size_y: number;
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
  component_reference: string;
  padstack_id: number;
}

export interface Via {
  x: number;
  y: number;
  padstack_id: number;
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
  polygon_type: PolygonType;
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
  unrouted_segments: UnroutedSegment[];
  arcs: Arc[];
  polygons: Polygon[];
  polyvoids: Polyvoid[];
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
