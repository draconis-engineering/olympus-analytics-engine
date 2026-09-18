# Olympus Analytics Engine

> Local-first analytics and data platform for Olympus.

Olympus Analytics Engine (OAE) is the analytical backbone of the Olympus ecosystem. It provides data processing, storage, analytics, and a local web interface for exploring training and performance data.

OAE is designed around a simple principle:

**Your data stays local.**

No cloud account. No mandatory synchronization. No external processing.

---

## Overview

OAE sits between Olympus data and the user-facing analytics interface.

```text
                    OLYMPUS
                       │
             ┌─────────┴─────────┐
             │                   │
        Olympus Terminal       OAE
             │                   │
        Device / Session     Analytics
             │               Storage
             │               API
             │                 │
             └───────┬─────────┘
                     │
                  SQLite
                     │
                     ▼
              Local Web Server
                     │
                     ▼
               React + Vite
                     │
                     ▼
                  Browser
```

The backend is written in Rust, while the user interface is built with React, TypeScript, and Vite.

---

## Goals

OAE aims to provide:

* Local-first training data storage
* High-performance analytics
* A unified Olympus data model
* Support for multiple training data formats
* Historical performance analysis
* Anomaly detection
* Athlete and session analytics
* A modern engineering-focused user interface
* A stable API for future Olympus and DragonSuite applications

OAE should prioritize **useful analysis over metric overload**.

---

## Architecture

OAE is structured as a Rust workspace with a separate web frontend.

```text
oae/
├── crates/
│   ├── oae-core/
│   ├── oae-data/
│   ├── oae-analytics/
│   ├── oae-storage/
│   └── oae-api/
│
├── src/
│   └── main.rs
│
├── web/
│   └── React + TypeScript + Vite
│
└── docs/
```

### `oae-core`

Shared primitives and foundational functionality.

Examples:

* IDs
* Time
* Units
* Configuration
* Common errors

### `oae-data`

The canonical Olympus data model.

This crate defines how OAE represents:

* Activities
* Workouts
* Athletes
* Sensors
* Time-series data
* Power
* Heart rate
* Cadence
* Pace
* Sport-specific data

The goal is to provide a common representation independent of the original data format.

### `oae-analytics`

The analytical engine.

Potential modules include:

* Power analysis
* Heart-rate analysis
* Training load
* Performance records
* HRV
* Resting heart rate
* Power-duration curves
* Anomaly detection
* Historical trends

Analytics should operate on Olympus data models rather than directly accessing storage or HTTP.

### `oae-storage`

Persistence layer.

Responsible for:

* SQLite
* Database migrations
* Activity storage
* Athlete data
* Workout data
* Querying and persistence

SQLite acts as the local source of truth for OAE.

### `oae-api`

The interface between the OAE backend and external clients.

The primary consumer is the OAE web application.

Planned interfaces include:

* REST API
* WebSocket API
* Live telemetry
* Activity endpoints
* Analytics endpoints
* Athlete endpoints
* Record endpoints
* Anomaly endpoints

### `src/`

The OAE executable.

The executable is responsible for assembling the components and starting the local OAE runtime.

Eventually, this will power:

```bash
olympus analytics
```

---

## Web Interface

The OAE web interface is built using:

* React
* TypeScript
* Vite
* Bklit UI

The interface is intentionally designed around an engineering-oriented visual language rather than a conventional fitness-dashboard aesthetic.

The goal is to make OAE feel more like a **performance engineering workstation** than a generic fitness application.

---

## Data Formats

OAE is intended to work with common training formats including:

* FIT
* ERG
* ZWO

These formats are normalized into Olympus's internal data model before being processed by the analytics engine.

```text
FIT / ERG / ZWO
       │
       ▼
   Data Parser
       │
       ▼
Olympus Data Model
       │
       ├── Storage
       │
       └── Analytics
```

---

## Analytics

The analytics engine is designed to grow incrementally.

Initial areas include:

### Performance

