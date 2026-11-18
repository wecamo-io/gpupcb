# HyperLynx File Format

SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>

SPDX-License-Identifier: GPL 3.0

---

## 1. Introduction

HyperLynx (.hyp) is a text format used to represent PCB design.

It describes the physical and electrical structure of a PCB, including:
- Board outline and perimeter geometry
- Layer stackup and layers' properties
- Component placement references
- Padstack definitions and pad geometry
- Electrical nets and their connections
- Tracks, arcs, vias and copper polygons
- Unrouted connections

This document describes the elements represented by GPUPCB's hyperlynx-parser crate.

## 2. File structure

A HyperLynx exchange file consists of named blocks enclosed in curly braces.

A typical file follows this logical structure:

```
{VERSION=2.14}
* optional
{DATA_MODE=...}
{UNITS=... LENGTH}

{BOARD
  (PERIMETER_SEGMENT X1=VAL1 Y1=VAL2 X2=VAL3 Y2=VAL4)
  ...
}

{STACKUP
  (DIELECTRIC T=VAL1 C=VAL2 L=VAL3 M=VAL4)
  (SIGNAL T=VAL1 P=VAL2 C=VAL3 L=VAL4 M=VAL5)
  ...
}

{DEVICES
  (? REF=VAL1 NAME=OPTVAL2 L=VAL3)
  ...
}

{PADSTACK=1
  (VAL1, VAL2, VAL3, VAL4, VAL5, VAL6)
    ...
}

{PADSTACK=2
    (VAL1, VAL2, VAL3, VAL4, VAL5, VAL6)
    (VAL1, VAL2, VAL3, VAL4, VAL5, VAL6)
    ...
}

{NET=GND
    (PIN X=VAL1 Y=VAL2 R=VAL3 P=VAL4)
    (VIA X=VAL1 Y=VAL2 P=VAL3)
    (SEG X1=VAL1 Y1=VAL2 X2=VAL3 Y2=VAL4 W=VAL5 L=VAL6)
    (USEG X1=VAL1 Y1=VAL2 L1=VAL3 X2=VAL4 Y2=VAL5 L2=VAL6 Z=VAL7 D=VAL8 R=VAL9)
    (ARC X1=VAL1 Y1=VAL2 X2=VAL3 Y2=VAL4 XC=VAL5 YC=VAL6 R=VAL7 W=VAL8 L=VAL9)
    {POLYGON T=VAL1 L=VAL2 ID=VAL3 X=VAL4 Y=VAL5
      (LINE X=VAL1 Y=VAL2)
      ...
    }
    {POLYVOID ID=VAL1 X=VAL2 Y=VAL3
      (LINE X=VAL1 Y=VAL2)
      ...
    }
    {POLYLINE L=VAL1 W=VAL2 ID=VAL3 X=VAL4 Y=VAL5
      (LINE X=VAL1 Y=VAL2)
      ...
    }
}

{NET=SIGNAL_1
    ...
}

* optional
{END}
```

## 3. Block syntax
Blocks use curly braces:

```
{BLOCK_NAME}
{BLOCK_NAME=VALUE}
{BLOCK_NAME VALUE}
{BLOCK_NAME=VALUE, ANOTHER_VALUE}
```

Some blocks  like {NET} contain nested blocks like {POLYGON}.

The parser processes blocks in a defined order. The following top-level blocks are recognized:

| Block | Purpose |
|---|---|
| VERSION | Format version |
| DATA_MODE | Optional data mode declaration |
| UNITS | Unit system used in file (metric, english) |
| BOARD | Physical outline of the board |
| STACKUP | PCB layer construction |
| DEVICES | Component references and their placement |
| PADSTACK | Padstack definitions |
| NET | Electrical connections and geometry |

## 4 Blocks explained

### 4.1 Version

The file begins with a version declaration:

```
{VERSION=2.14}
```

The parser accepts a version value consisting of two numeric parts separated by a dot.

In GPUPCB it is stored as a string value.

### 4.2 Data mode

An optional data-mode declaration may appear after Version block:

```
{DATA_MODE=...}
```

The current parser skips this declaration and does not include it anywhere in returned data.

### 4.3 Units

The units declaration specifies the system used by the board.

#### 4.3.1 English units

```
{UNITS=ENGLISH LENGTH}
```

The parser represents this as:

```rust
Units::English
```

English dimensions are expressed in inches.

#### 4.3.2 Metric units

```
{UNITS=METRIC LENGTH}
```

The parser represents this as:

```rust
Units::Metric
```

Metric dimensions are expressed in centimeters.

