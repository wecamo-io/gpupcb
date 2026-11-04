// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum Units {
    English, // inches
    Metric,  // centimeters
}

#[derive(Debug, PartialEq, Serialize)]
pub enum LayerType {
    Dielectric {
        dielectric_constant: f64,
    },
    Signal {
        resistivity: f64,
        plating_thickness: f64,
    },
    Plane,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum LayerMaterial {
    SolderMask,
    COPPER,
    FR4,
    PR1080,
    PR2116,
    Core009,
    PP006,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum PadShape {
    Oval,
    Roundrect,
    Rectangle,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum PadType {
    M, // Metal
    A, // Antipad
    T, // Thermal relief
}

#[derive(Debug, PartialEq, Serialize)]
pub enum PolygonType {
    Pour,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum PerimeterType {
    Segment,
    Arc { xc: f64, yc: f64, radius: f64 },
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Perimeter {
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
    pub r#type: PerimeterType,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Layer {
    pub layer_type: LayerType,
    pub thickness: f64,
    pub layer_name: String,
    pub material: LayerMaterial,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Device {
    pub reference: String,
    pub layer: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Pad {
    pub layer: String,
    pub shape: PadShape,
    pub size_x: f64,
    pub size_y: f64,
    pub angle: f64,
    pub r#type: PadType,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Padstack {
    pub id: u32,
    pub drill: f64,
    pub pads: Vec<Pad>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Pin {
    pub x: f64,
    pub y: f64,
    pub component_reference: String,
    pub padstack_id: u32,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Via {
    pub x: f64,
    pub y: f64,
    pub padstack_id: u32,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Segment {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub width: f64,
    pub layer: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct UnroutedSegment {
    pub x1: f64,
    pub y1: f64,
    pub layer1: String,
    pub x2: f64,
    pub y2: f64,
    pub layer2: String,
    pub z: f64, // characteristic_impedance
    pub delay: f64,
    pub resistance: f64,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Arc {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub xc: f64,
    pub yc: f64,
    pub radius: f64,
    pub width: f64,
    pub layer: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Curve {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub xc: f64,
    pub yc: f64,
    pub radius: f64,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Polygon {
    pub polygon_type: PolygonType,
    pub layer: String,
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub lines: Vec<(f64, f64)>,
    pub curves: Vec<Curve>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Polyvoid {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub lines: Vec<(f64, f64)>,
    pub curves: Vec<Curve>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Polyline {
    pub layer: String,
    pub width: f64,
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub lines: Vec<(f64, f64)>,
    pub curves: Vec<Curve>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Net {
    pub name: String,
    pub pins: Vec<Pin>,
    pub vias: Vec<Via>,
    pub segments: Vec<Segment>,
    pub unrouted_segments: Vec<UnroutedSegment>,
    pub arcs: Vec<Arc>,
    pub polygons: Vec<Polygon>,
    pub polyvoids: Vec<Polyvoid>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Board {
    pub version: String,
    pub units: Units,
    pub outline: Vec<Perimeter>,
    pub stackup: Vec<Layer>,
    pub nets: Vec<Net>,
    pub devices: Vec<Device>,
    pub padstacks: Vec<Padstack>,
}
