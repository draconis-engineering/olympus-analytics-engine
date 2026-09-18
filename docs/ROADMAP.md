# Olympus Analytics Engine — Roadmap

> Industry-standard progression: TrainingPeaks (Coggan) as spec, Strava/Intervals.icu as UX reference, Draconis principles as constraints (local-first, transparent, `docs/README.md:402`).

This roadmap replaces the initial bucket list (`docs/README.md:342`, `docs/ARCHITECTURE.md:214`). It is ordered so every phase ships a vertical slice: **analytics → storage → API → web**, and each phase is learnable solo.

---

## Principles (non-negotiable)

* **Local-first** — `~/olympus/data/olympus.db` is source of truth (`crates/oae-storage/src/lib.rs:1`, `crates/oae-core/src/config.rs:7`). No cloud required.
* **Data over decoration** — `docs/README.md:407`. Ship `NP/TSS/CTL` correctly before dashboards.
* **Transparent analytics** — show formula + inputs, not opaque scores (`docs/ARCHITECTURE.md:107` anomaly card).
* **Modular** — `oae-analytics` never touches SQLite/HTTP; `oae-api` is thin over `oae-storage` + `oae-analytics`.

---

## Phase 0 — Done (verify)

* [x] DracoLIX integration
* [x] FIT / ERG / ZWO parsing (via `olympus/src/data.rs:62` + `olympus/src/erg.rs:1`)
* [x] Workspace `Cargo.toml:9` — `oae-core`, `oae-data`, `oae-analytics`, `oae-storage`, `oae-api`
* [x] Web shell `web/src/App.tsx:1` + `web/src/main.tsx:1` (Vite + Tailwind, engineering workstation)
* [x] Storage resolver `crates/oae-core/src/config.rs:7` → `~/olympus/data/olympus.db` / `~/Prosjekter/olympus/data/olympus.db` + `cargo run` health check (8 sessions verified `src/main.rs:19`)
* [x] Schema `fit_sessions` / `samples` (`oae-storage::open_db` mirrors `olympus/src/data.rs:160`)

---

## Phase 1 — Foundation: Data Model & Normalization (next)

**Why first:** Industry tools all normalize to a canonical model before any metric (`docs/README.md:102` `oae-data`). Without this, `NP/TSS` drifts.

* [ ] `oae-data`: `Activity { id, sport, started_at, duration, distance, samples[] }`, `Sample { t, power, hr, cadence, speed, lat/lng }`, `Athlete { ftp, weight, max_hr, zones }`
* [ ] Unit normalization (FIT `uint16 speed/1000`, distance `m→km`, `W/bpm` validation)
* [ ] Timestamp handling (FIT epoch → `chrono`, TZ handling, gap detection)
* [ ] Missing-data handling (0 vs null, interpolated vs dropped)
* [ ] Parser adapters `FIT/ERG/ZWO → Activity` (reuse `fitparser` + `olympus/src/erg.rs:1`)
* [ ] `oae-storage` migrations (`user_version` → `sqlx`/`rusqlite` migrations, `crates/oae-storage/src/lib.rs:1`)

**Exit:** `cargo test` with regression FIT files; `SELECT` round-trips through `oae-data` structs.

---

## Phase 2 — Core Analytics: The Coggan Standard (industry baseline)

**Spec:** TrainingPeaks/WKO definition. If this is wrong, nothing else matters.

* [ ] `oae-analytics::power` — `NP` (30s rolling avg ^4), `AP`, `IF = NP/FTP`, `VI = NP/AP`
* [ ] `oae-analytics::load` — `TSS = (sec * NP * IF)/(FTP*3600)*100`, `TRIMP` (HR), `kJ`
* [ ] `oae-analytics::zones` — Coggan 7 power zones, 5 HR zones, pace zones; `time_in_zone` + distribution
* [ ] `oae-analytics::coupling` — `EF = NP/HR`, `Decoupling %` (first/second half) — the `docs/ARCHITECTURE.md:107` card
* [ ] Port reference math from `olympus/src/math.rs:1` (`normalized_power`, `intensity_factor`, `best_rolling_mean`) with unit tests vs known values

