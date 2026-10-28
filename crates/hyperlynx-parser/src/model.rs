// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

#[derive(Debug, PartialEq)]
pub enum Units {
    English,
}

#[derive(Debug, PartialEq)]
pub enum LayerType {
    Dielectric {
        dielectric_constant: f64,
    },
    Signal {
        resistivity: f64,
        plating_thickness: f64,
    },
}

#[derive(Debug, PartialEq)]
pub enum LayerMaterial {
    SolderMask,
    COPPER,
    FR4,
    PR1080,
    PR2116,
}

#[derive(Debug, PartialEq)]
pub enum PadShape {
    Oval,
    Roundrect,
    Rectangle,
}

#[derive(Debug, PartialEq)]
pub enum PolygonType {
    Pour,
}

#[derive(Debug, PartialEq)]
pub struct PerimeterSegment {
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
}

#[derive(Debug, PartialEq)]
pub struct Layer {
    pub layer_type: LayerType,
    pub thickness: f64,
    pub layer_name: String,
    pub material: LayerMaterial,
}

#[derive(Debug, PartialEq)]
pub struct Device {
    pub reference: String,
    pub layer: String,
}

#[derive(Debug, PartialEq)]
pub struct Pad {
    pub layer: String,
    pub shape: PadShape,
    pub size_x: f64,
    pub size_y: f64,
    pub angle: f64,
}

#[derive(Debug, PartialEq)]
pub struct Padstack {
    pub id: u32,
    pub drill: f64,
    pub pads: Vec<Pad>,
}

#[derive(Debug, PartialEq)]
pub struct Pin {
    pub x: f64,
    pub y: f64,
    pub component_reference: String,
    pub padstack_id: u32,
}

#[derive(Debug, PartialEq)]
pub struct Via {
    pub x: f64,
    pub y: f64,
    pub padstack_id: u32,
}

#[derive(Debug, PartialEq)]
pub struct Segment {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub width: f64,
    pub layer: String,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Polygon {
    pub polygon_type: PolygonType,
    pub layer: String,
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub outline: Vec<(f64, f64)>,
}

#[derive(Debug, PartialEq)]
pub struct Polyvoid {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub outline: Vec<(f64, f64)>,
}

#[derive(Debug, PartialEq)]
pub struct Net {
    pub name: String,
    pub pins: Vec<Pin>,
    pub vias: Vec<Via>,
    pub segments: Vec<Segment>,
    pub arcs: Vec<Arc>,
    pub polygons: Vec<Polygon>,
    pub polyvoids: Vec<Polyvoid>,
}

#[derive(Debug, PartialEq)]
pub struct Board {
    pub version: String,
    pub units: Units,
    pub outline: Vec<PerimeterSegment>,
    pub stackup: Vec<Layer>,
    pub nets: Vec<Net>,
    pub devices: Vec<Device>,
    pub padstacks: Vec<Padstack>,
}
