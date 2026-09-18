import { useState } from "react";

type NavId = "dashboard" | "activities" | "analytics" | "records" | "anomalies" | "athlete";

const NAV: { id: NavId; label: string; desc: string }[] = [
  { id: "dashboard", label: "DASHBOARD", desc: "Overview & status" },
  { id: "activities", label: "ACTIVITIES", desc: "Session browser" },
  { id: "analytics", label: "ANALYTICS", desc: "Power / HR / Pace" },
  { id: "records", label: "RECORDS", desc: "Personal bests" },
  { id: "anomalies", label: "ANOMALIES", desc: "Deviation tracking" },
  { id: "athlete", label: "ATHLETE", desc: "Profile & history" },
];

function StatusDot({ ok }: { ok: boolean }) {
  return (
    <span
      className={`inline-block h-2 w-2 rounded-full ${ok ? "bg-emerald-500" : "bg-zinc-600"} shadow-[0_0_8px_rgba(16,185,129,0.5)]`}
    />
  );
}

export default function App() {
  const [active, setActive] = useState<NavId>("dashboard");

  return (
    <div className="min-h-screen bg-[#0a0a0b] text-zinc-100 flex flex-col font-sans">
      {/* Top bar — engineering workstation */}
      <header className="h-14 border-b border-[#232326] bg-[#0f0f10] flex items-center justify-between px-4 sticky top-0 z-10">
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-3">
            <div className="h-7 w-7 rounded bg-zinc-100 text-black grid place-items-center font-mono text-xs font-bold tracking-widest">
              Ω
            </div>
            <div>
              <div className="font-mono text-sm font-semibold tracking-[0.14em] leading-none">
                OLYMPUS
              </div>
              <div className="font-mono text-[10px] tracking-[0.22em] text-zinc-500 leading-none">
                ANALYTICS ENGINE
              </div>
            </div>
            <span className="ml-2 hidden sm:inline-flex items-center gap-1.5 rounded border border-[#232326] bg-[#141416] px-2 py-1 font-mono text-[10px] tracking-widest text-zinc-400">
              <span className="h-1.5 w-1.5 rounded-full bg-amber-500 animate-pulse" />
              LOCAL-FIRST • v0.1.0
            </span>
          </div>
        </div>

        <div className="hidden lg:flex items-center gap-3 font-mono text-[11px] tracking-wide">
          <span className="flex items-center gap-1.5 text-zinc-400">
            <StatusDot ok /> DracoLIX
          </span>
          <span className="text-zinc-700">/</span>
          <span className="flex items-center gap-1.5 text-zinc-400">
            <StatusDot ok /> SQLite
          </span>
          <span className="text-zinc-700">/</span>
          <span className="flex items-center gap-1.5 text-zinc-500">
            <StatusDot ok={false} /> API
          </span>
          <span className="text-zinc-700">/</span>
          <span className="flex items-center gap-1.5 text-zinc-400">
            <StatusDot ok /> WEB
          </span>
        </div>

        <div className="flex items-center gap-2">
          <span className="hidden sm:inline font-mono text-[10px] tracking-widest text-zinc-500 border border-[#232326] rounded px-2 py-1 bg-[#141416]">
            localhost:5173
          </span>
          <div className="h-8 w-8 rounded-full bg-[#1a1a1e] border border-[#232326] grid place-items-center text-[11px] font-mono text-zinc-400">
            SA
          </div>
        </div>
      </header>

      <div className="flex flex-1 min-h-0">
        {/* Sidebar */}
        <aside className="hidden md:flex w-[240px] shrink-0 border-r border-[#232326] bg-[#0a0a0b] flex-col">
          <div className="p-3">
            <div className="font-mono text-[10px] tracking-[0.2em] text-zinc-500 px-2 py-2">
              NAVIGATION
            </div>
            <nav className="space-y-1">
              {NAV.map((item) => (
                <button
                  key={item.id}
                  onClick={() => setActive(item.id)}
                  className={`w-full text-left rounded border px-3 py-2.5 flex flex-col gap-0.5 transition ${
                    active === item.id
                      ? "bg-[#141416] border-zinc-700 text-zinc-100"
                      : "bg-transparent border-transparent text-zinc-500 hover:text-zinc-300 hover:bg-[#141416]/60 hover:border-[#232326]"
                  }`}
                >
                  <span className="font-mono text-xs tracking-[0.14em]">{item.label}</span>
                  <span className="font-mono text-[10px] tracking-wide opacity-60">{item.desc}</span>
                </button>
              ))}
            </nav>
          </div>

          <div className="mt-auto p-3 border-t border-[#232326]">
            <div className="rounded border border-[#232326] bg-[#141416] p-3">
              <div className="font-mono text-[10px] tracking-[0.18em] text-zinc-500">SYSTEM</div>
              <div className="mt-2 space-y-1.5 font-mono text-[11px] leading-none">
                <div className="flex justify-between">
                  <span className="text-zinc-500">Storage</span>
                  <span className="text-zinc-300">SQLite</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-zinc-500">Mode</span>
                  <span className="text-emerald-400">LOCAL</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-zinc-500">Parsers</span>
                  <span className="text-zinc-300">FIT / ERG / ZWO</span>
                </div>
              </div>
              <div className="mt-3 font-mono text-[10px] leading-relaxed text-zinc-500">
                Your data stays local. No cloud sync required.
              </div>
            </div>
            <div className="mt-3 font-mono text-[10px] tracking-wide text-zinc-600 text-center">
              Draconis Engineering © 2026
            </div>
          </div>
        </aside>

        {/* Main */}
        <main className="flex-1 min-w-0 bg-[#0a0a0b]">
          {/* Mobile nav */}
          <div className="md:hidden flex gap-1.5 p-3 border-b border-[#232326] overflow-x-auto">
            {NAV.map((n) => (
              <button
                key={n.id}
                onClick={() => setActive(n.id)}
                className={`shrink-0 rounded border px-3 py-1.5 font-mono text-xs tracking-widest ${
                  active === n.id
                    ? "bg-zinc-100 text-black border-zinc-100"
                    : "bg-[#141416] text-zinc-400 border-[#232326]"
                }`}
              >
                {n.label}
              </button>
            ))}
          </div>

          <div className="p-4 sm:p-6 max-w-[1280px] mx-auto">
            {active === "dashboard" && <Dashboard />}
            {active === "activities" && <Placeholder title="Activities" hint="FIT/ERG/ZWO ingestion + session browser — next phase." />}
            {active === "analytics" && <Placeholder title="Analytics" hint="Power • HR • Pace • Training load • Zone distribution." />}
            {active === "records" && <Placeholder title="Records" hint="5s / 1m / 5m / 20m power, HR and pace personal bests." />}
            {active === "anomalies" && <AnomaliesPreview />}
            {active === "athlete" && <Placeholder title="Athlete" hint="Profile, history, HRV/RHR baselines." />}
          </div>
        </main>
      </div>

      <footer className="border-t border-[#232326] bg-[#0f0f10] px-4 py-2 flex items-center justify-between font-mono text-[10px] tracking-widest text-zinc-600">
        <span>OLYMPUS ANALYTICS ENGINE — PERFORMANCE ENGINEERING WORKSTATION</span>
        <span className="hidden sm:inline">cargo run → http://localhost:43100 (planned) • web:5173 (dev)</span>
      </footer>
    </div>
  );
}