**API you will wire in Phase 4:**
```
GET /api/v1/activities/:id/summary → { ap, np, if, vi, tss, kj, decoupling, zone_dist }
```

**Web:** Summary bar on activity detail (no charts yet).

---

## Phase 3 — Records & Power-Duration Curve

**Strava/Intervals.icu parity:** Users expect 5s/1m/5m/20m MMP instantly.

* [ ] Rolling bests `best_rolling_mean` for `5s, 15s, 30s, 1m, 5m, 12m, 20m, 60m` per activity + lifetime
* [ ] Personal-best detection + history (store `records` table, invalidate on new PR)
* [ ] Power-duration curve `MMP (days=90)` + `CP ≈ 20m*0.95` (Morton/Monod), `W'` placeholder
* [ ] `FTP` estimation (`20m*0.95`, ramp test detection)
* [ ] `oae-analytics::records` isolates logic; `oae-storage` persists `records` cache

**API:**
```
GET /api/v1/records?metric=power&durations=5s,1m,5m,20m
GET /api/v1/analytics/power-curve?days=90
GET /api/v1/analytics/ftp-estimate
```

**Web:** `features/records` + power-curve chart (Recharts/D3 `docs/ARCHITECTURE.md:8`).

---

## Phase 4 — Training Load: PMC (the moat)

**TrainingPeaks PMC is the industry's retention engine.** OAE does it locally.

* [ ] Daily `TSS` + `TRIMP` aggregation
* [ ] `CTL (42d exp avg)`, `ATL (7d)`, `TSB = CTL-ATL`, `Ramp Rate`, `ACWR`
* [ ] Weekly buckets + `monotony/strain` (Foster)
* [ ] Metrics cache table (avoid recomputing 8→10k sessions)

**API (your first time-series endpoint):**
```
GET /api/v1/analytics/ctl?days=90  → [{date, tss, ctl, atl, tsb}]
GET /api/v1/analytics/load?from=&to=&group=week
```

**Web:** PMC chart (the “fitness” chart every app copies), calendar `features/analytics`.

---

## Phase 5 — Physiology / Recovery (HRV / RHR)

**Intervals.icu/Garmin parity:** Daily readiness, not a black-box score.

* [ ] `RHR` 7d rolling mean + `±SD` bands
* [ ] `HRV` `rMSSD` ingestion (from FIT `hrv` fields or manual), 7d baseline
* [ ] Trends + deviation flags (e.g., `RHR +7% vs baseline`)
* [ ] `Athlete` profile `data/user/profile.json` mirror + `oae-data::athlete` (`ftp`, `max_hr`, weight)

**API:**
```
GET /api/v1/athlete              → { ftp, weight, max_hr, zones }
PUT /api/v1/athlete
GET /api/v1/analytics/hrv?window=7d
GET /api/v1/analytics/rhr?window=30d
```

**Web:** Athlete page `features/*` with baseline bands (transparent).

---

## Phase 6 — API: REST + OpenAPI (your build — learn here)

**You asked to build this yourself — this is the learning phase.** Follow Strava API conventions (`docs/ARCHITECTURE.md:148`).

* [ ] Stack: `Tokio` + `Axum` + `Serde` (`docs/ARCHITECTURE.md:16`), `rusqlite` (or `sqlx` if you migrate)
* [ ] `GET /api/v1/health` → `{ status, db_path: PathBuf, sessions, samples, candidates }` (wraps `oae-storage::health_check`)
* [ ] `GET /api/v1/activities?limit=&from=&to=` → `StoredSession[]` (paginated)
* [ ] `GET /api/v1/activities/:id` → `StoredSession + samples`
* [ ] `GET /api/v1/activities/:id/summary` → Phase 2 metrics
* [ ] `GET /api/v1/records`, `/analytics/*`, `/athlete/*` (thin handlers → `oae-analytics`/`oae-storage`)
* [ ] OpenAPI spec `openapi.yaml` (generated from `utoipa` or hand-written)
* [ ] Versioning `/api/v1/`, error envelope `{ error, code }`, CORS for Vite `http://localhost:5173`
* [ ] Tests: unit + `axum::TestClient` integration + FIT regression fixtures

