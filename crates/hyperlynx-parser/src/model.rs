// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

enum Units {
    English,
}

enum LayerType {
    Dielectric { dielectric_constant: f64 },
    Signal { resistivity: f64 },
}

enum LayerMaterial {
    SolderMask,
    COPPER,
    FR4,
    PR1080,
    PR2116,
}

enum PadShape {
    Oval,
    Roundrect,
    Rectangle,
}

enum PolygonType {
    Pour,
}

struct PerimeterSegment {
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

struct Layer {
    layer_type: LayerType,
    thickness: f64,
    layer_name: String,
    material: LayerMaterial,
}

struct Device {
    reference: String,
    layer: String,
}

struct Pad {
    layer: String,
    shape: PadShape,
    size_x: f64,
    size_y: f64,
    angle: f64,
}

struct Padstack {
    id: u32,
    drill: f64,
    pads: Vec<Pad>,
}

struct Pin {
    x: f64,
    y: f64,
    component_reference: String,
    padstack_id: u32,
}

struct Via {
    x: f64,
    y: f64,
    padstack_id: u32,
}

struct Segment {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    width: f64,
    layer: String,
}

struct Arc {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    xc: f64,
    yc: f64,
    radius: f64,
    width: f64,
    layer: String,
}

struct Polygon {
    polygon_type: PolygonType,
    layer: String,
    id: u32,
    x: f64,
    y: f64,
    outline: Vec<(f64, f64)>,
}

struct Polyvoid {
    id: u32,
    x: f64,
    y: f64,
    outline: Vec<(f64, f64)>,
}

struct Net {
    name: String,
    pins: Vec<Pin>,
    vias: Vec<Via>,
    segments: Vec<Segment>,
    arcs: Vec<Arc>,
    polygons: Vec<Polygon>,
    polyvoids: Vec<Polyvoid>,
}

struct Board {
    version: String,
    units: Units,
    outline: Vec<PerimeterSegment>,
    stackup: Vec<Layer>,
    nets: Vec<Net>,
    devices: Vec<Device>,
    padstacks: Vec<Padstack>,
}
