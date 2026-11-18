# Simulation

## 1. Introduction

The goal is to simulate electromagnetic phenomena inside and around PCBs, including:

- Electric and magnetic field propagation.
- Electromagnetic coupling between traces.
- Skin and proximity effect.
- Current distribution in copper traces.
- Signal propagation and reflections.
- Power dissipation.

The project uses two numerical methods:

- Finite-Difference Time-Domain (**FDTD**).
- Finite-Difference Frequency-Domain (**FDFD**).

These methods provide complementary approaches to electromagnetic analysis.

## 2. Foundations

The EM simulation is based on Maxwell's equations:

### Gauss' Law for Electricity
∇ · D = ρ

This equation describes the relationship between electric displacement and electric charge density.

### Gauss' Law for Magnetism
∇ · B = 0

This equation states that magnetic monopoles do not exist and they always create closed loops.

### Faraday's Law
∇ × E = −∂B / ∂t

A changing magnetic field produces an electric field.

### Ampère-Maxwell Law
∇ × H = J + ∂D / ∂t

Electric currents and changing electric fields produce magnetic fields.

Where:

---

- D = εE
- B = μH
- J = σE

---

- E is the electric field.
- D is the electric displacement field.
- H is the magnetic field.
- B is the magnetic flux density.
- J is the electric current density.
- μ is the magnetic permeability.
- ρ is the electric charge density.
- ε is the electrical permittivity.
- σ is the electrical conductivity.

## 3. FDTD – Finite-Difference Time-Domain

### 3.1 Overview
FDTD is a numerical method used to solve Maxwell's equations directly in the time domain.

Instead of calculating electromagnetic behaviour at only one frequency, FDTD calculates how electric and magnetic fields evolve over time.

The simulation space is divided into a three-dimensional grid.

Each grid cell represents a small portion of physical space with defined electromagnetic material properties.

The simulation advances through discrete time steps, constantly updating electric and magnetic fields' values.

### 3.2 Yee Grid
A common spatial discretization used in FDTD is the Yee grid.

Electric and magnetic field components are stored at different positions within each computational cell.

This arrangement allows Maxwell's 3rd and 4th equations to be discretized efficiently while preserving their mathematical correctness.

The grid contains:
- Electric field components: Eₓ, Eᵧ and Eᵤ.
- Magnetic field components: Hₓ, Hᵧ and Hᵤ.
- Material parameters for each relevant grid location.

The spatial resolution determines how accurately electromagnetic structures can be represented. For PCB EM simulation, the resolution must be sufficient to represent copper traces, dielectric layers, vias, components, etc. It also needs to be non-uniform in this case. meaning the grid has to be denser inside and around copper traces.

### 3.3 Time Stepping
FDTD uses an iterative time-stepping algorithm.

Each simulation step consists of two substepts:
1. Update magnetic fields using the neighbouring values of electric field.
2. Update electric fields using the neighbouring values of magnetic field.

This way of calculating new values is called leapfrogging.

### 3.4 Numerical Stability
FDTD requires a stable relationship between spatial resolution and time-step size.

This restriction is commonly known as the [Courant-Friedrichs-Lewy](https://en.wikipedia.org/wiki/Courant%E2%80%93Friedrichs%E2%80%93Lewy_condition) condition.

For a three-dimensional grid it is:

C = Δt · (uₓ/Δx + uᵧ/Δy + u_z/Δz) ≤ Cₘₐₓ

---

Where:
- C = Courant number.
- Δt = simulation time step (s).
- uₓ, uᵧ, u_z = propagation velocities along the X, Y and Z axes (m/s).
- Δx, Δy, Δz = spatial grid spacing along the X, Y and Z axes (m).
- Cₘₐₓ = maximum allowed Courant number for the numerical scheme.

---

An excessively large time step can cause numerical instability.

***Note:*** A single transient simulation can provide information about a range of frequencies after Fast Fourier Transform.

## 4. FDFD – Finite-Difference Frequency-Domain

### 4.1 Overview
[FDFD](https://empossible.net/fdfdbook/) is a numerical method that solves Maxwell's equations directly in the frequency domain.

Instead of calculating field values in next cells over time, it calculates electromagnetic fields at selected frequency.

The frequency-domain Maxwell equations become:

∇ × E = −jωμH

∇ × H = J + jωεE

These equations are discretized into a system of linear equations that can be written as:

A · x = b

### 4.2 Advantages of FDFD
FDFD is useful for analysing electromagnetic behaviour at specific frequencies.

It can be applied to:
- Transmission-line analysis.
- Frequency-dependent impedance.
- Resonance detection.
- Electromagnetic coupling.
- Field distributions at selected frequencies.
- Frequency-dependent material properties.

FDFD provides direct access to steady-state EM behaviour at a given frequency.

Unlike FDTD, which requires a time-domain simulation followed by frequency analysis, FDFD directly solves the frequency-domain equations.

## 5. Boundary Conditions

Electromagnetic simulations require boundary conditions to define how fields behave at the edges of the computational 3D space.

**Perfectly Matched Layer (PML)*- is an artificial absorbing boundary designed to remove reflections from the edges of the simulation domain. It allows electromagnetic waves to leave the computational region with minimal or zero reflection back to simulation space. PML is very important when simulating PCBs in open electromagnetic environment.

## 6. Selected EM simulation phenomenas

### 6.1 Skin Effect
Skin effect describes the tendency of current to concentrate near the surface of a conductor as frequency increases.

δ = √(2 / (ωμσ))

Where:

- δ is a skin depth (m).
- ω (2πf) is an angular frequency (rad/s).
- μ is a magnetic permeability (H/m).
- σ is a nelectrical conductivity (S/m).

### 6.2 Proximity Effect
Proximity effect occurs when the electromagnetic field generated by one conductor influences the current distribution in a nearby conductor.

It occurs for closely spaced conductors and differential transmission lines.

### 6.3 Crosstalk
Crosstalk occurs when electromagnetic energy from one signal trace couples into another nearby trace.

The amount of coupling may depend on:
- Distance between traces
- Trace geometry
- Dielectric properties
- Signal frequency

An electromagnetic simulator can calculate the coupled field distribution and help evaluate interference between neighbouring signals.

### 6.4 Signal Reflections
Signal reflections occur when a propagating electromagnetic wave encounters a discontinuity in impedance.

Examples include:
- Changes in trace width
- Vias
- Branching transmission lines

FDTD can visualize the reflected wave propagating back through the PCB.

## 7. Electromagnetic Energy Flow

The Poynting vector describes the direction and magnitude of electromagnetic energy flow that is defined as:

S = E × H

Where:
- S is a Poynting vector (W/m²).
- E is an electric field (V/m).
- H is a magnetic field (A/m).

The Poynting vector is particularly useful for understanding how electromagnetic energy propagates through PCB structures.

It can help visualize energy transfer between transmission lines or dielectric materials.

## 8. Simulation Results and Visualization

The simulation needs to provide access to the calculated electromagnetic values.

Important visualization targets include:
- Electromagnetic wave propagation
- Power dissipation
- Current density
- Poynting-vector direction
- Crosstalk
- Skin effect
- Proximity effect