**Runtime (`docs/ARCHITECTURE.md:173`):**
* [ ] `olympus analytics` → start Axum on `43100` (fallback random), health endpoint, `web/dist` static serve
* [ ] Graceful shutdown, logging (`env_logger`), config (`oae-core::config`)

**Exit:** `cargo run` prints `Opening http://localhost:43100` and web fetches live sessions (no more `—` placeholders `web/src/App.tsx:1`).

---

## Phase 7 — Web: Analytics Views (consume your API)

* [ ] `TanStack Query` for fetch/cache (`docs/ARCHITECTURE.md:7`)
* [ ] `layouts` + `features/activities` list/detail with traces (power/HR/cadence)
* [ ] `features/analytics` — PMC, power-curve, zone distribution
* [ ] `features/records`, `features/anomalies` (cards), `pages` routing
* [ ] `lib/api` client (`fetch` wrapper, types from `oae-data`), `lib/types` DTOs
* [ ] Connection status dot (API health)

**Exit:** `cd web && npm run dev` shows 8 real sessions from `~/Prosjekter/olympus/data/olympus.db`.

---

## Phase 8 — Anomaly Engine

* [ ] Framework: baseline (rolling mean/SD per metric), z-score, severity `low/med/high`
* [ ] Detectors: `HR/power decoupling`, `VI spike`, `RHR drift`, `sudden PR`, `sensor drop`
* [ ] `anomalies` table + history, `docs/ARCHITECTURE.md:93` card data
* [ ] Notification hook (in-app only, no push)

**API:**
```
GET /api/v1/anomalies?severity=&from=
GET /api/v1/anomalies/:id
```

---

## Phase 9 — Advanced & VAL

* [ ] Critical Power modeling (2/3-param), `FTP` auto-detect vs manual
* [ ] Aerobic decoupling trend, fatigue modeling
* [ ] Variable Abstraction Layer `docs/README.md:388` (sport-agnostic metrics)
* [ ] Benchmark suite (`criterion`), large-history perf (`10k samples` / `1k sessions`)

---

## Phase 10 — Olympus Integration

* [ ] `olympus analytics` bin (`src/main.rs:19` → real server + `open::that("http://localhost:43100")`)
* [ ] `Olympus Terminal ↔ OAE` live telemetry (`WebSocket` `/api/v1/live`)
* [ ] `Bluetooth/live data` `olympus/src/ble.rs:1` → OAE ingestion (optional)
* [ ] Install scripts `olympus/scripts/install.sh:1` add `olympus analytics` entry

---

## How to progress (for you)

1. **Phase 1→2** — pure Rust math + tests (no HTTP). Port `olympus/src/math.rs:1` correctly; write `assert_eq!(tss, 65.2)` tests.
2. **Phase 6** — build `GET /health` then `/activities`. Keep handlers <30 lines; push logic to crates.
3. **Phase 4/7** — hook web to your API; delete mock `—` in `web/src/App.tsx:1`.
4. Never build a metric without a test vector from TrainingPeaks docs or `olympus/data` samples.

**Current branch:** `cargo run` connects to `~/Prosjekter/olympus/data/olympus.db` (8 sessions). Next `git commit` should be Phase 1 `oae-data` structs + normalization tests.

---

## References

* Coggan/Allen *Training and Racing with a Power Meter* (NP/IF/TSS/CTL)
* Strava API `https://developers.strava.com/docs/reference/`, Intervals.icu `https://intervals.icu/api/docs`
* `docs/README.md:224` analytics, `docs/ARCHITECTURE.md:61` engine checklist