#### 4.3.3 Internal conversion
For simulation and calculations, dimensions should be normalized to a common unit (metric by default).

### 4.4 Board
The Board block describes the PCB's physical shape.

The parser represents the outline as a collection of Perimeter objects.

Each perimeter element contains:
- x1, y1: first endpoint.
- x2, y2: second endpoint.
- A perimeter type.

#### 4.4.1 Straight segments
A straight perimeter element is represented by:


```rust
PerimeterType::Segment
```
It connects two points:


#### 4.4.2 Circular arcs
An arc is represented by:


```rust
PerimeterType::Arc {
    xc,
    yc,
    radius
}
```

The arc contains:

- Start point
- End point
- Center coordinates
- Radius

The center is C = (x_c, y_c)

and the radius is r

The order and continuity of perimeter elements are important for reconstructing  boundaries.

### 4.5 Stackup
The stackup block describes physical construction of the PCB.

The parser represents each layer using:


```rust
pub struct Layer {
    pub layer_type: LayerType,
    pub thickness: f64,
    pub layer_name: String,
    pub material: LayerMaterial,
}
```

#### 4.5.1 Layer types
Three layer types are represented.

Dielectric


```rust
LayerType::Dielectric {
    dielectric_constant
}
```

Signal

```rust
LayerType::Signal {
    resistivity,
    plating_thickness
}
```

Plane

```rust
LayerType::Plane {
    resistivity
}
```

#### 4.5.2 Layer materials
The parser recognizes the following material identifiers in its internal model:

- SolderMask
- COPPER
- FR4	FR-4 
- PR1080	 
- PR2116	 
- Core009	
- PP006	
- PP014	
- Core018	C
- CompositeDielectric	
- Polyimide	
- UnknownMaterial(String)

Unknown material names should generate some kind of warning instead of silently being mapped to a known material.

### 4.6 Devices
The DEVICES block describes component references.

The internal representation is:

```rust
pub struct Device {
    pub reference: String,
    pub layer: String,
}
```

Each device contains:
- reference	
- layer	

Examples of reference designators include U1, R12, C5 and J1.

The device declaration provides component identification and layer association. Pin positions are represented separately through NET blocks.

### 4.7 Padstacks

The PADSTACK block describes the physical construction of pads and drilled connections.

Each padstack is identified by a numeric ID:

```
{PADSTACK=1
    ...
}
```
The internal representation is:

```rust
pub struct Padstack {
    pub id: u32,
    pub drill: f64,
    pub pads: Vec<Pad>,
}
```

Each padstack contains:
- id: Unique padstack identifier
- drill: Drill diameter
- pads: Collection of pad definitions associated with the padstack

#### 4.7.1 Pad definition

Each pad contains:

```rust
pub struct Pad {
    pub layer: String,
    pub shape: PadShape,
    pub size_x: f64,
    pub size_y: f64,
    pub angle: f64,
    pub r#type: PadType,
}
```

The pad definition includes:
- layer: Layer on which the pad is located
- shape: Geometric shape of the pad
- size_x: Pad dimension along the X axis
- size_y: Pad dimension along the Y axis
- angle: Pad rotation angle
- type: Pad type

#### 4.7.2 Pad shapes

The parser's internal model supports the following pad shapes:
- Oval
- Roundrect
- Rectangle
- 
#### 4.7.3 Pad types

The parser supports three pad types:

Metal:

```rust
PadType::M
```

Antipad:

```rust
PadType::A
```

Thermal relief:

```rust
PadType::T {
    shape,
    size_x,
    size_y,
    angle
}
```

### 4.8 Nets

The NET block describes electrical connections and associated PCB geometry.

Each net is identified by a name:

```
{NET=GND
    ...
}
```

The internal representation is:

```rust
pub struct Net {
    pub name: String,
    pub pins: Vec<Pin>,
    pub vias: Vec<Via>,
    pub segments: Vec<Segment>,
    pub unrouted_segments: Vec<UnroutedSegment>,
    pub arcs: Vec<Arc>,
    pub polygons: Vec<Polygon>,
    pub polyvoids: Vec<Polyvoid>,
    pub polylines: Vec<Polyline>,
}
```

A net groups all its electrical connections and associated physical geometry.

Each net contains:
- name: Net name
- pins: Component pins connected to the net
- vias: Via connections
- segments: Straight track segments
- unrouted_segments: Connections that are not represented as ordinary routed tracks
- arcs: Curved track segments
- polygons: Copper polygon geometry
- polyvoids: Voids inside copper polygons
- polylines: Connected sequences of straight and curved elements

