
## FRONTEND

- [ ] React + TypeScript
- [ ] Vite
- [ ] Bklit UI
- [ ] TanStack Query
- [ ] D3 / Recharts

## API

- [ ] REST + WebSocket
- [ ] OpenAPI

## BACKEND

- [ ] Rust
- [ ] Tokio
- [ ] Axum
- [ ] Serde
- [ ] SQLx
- [ ] SQLite

## DATA / CORE

- [x] DracoLIX
- [ ] Olympus data models
- [x] FIT / ERG / ZWO
- [ ] Analytics Engine
  - [ ] Anomaly tracking
  - [ ] HRV/RHR tracking
  - [ ] Power record tracking

## 🧠 OAE — DATA / CORE

### Data foundation
- [x] DracoLIX
- [ ] Olympus data models
- [x] FIT / ERG / ZWO
- [ ] Analytics Engine
  - [ ] Anomaly tracking
  - [ ] HRV/RHR tracking
  - [ ] Power record tracking

### Data normalization
- [ ] Unit normalization
- [ ] Timestamp handling
- [ ] Missing-data handling
- [ ] Data validation

### Storage
- [ ] Local database
- [ ] SQLite er et veldig naturlig valg
- [ ] Activity storage
- [ ] Athlete profile/history
- [ ] Workout storage
- [ ] Metrics cache
- [ ] Import/export
- [ ] Database migrations

### Analytics Engine

- [ ] Core analytics
  - [ ] Power analysis
  - [ ] Heart-rate analysis
  - [ ] Pace analysis
  - [ ] Cadence analysis
  - [ ] Training load
  - [ ] Intensity analysis
  - [ ] Zone distribution
  - [ ] Workout comparison
  - [ ] Historical trends
- [ ] Performance records
  - [ ] Power record tracking
  - [ ] HR record tracking
  - [ ] Pace record tracking
  - [ ] Duration-based records
  - [ ] 5 sec
  - [ ] 1 min
  - [ ] 5 min
  - [ ] 20 min
  - [ ] etc.
  - [ ] Personal-best detection
  - [ ] Record history
- [ ] Physiology / recovery
  - [ ] HRV tracking
  - [ ] Resting HR tracking
  - [ ] HRV trends
  - [ ] RHR trends
  - [ ] Recovery indicators
  - [ ] Baseline calculation

### 🚨 Anomaly Engine

- [ ] Anomaly detection framework
- [ ] Baseline generation
- [ ] Outlier detection
- [ ] Performance anomalies
- [ ] HR/power relationship anomalies
- [ ] Cadence anomalies
- [ ] Unusual fatigue patterns
- [ ] Sensor/data anomalies
- [ ] Anomaly severity
- [ ] Historical anomaly tracking

Example:

OLYMPUS / ANOMALY

┌─────────────────────────────────────┐
│ HR / POWER DECOUPLING               │
│                                     │
│ Current session       +8.4%         │
│ Personal baseline     +3.1%         │
│                                     │
│ ↑ Significant deviation             │
└─────────────────────────────────────┘

### 🔬 Advanced Analytics

- [ ] Power-duration curve
- [ ] Critical power estimation
- [ ] FTP estimation
- [ ] HR-power relationship
- [ ] Aerobic decoupling
- [ ] Fatigue trends
- [ ] Training monotony
- [ ] Training strain
- [ ] Acute/chronic load
- [ ] Fitness/fatigue modelling
- [ ] Performance trend modelling

### 🔌 OAE API

- [ ] REST API
- [ ] WebSocket API
- [ ] API versioning
- [ ] OpenAPI specification
- [ ] Activity endpoints
- [ ] Analytics endpoints
- [ ] Athlete endpoints
- [ ] Workout endpoints
- [ ] Device endpoints
- [ ] Live telemetry endpoints

Example:

/api/v1/
    activities
    athletes
    workouts
    metrics
    records
    anomalies
    analytics
    devices
    
### 🌐 Web application

- [ ] React
- [ ] TypeScript
- [ ] Vite
- [ ] Bklit UI
- [ ] Dashboard
- [ ] Activity viewer
- [ ] Analytics viewer
- [ ] Power charts
- [ ] HR charts
- [ ] Training calendar
- [ ] Records
- [ ] Anomalies

### ⚙️ Runtime

For olympus analytics-opplevelsen:

- [ ] Local HTTP server
- [ ] Automatic port selection
- [ ] Browser auto-launch
- [ ] Graceful shutdown
- [ ] Configuration
- [ ] Logging
- [ ] Error handling
- [ ] Health endpoint
- [ ] Frontend ↔ backend connection status

### Example run

```sh
$ olympus analytics

Starting Olympus Analytics Engine...

✓ DracoLIX
✓ Database
✓ Analytics
✓ API
✓ Web interface

Opening http://localhost:43100
```

### Tests
- [ ] Unit tests
- [ ] Analytics correctness tests
- [ ] Parser tests
- [ ] Data normalization tests
- [ ] Regression datasets
- [ ] API tests
- [ ] Frontend tests
- [ ] End-to-end tests
- [ ] Benchmark suite

### Phase 1 — Foundation
- [x] DracoLIX
- [x] FIT / ERG / ZWO
- [ ] Olympus data models
- [ ] Normalization
- [ ] SQLite storage
### Phase 2 — Analytics
- [ ] Power analytics
- [ ] HR analytics
- [ ] Pace analytics
- [ ] Training load
- [ ] Power records
- [ ] HRV/RHR
- [ ] Anomaly framework
### Phase 3 — API
- [ ] REST
- [ ] WebSocket
- [ ] OpenAPI
- [ ] Live telemetry
### Phase 4 — Interface
- [ ] React
- [ ] TypeScript
- [ ] Vite
- [ ] Bklit UI
- [ ] Dashboard
- [ ] Activity viewer
- [ ] Analytics views
- [ ] Records
- [ ] Anomalies
### Phase 5 — Olympus integration
- [ ] `olympus analytics`
- [ ] Start OAE
- [ ] Open browser
- [ ] Terminal ↔ OAE
- [ ] Bluetooth/live data
- [ ] Session control
### Phase 6 — Advanced
- [ ] VAL - Variable Abstraction Layer
- [ ] Power-duration curve
- [ ] CP/FTP modelling
- [ ] Advanced anomaly detection
- [ ] Advanced training models
- [ ] More sport-specific analytics


## Separation

             OLYMPUS
                 │
       ┌─────────┴─────────┐
       │                   │
    TERMINAL              OAE
       │                   │
   Control            Computation
   Devices            Analytics
   Sessions           Storage
                     API
                      │
                      ▼
                 Web Interface
