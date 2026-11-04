// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { type Accessor, createSignal, For, type JSX, Show } from "solid-js";
import type {
  Arc,
  Board,
  Curve,
  Device,
  Layer,
  Net,
  Pad,
  Padstack,
  Perimeter,
  Pin,
  Polygon,
  Polyline,
  Polyvoid,
  Segment,
  UnroutedSegment,
  Via,
} from "../types";

const PAGE = 200;

// helpers

function show(v: unknown): string {
  if (v === null || v === undefined) return "—";
  if (typeof v === "object") return JSON.stringify(v);
  return String(v);
}

// one line in form of key: value
function Row(props: { f: Record<string, unknown> }) {
  return (
    <div>
      {Object.entries(props.f)
        .map(([k, v]) => `${k}: ${show(v)}`)
        .join("  ·  ")}
    </div>
  );
}

function Section(props: { title: string; children: JSX.Element }) {
  const [opened, setOpened] = createSignal(false);
  return (
    <details onToggle={(e) => e.currentTarget.open && setOpened(true)}>
      <summary>{props.title}</summary>
      <Show when={opened()}>
        <div style={{ "padding-left": "1rem" }}>{props.children}</div>
      </Show>
    </details>
  );
}

function Group<T>(props: {
  title: string;
  items: T[] | null | undefined;
  render: (item: T) => JSX.Element;
}) {
  const [limit, setLimit] = createSignal(PAGE);
  const items = () => props.items ?? [];
  return (
    <Section title={`${props.title} (${items().length})`}>
      <For each={items().slice(0, limit())}>{(item) => props.render(item)}</For>
      <Show when={items().length > limit()}>
        <button type="button" onClick={() => setLimit((l) => l + PAGE)}>
          Show more ({items().length - limit()} left)
        </button>
      </Show>
    </Section>
  );
}

function PerimeterView(p: Perimeter) {
  return <Row f={{ x1: p.x1, y1: p.y1, x2: p.x2, y2: p.y2, type: p.type }} />;
}

function LayerView(l: Layer) {
  const kind = Object.keys(l.layerType ?? {})[0];
  return (
    <Section title={`${show(l.layerName)}  (${show(kind)})`}>
      <Row
        f={{
          layerName: l.layerName,
          layerType: l.layerType,
          thickness: l.thickness,
          material: l.material,
        }}
      />
    </Section>
  );
}

function DeviceView(d: Device) {
  return <Row f={{ reference: d.reference, layer: d.layer }} />;
}

function PadView(p: Pad) {
  return (
    <Row
      f={{
        layer: p.layer,
        shape: p.shape,
        sizeX: p.sizeX,
        sizeY: p.sizeY,
        angle: p.angle,
        type: p.type,
      }}
    />
  );
}

function PadstackView(ps: Padstack) {
  return (
    <Section title={`Padstack #${ps.id}  (drill ${show(ps.drill)})`}>
      <Row f={{ id: ps.id, drill: ps.drill }} />
      <Group title="pads" items={ps.pads} render={PadView} />
    </Section>
  );
}

function PinView(p: Pin) {
  return (
    <Row
      f={{
        componentReference: p.componentReference,
        x: p.x,
        y: p.y,
        padstackId: p.padstackId,
      }}
    />
  );
}

function ViaView(v: Via) {
  return <Row f={{ x: v.x, y: v.y, padstackId: v.padstackId }} />;
}

function SegmentView(s: Segment) {
  return (
    <Row
      f={{
        layer: s.layer,
        x1: s.x1,
        y1: s.y1,
        x2: s.x2,
        y2: s.y2,
        width: s.width,
      }}
    />
  );
}

function UnroutedSegmentView(us: UnroutedSegment) {
  return (
    <Row
      f={{
        layer1: us.layer1,
        x1: us.x1,
        y1: us.y1,
        layer2: us.layer2,
        x2: us.x2,
        y2: us.y2,
        z: us.z,
        delay: us.delay,
        resistance: us.resistance,
      }}
    />
  );
}

function ArcView(a: Arc) {
  return (
    <Row
      f={{
        layer: a.layer,
        x1: a.x1,
        y1: a.y1,
        x2: a.x2,
        y2: a.y2,
        xc: a.xc,
        yc: a.yc,
        radius: a.radius,
        width: a.width,
      }}
    />
  );
}

function CurveView(c: Curve) {
  return (
    <Row
      f={{
        x1: c.x1,
        y1: c.y1,
        x2: c.x2,
        y2: c.y2,
        xc: c.xc,
        yc: c.yc,
        radius: c.radius,
      }}
    />
  );
}

function PolygonView(p: Polygon) {
  return (
    <Section
      title={`Polygon #${p.id}  (${show(p.polygonType)}, ${show(p.layer)})`}
    >
      <Row
        f={{
          id: p.id,
          polygonType: p.polygonType,
          layer: p.layer,
          x: p.x,
          y: p.y,
        }}
      />
      <Group
        title="lines"
        items={p.lines}
        render={(l) => <Row f={{ x: l?.[0], y: l?.[1] }} />}
      />
      <Group title="curves" items={p.curves} render={CurveView} />
    </Section>
  );
}

function PolyvoidView(p: Polyvoid) {
  return (
    <Section title={`Polyvoid #${p.id}`}>
      <Row f={{ id: p.id, x: p.x, y: p.y }} />
      <Group
        title="lines"
        items={p.lines}
        render={(l) => <Row f={{ x: l?.[0], y: l?.[1] }} />}
      />
      <Group title="curves" items={p.curves} render={CurveView} />
    </Section>
  );
}

function PolylineView(p: Polyline) {
  return (
    <Section title={`Polyline #${p.id}`}>
      <Row
        f={{
          layer: p.layer,
          width: p.width,
          id: p.id,
          x: p.x,
          y: p.y,
        }}
      />
      <Group
        title="lines"
        items={p.lines}
        render={(l) => <Row f={{ x: l?.[0], y: l?.[1] }} />}
      />
      <Group title="curves" items={p.curves} render={CurveView} />
    </Section>
  );
}

function NetView(n: Net) {
  return (
    <Section title={n.name}>
      <Row f={{ name: n.name }} />
      <Group title="pins" items={n.pins} render={PinView} />
      <Group title="vias" items={n.vias} render={ViaView} />
      <Group title="segments" items={n.segments} render={SegmentView} />
      <Group
        title="unroutedSegments"
        items={n.unroutedSegments}
        render={UnroutedSegmentView}
      />
      <Group title="arcs" items={n.arcs} render={ArcView} />
      <Group title="polygons" items={n.polygons} render={PolygonView} />
      <Group title="polyvoids" items={n.polyvoids} render={PolyvoidView} />
      <Group title="polylines" items={n.polylines} render={PolylineView} />
    </Section>
  );
}

export function BoardAccordion(props: { board: Accessor<Board | null> }) {
  return (
    <Show when={props.board()}>
      {(board) => (
        <div style={{ "text-align": "left" }}>
          <Section title="Info">
            <Row f={{ version: board().version, units: board().units }} />
          </Section>
          <Group
            title="outline"
            items={board().outline}
            render={PerimeterView}
          />
          <Group title="stackup" items={board().stackup} render={LayerView} />
          <Group title="nets" items={board().nets} render={NetView} />
          <Group title="devices" items={board().devices} render={DeviceView} />
          <Group
            title="padstacks"
            items={board().padstacks}
            render={PadstackView}
          />
        </div>
      )}
    </Show>
  );
}