#### 4.8.1 Pins

A pin is represented by:

```rust
pub struct Pin {
    pub x: f64,
    pub y: f64,
    pub component_reference: String,
    pub padstack_id: u32,
}
```

Each pin contains:
- x, y: Pin coordinates
- component_reference: Reference designator of the associated component
- padstack_id: Identifier of the associated padstack

The component reference identifies the device, while the padstack ID associates the pin with its physical pad geometry.

#### 4.8.2 Vias

A via is represented by:

```rust
pub struct Via {
    pub x: f64,
    pub y: f64,
    pub padstack_id: u32,
}
```

Each via contains:
- x, y: Via coordinates
- padstack_id: Identifier of the associated padstack

Vias provide electrical connections between layers of the PCB. Their physical geometry is determined by the associated padstack definition.

#### 4.8.3 Straight track segments

A straight track segment is represented by:

```rust
pub struct Segment {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub width: f64,
    pub layer: String,
}
```

Each segment contains:
- x1, y1: Starting coordinates
- x2, y2: Ending coordinates
- width: Track width
- layer: Layer on which the track is located

The segment connects two points:

P1 = (x1, y1)
P2 = (x2, y2)

#### 4.8.4 Unrouted segments

An unrouted segment is represented by:

```rust
pub struct UnroutedSegment {
    pub x1: f64,
    pub y1: f64,
    pub layer1: String,
    pub x2: f64,
    pub y2: f64,
    pub layer2: String,
    pub z: f64,
    pub delay: f64,
    pub resistance: f64,
}
```

Each unrouted segment contains:
- x1, y1: Starting coordinates
- layer1: Starting layer
- x2, y2: Ending coordinates
- layer2: Ending layer
- z: Characteristic impedance
- delay
- resistance: Electrical resistance

Unrouted segments describe electrical connections that are not represented as normally routed copper tracks.

#### 4.8.5 Circular arcs

An arc is represented by:

```rust
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
```

Each arc contains:
- x1, y1: Starting coordinates
- x2, y2: Ending coordinates
- xc, yc: Center coordinates
- radius: Arc radius
- width: Track width
- layer: Layer on which the arc is located

The arc describes a curved conductive track.

Its circular geometry follows the equation:

(x - xc)² + (y - yc)² = radius²

The start and end coordinates define the portion of the circle used by the arc.

#### 4.8.6 Polygons

A polygon is represented by:

```rust
pub struct Polygon {
    pub polygon_type: PolygonType,
    pub layer: String,
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub lines: Vec<(f64, f64)>,
    pub curves: Vec<Curve>,
}
```

Each polygon contains:
- polygon_type: Polygon type
- layer: Layer on which the polygon is located
- id: Polygon identifier
- x, y: Reference coordinates
- lines: Straight boundary elements
- curves: Curved boundary elements

The parser supports the following polygon types:

```rust
PolygonType::Pour
PolygonType::COPPER
```

Their geometry is constructed from straight and curved boundary elements.

#### 4.8.7 Polygon voids

A polygon void is represented by:

```rust
pub struct Polyvoid {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub lines: Vec<(f64, f64)>,
    pub curves: Vec<Curve>,
}
```

Each polygon void contains:
- id: Associated polygon identifier
- x, y: Reference coordinates
- lines: Straight boundary elements
- curves: Curved boundary elements

Polygon voids describe areas excluded from copper polygon geometry.

#### 4.8.8 Polylines

A polyline is represented by:

```rust
pub struct Polyline {
    pub layer: String,
    pub width: f64,
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub lines: Vec<(f64, f64)>,
    pub curves: Vec<Curve>,
}
```

Each polyline contains:
- layer: Layer on which the polyline is located
- width: Polyline width
- id: Polyline identifier
- x, y: Reference coordinates
- lines: Straight boundary elements
- curves: Curved boundary elements

A polyline represents a connected sequence of straight and curved geometric elements.

#### 4.8.9 Curves

Curved geometry used by polygons and polylines is represented by:

```rust
pub struct Curve {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub xc: f64,
    pub yc: f64,
    pub radius: f64,
}
```

Each curve contains:
- x1, y1: Starting coordinates
- x2, y2: Ending coordinates
- xc, yc: Center coordinates
- radius: Curve radius

The curve describes a circular boundary element.

The start and end coordinates determine its endpoints, while the center and radius define its circular geometry.

#### 4.8.10 Connectivity

All objects contained in a NET block belong to the same network.

During import, GPUPCB preserves these relationships so that electrical connectivity remains consistent with the original PCB design.