function Dashboard() {
  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-baseline justify-between gap-3">
        <h1 className="font-mono text-sm tracking-[0.18em] text-zinc-200">DASHBOARD — OVERVIEW</h1>
        <span className="font-mono text-[11px] tracking-wide text-zinc-500">
          Early development • APIs subject to change
        </span>
      </div>

      {/* KPI grid */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <Kpi label="Activities" value="—" sub="SQLite source of truth" />
        <Kpi label="Avg Power" value="— W" sub="Last 30d • normalized" />
        <Kpi label="Avg HR" value="— bpm" sub="Baseline pending" />
        <Kpi label="Training Load" value="—" sub="TSS placeholder" />
      </div>

      {/* Middle row */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-3">
        <div className="lg:col-span-2 rounded border border-[#232326] bg-[#141416] p-4">
          <div className="flex items-center justify-between">
            <h2 className="font-mono text-xs tracking-[0.16em] text-zinc-300">POWER-DURATION — PLACEHOLDER</h2>
            <span className="font-mono text-[10px] tracking-widest text-zinc-500 border border-[#232326] rounded px-2 py-0.5">
              FUTURE: CP / FTP MODEL
            </span>
          </div>
          <div className="mt-4 h-[180px] rounded border border-dashed border-zinc-700/50 bg-[#0a0a0b] grid place-items-center">
            <div className="text-center">
              <div className="font-mono text-xs tracking-widest text-zinc-500">NO DATA YET</div>
              <div className="font-mono text-[11px] text-zinc-600 mt-1">
                Import FIT / ERG / ZWO → Olympus Data Model → Storage → Analytics
              </div>
            </div>
          </div>
          <div className="mt-3 flex gap-2 font-mono text-[10px] tracking-wide">
            <span className="rounded bg-zinc-800 px-2 py-1 text-zinc-400">5s</span>
            <span className="rounded bg-zinc-800 px-2 py-1 text-zinc-400">1m</span>
            <span className="rounded bg-zinc-800 px-2 py-1 text-zinc-400">5m</span>
            <span className="rounded bg-zinc-800 px-2 py-1 text-zinc-400">20m</span>
            <span className="rounded border border-zinc-700 px-2 py-1 text-zinc-500">FTP est.</span>
          </div>
        </div>

        <div className="rounded border border-[#232326] bg-[#141416] p-4">
          <h2 className="font-mono text-xs tracking-[0.16em] text-zinc-300">RECENT SESSIONS</h2>
          <div className="mt-4 space-y-2">
            {[1, 2, 3].map((i) => (
              <div key={i} className="rounded border border-[#232326] bg-[#0a0a0b] p-3 flex justify-between items-center">
                <div>
                  <div className="font-mono text-xs tracking-wide text-zinc-300">— No session —</div>
                  <div className="font-mono text-[11px] text-zinc-600">Waiting for first import</div>
                </div>
                <span className="font-mono text-[10px] tracking-widest text-zinc-600 border border-[#232326] rounded px-2 py-1">
                  00:00
                </span>
              </div>
            ))}
          </div>
          <button className="mt-4 w-full rounded border border-zinc-700 bg-zinc-100 text-black font-mono text-xs tracking-[0.14em] py-2 hover:bg-white transition">
            IMPORT ACTIVITY
          </button>
          <p className="mt-2 font-mono text-[10px] leading-relaxed text-zinc-600 text-center">
            Drop a .fit file here (planned) or use Olympus Terminal → OAE
          </p>
        </div>
      </div>

      {/* Bottom row — docs alignment */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-3">
        <div className="rounded border border-[#232326] bg-[#0f0f10] p-4">
          <h3 className="font-mono text-[11px] tracking-[0.18em] text-zinc-400">ARCHITECTURE</h3>
          <pre className="mt-3 font-mono text-[11px] leading-relaxed text-zinc-500 overflow-x-auto">
{`FIT / ERG / ZWO
     ↓
 Data Parser → Olympus Data Model
                 ├─→ Storage (SQLite)
                 └─→ Analytics → API → React + Vite
                                          ↓
                                       Browser`}
          </pre>
        </div>
        <div className="rounded border border-[#232326] bg-[#0f0f10] p-4">
          <h3 className="font-mono text-[11px] tracking-[0.18em] text-zinc-400">NEXT STEPS (per ROADMAP)</h3>
          <ul className="mt-3 space-y-1.5 font-mono text-xs text-zinc-500 list-disc list-inside">
            <li>Olympus data models + normalization</li>
            <li>SQLite storage + migrations</li>
            <li>REST / WebSocket API + OpenAPI</li>
            <li>Power / HR / HRV analytics + anomaly engine</li>
          </ul>
          <div className="mt-3 font-mono text-[11px] text-zinc-600">
            Run: <span className="text-zinc-300">cargo run</span> + <span className="text-zinc-300">cd web && npm run dev</span>
          </div>
        </div>
      </div>
    </div>
  );
}

function Kpi({ label, value, sub }: { label: string; value: string; sub: string }) {
  return (
    <div className="rounded border border-[#232326] bg-[#141416] p-4">
      <div className="font-mono text-[10px] tracking-[0.2em] text-zinc-500">{label}</div>
      <div className="mt-2 font-mono text-xl tracking-wide text-zinc-100">{value}</div>
      <div className="font-mono text-[11px] text-zinc-600">{sub}</div>
    </div>
  );
}

function Placeholder({ title, hint }: { title: string; hint: string }) {
  return (
    <div className="rounded border border-dashed border-zinc-700/50 bg-[#141416]/50 p-8 text-center">
      <div className="font-mono text-sm tracking-[0.18em] text-zinc-300">{title.toUpperCase()}</div>
      <div className="font-mono text-xs text-zinc-500 mt-2 max-w-[560px] mx-auto">{hint}</div>
      <div className="mt-6 inline-flex items-center gap-2 rounded border border-[#232326] bg-[#0a0a0b] px-3 py-2 font-mono text-[11px] tracking-wide text-zinc-500">
        <span className="h-2 w-2 rounded-full bg-amber-500/70" />
        Scaffolded for OAE — implementation pending
      </div>
    </div>
  );
}

function AnomaliesPreview() {
  return (
    <div className="space-y-4">
      <h2 className="font-mono text-sm tracking-[0.18em] text-zinc-200">ANOMALIES — PREVIEW</h2>
      <div className="rounded border border-[#232326] bg-[#141416] p-4">
        <div className="font-mono text-xs tracking-[0.14em] text-zinc-300">HR / POWER DECOUPLING</div>
        <div className="mt-3 grid grid-cols-3 gap-3 font-mono text-xs">
          <div className="rounded bg-[#0a0a0b] border border-[#232326] p-3">
            <div className="text-zinc-500 text-[11px] tracking-wide">Current session</div>
            <div className="text-amber-400 mt-1">+8.4%</div>
          </div>
          <div className="rounded bg-[#0a0a0b] border border-[#232326] p-3">
            <div className="text-zinc-500 text-[11px] tracking-wide">Personal baseline</div>
            <div className="text-zinc-300 mt-1">+3.1%</div>
          </div>
          <div className="rounded bg-[#0a0a0b] border border-amber-900/50 p-3">
            <div className="text-zinc-500 text-[11px] tracking-wide">Deviation</div>
            <div className="text-amber-400 mt-1">↑ Significant</div>
          </div>
        </div>
        <p className="mt-3 font-mono text-[11px] leading-relaxed text-zinc-600">
          Example from <span className="text-zinc-400">docs/ARCHITECTURE.md</span> — transparent, data-backed anomaly cards. Engine will compare live session to historical baseline.
        </p>
      </div>
      <Placeholder title="Anomaly Engine" hint="Baseline generation • Outlier detection • HR/power decoupling • Sensor anomalies — see ARCHITECTURE.md Anomaly Engine." />
    </div>
  );
}