* Power records
* Heart-rate records
* Pace records
* Power-duration analysis
* Personal best detection
* Historical performance

### Recovery

* HRV tracking
* Resting heart rate tracking
* Baseline calculation
* Historical trends

### Training

* Training load
* Intensity
* Zone distribution
* Session comparison
* Long-term trends

### Anomalies

OAE will track deviations from an athlete's historical baseline.

Potential examples include:

* Unusual heart-rate response
* Power/heart-rate relationship changes
* Unexpected performance changes
* Sensor anomalies
* Unusual training patterns

OAE should prefer transparent, data-backed analysis over opaque scores.

---

## Local-First Architecture

OAE is designed to run entirely on the user's machine.

```text
┌──────────────────────────────┐
│         User Device          │
│                              │
│  Olympus                     │
│      │                       │
│      ▼                       │
│  OAE Backend                 │
│      │                       │
│      ├── Analytics           │
│      ├── SQLite              │
│      └── API                 │
│             │                │
│             ▼                │
│       Local Browser          │
└──────────────────────────────┘
```

The web interface communicates with the local OAE backend through localhost.

OAE does not require a remote server to perform its core functionality.

---

## Development

### Backend

Requirements:

* Rust
* Cargo

Build the workspace:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Run OAE:

```bash
cargo run
```

### Frontend

Requirements:

* Node.js
* npm

```bash
cd web
npm install
npm run dev
```

The Vite development server provides the frontend during development.

---

## Development Roadmap

### Foundation

* [x] DracoLIX integration
* [x] FIT support
* [x] ERG support
* [x] ZWO support
* [ ] Olympus data models
* [ ] Data normalization
* [ ] Data validation
* [ ] SQLite storage

### Analytics Engine

* [ ] Power analytics
* [ ] Power record tracking
* [ ] Heart-rate analytics
* [ ] HRV tracking
* [ ] Resting heart-rate tracking
* [ ] Training load
* [ ] Zone analysis
* [ ] Historical trends
* [ ] Anomaly tracking
* [ ] Power-duration curves

### API

* [ ] REST API
* [ ] WebSocket API
* [ ] OpenAPI specification
* [ ] Live telemetry
* [ ] Activity endpoints
* [ ] Analytics endpoints
* [ ] Athlete endpoints

### Web

* [ ] OAE dashboard
* [ ] Activity viewer
* [ ] Analytics views
* [ ] Performance records
* [ ] Anomaly interface
* [ ] Training history
* [ ] Athlete profile
* [ ] Settings
* [ ] Variable Abstraction Layer

### Olympus Integration

* [ ] `olympus analytics`
* [ ] Automatic OAE startup
* [ ] Browser auto-launch
* [ ] Terminal ↔ OAE integration
* [ ] Live device data
* [ ] Session control

---

## Design Principles

### Local-first

Training data should remain on the user's machine by default.

### Data over decoration

Analytics should provide meaningful information rather than simply generating more metrics.

### Transparent analytics

Users should be able to understand where an analytical result comes from.

### Modular

Analytics, storage, API, and presentation should remain independently replaceable.

### Extensible

OAE should provide a foundation for future Olympus and DragonSuite applications.

### Performance

Large training datasets and time-series calculations should remain responsive even as the athlete's history grows.

---

## Relationship to Olympus

OAE is a separate component of Olympus and is intended to complement Olympus Terminal.

```text
Olympus Terminal
────────────────
Devices
Sessions
Control
CLI
Local interaction


Olympus Analytics Engine
───────────────────────
Data
Storage
Analytics
Visualization
API
Performance history
```

Terminal and OAE may be used independently, while sharing common Olympus data models and infrastructure where appropriate.

---

## Status

**Early development.**

The architecture and APIs are subject to change as the Olympus ecosystem develops.

---

## License

See `LICENSE` for licensing information.

---

## Part of Draconis Engineering

Olympus Analytics Engine is developed as part of the **Draconis Engineering** ecosystem.
