import { useEffect, useState } from "react";
import { BarChart3, Settings2 } from "lucide-react";
import { CartesianGrid, Line, LineChart, ResponsiveContainer, Tooltip, XAxis, YAxis, Bar, BarChart, Cell } from "recharts";
import { api } from "../lib/api";
import type { Cs2ssOverviewResponse, Cs2ssPlayerDetailResponse, Cs2ssDmOverview, Cs2ssMatchWithStats } from "../data/cs2ssTypes";
import { cs2ssCalcRating, cs2ssCalcAdr, cs2ssCalcKast } from "../data/cs2ssRating";
import { cs2ssMapLabel } from "../data/cs2ssMaps";
import StatsMatchHistory from "./StatsMatchHistory";
import StatsMatchDetail from "./StatsMatchDetail";
import { useStore } from "../state/store";
import { useT } from "../i18n";
import "./StatsPanel.css";

type SubView = "dashboard" | "history" | "matchDetail";
const HI = "#20b486", MID = "#e67e22", LO = "#e05d75";
function rcol(r: number) { return r >= 1.1 ? HI : r >= 0.9 ? MID : LO; }
function fmtD(iso: string) { try { const d = new Date(iso); return `${d.getMonth() + 1}/${d.getDate()} ${String(d.getHours()).padStart(2, "0")}:${String(d.getMinutes()).padStart(2, "0")}`; } catch { return iso; } }
function validSteamId64(value: string) { return /^7656119\d{10}$/.test(value.trim()); }

export default function StatsDashboard() {
  const { reportError, directory } = useStore();
  const t = useT();
  const csgo = directory?.valid ? directory.selected ?? "" : "";
  const [sub, setSub] = useState<SubView>("dashboard");
  const [selMatch, setSelMatch] = useState(0);
  const [data, setData] = useState<Cs2ssOverviewResponse | null>(null);
  const [matches, setMatches] = useState<Cs2ssMatchWithStats[]>([]);
  const [loading, setLoading] = useState(true);
  const [err, setErr] = useState(false);
  const [mode, setMode] = useState<"competitive" | "deathmatch">("competitive");
  const [pid, setPid] = useState("");
  const [pd, setPd] = useState<Cs2ssPlayerDetailResponse | null>(null);
  const [dm, setDm] = useState<Cs2ssDmOverview | null>(null);
  const [cfgOpen, setCfgOpen] = useState(false);
  const [cfgInput, setCfgInput] = useState("");
  const [cfgSaving, setCfgSaving] = useState(false);
  const comp = matches.filter(m => m.modeFamily === "competitive");
  const dms = matches.filter(m => m.modeFamily === "deathmatch");
  const cfgValid = validSteamId64(cfgInput);

  useEffect(() => {
    if (!csgo) return;
    let c = false;
    (async () => {
      try {
        const cfgData = await api.getCs2ssConfig(csgo);
        if (c) return;
        if (!cfgData.steamId) { setCfgInput(""); setCfgOpen(true); setLoading(false); return; }
        setPid(cfgData.steamId);
        try {
const [o, ms] = await Promise.all([api.getCs2ssOverview(csgo), api.listCs2ssMatchesWithStats(csgo)]);
          if (c) return;
          setData(o); setMatches(ms);
        } catch {
          if (c) return;
          setData({ matchCount: 0, players: [] });
          setMatches([]);
        }
      } catch (e) {
        if (!c) setErr(true);
        reportError(e);
      } finally { if (!c) setLoading(false); }
    })();
    return () => { c = true; };
  }, [csgo, reportError]);

  useEffect(() => {
    if (!pid) return;
    api.getCs2ssPlayerDetail(csgo, pid).then(setPd).catch(() => {});
  }, [pid, csgo]);

  useEffect(() => {
    if (!pid || !csgo) return;
    api.getCs2ssDmOverview(csgo, pid).then(setDm).catch(() => setDm(null));
  }, [pid, csgo]);

  useEffect(() => {
    if (comp.length === 0 && (dms.length > 0 || (dm?.sessionCount ?? 0) > 0)) {
      setMode("deathmatch");
    }
  }, [comp.length, dm?.sessionCount, dms.length]);

  const saveCfg = async () => {
    if (!cfgValid || !csgo) return;
    setCfgSaving(true);
    try {
      await api.saveCs2ssConfig(csgo, { steamId: cfgInput.trim() });
      setPid(cfgInput.trim());
      setCfgOpen(false);
      try {
        const [o, ms] = await Promise.all([api.getCs2ssOverview(csgo), api.listCs2ssMatchesWithStats(csgo)]);
        setData(o); setMatches(ms);
      } catch {
        setData({ matchCount: 0, players: [] });
        setMatches([]);
      }
    } catch (e) { reportError(e); }
    finally { setCfgSaving(false); }
  };

  const openConfig = () => {
    setCfgInput(pid);
    setCfgOpen(true);
  };

  if (cfgOpen) return (
    <div className="stats-panel stats-config">
      <div className="stats-config__card">
        <div className="stats-config__title"><Settings2 size={22} />{t("stats.configTitle")}</div>
        <p>{t("stats.configDescription")}</p>
        <code>https://steamcommunity.com/profiles/<b>XXXXXX</b>/</code>
        <input
          value={cfgInput}
          onChange={e => setCfgInput(e.target.value)}
          placeholder="7656119XXXXXXXXXX"
          autoFocus
          aria-invalid={cfgInput.length > 0 && !cfgValid}
          onKeyDown={e => { if (e.key === "Enter" && cfgValid) void saveCfg(); }}
        />
        {cfgInput.length > 0 && !cfgValid && <span className="stats-config__error">{t("stats.steamIdInvalid")}</span>}
        <div className="stats-config__actions">
          {pid && <button type="button" onClick={() => setCfgOpen(false)}>{t("stats.cancel")}</button>}
          <button type="button" className="primary" disabled={!cfgValid || cfgSaving} onClick={() => void saveCfg()}>
            {cfgSaving ? t("stats.saving") : t("stats.save")}
          </button>
        </div>
      </div>
    </div>
  );

  const po = data?.players.find(p => p.steamId === pid) ?? null;
  const tr = po?.totalRounds ?? 0;
  const tk = po?.kills ?? 0, td = po?.deaths ?? 0, ta = po?.assists ?? 0, tdm = po?.damage ?? 0, ths = po?.headshots ?? 0;
  const rating = (pd?.matches ?? []).length > 0
    ? (pd?.matches ?? []).reduce((s, m) => s + (m.roundsPlayed > 0
        ? cs2ssCalcRating(m.totalKills, m.totalDeaths, m.totalAssists, m.totalDamage, m.totalHeadshotKills, m.roundsPlayed, { kastRounds: m.kastRounds, tradeKills: m.tradeKills, multikill2: m.multikill2, multikill3: m.multikill3, multikill4: m.multikill4, multikill5: m.multikill5, clutchAttempts: m.clutchAttempts, clutchesWon: m.clutchesWon })
        : 0), 0) / (pd?.matches ?? []).length
    : 0;
  const adr = cs2ssCalcAdr(tdm, tr);
  const kast = cs2ssCalcKast(po?.kastRounds ?? 0, tr);

  const trend = (pd?.matches ?? []).filter(m => m.roundsPlayed > 0).slice(0, 20).reverse().map((m, i) => ({ i: i + 1, r: cs2ssCalcRating(m.totalKills, m.totalDeaths, m.totalAssists, m.totalDamage, m.totalHeadshotKills, m.roundsPlayed, { kastRounds: m.kastRounds, tradeKills: m.tradeKills, multikill2: m.multikill2, multikill3: m.multikill3, multikill4: m.multikill4, multikill5: m.multikill5, clutchAttempts: m.clutchAttempts, clutchesWon: m.clutchesWon }) }));
  const mpperf = (pd?.mapStats ?? []).filter(m => m.rounds > 0).map(m => ({ map: cs2ssMapLabel(m.map), r: cs2ssCalcRating(m.kills, m.deaths, m.assists, m.damage, m.headshots, m.rounds, { kastRounds: m.kastRounds, tradeKills: m.tradeKills, multikill2: m.multikill2, multikill3: m.multikill3, multikill4: m.multikill4, multikill5: m.multikill5, clutchAttempts: m.clutchAttempts, clutchesWon: m.clutchesWon }) })).sort((a, b) => b.r - a.r);

  const recent = (pd?.matches ?? []).slice(0, 10).map(m => {
    const r = m.roundsPlayed > 0 ? cs2ssCalcRating(m.totalKills, m.totalDeaths, m.totalAssists, m.totalDamage, m.totalHeadshotKills, m.roundsPlayed, { kastRounds: m.kastRounds, tradeKills: m.tradeKills, multikill2: m.multikill2, multikill3: m.multikill3, multikill4: m.multikill4, multikill5: m.multikill5, clutchAttempts: m.clutchAttempts, clutchesWon: m.clutchesWon }) : 0;
    return { ...m, r };
  });
  const highScoreSession = (dm?.sessions ?? []).reduce<Cs2ssDmOverview["sessions"][number] | null>(
    (best, session) => !best || session.score > best.score ? session : best,
    null,
  );
  const hasCompetitive = data ? data.matchCount > 0 || data.players.length > 0 || comp.length > 0 : false;
  const hasDeathmatch = dms.length > 0 || (dm?.sessionCount ?? 0) > 0;

  if (sub === "matchDetail" && selMatch > 0) return <StatsMatchDetail csgo={csgo} matchId={selMatch} onBack={() => setSub("dashboard")} />;
  if (sub === "history") return <StatsMatchHistory csgo={csgo} onOpenMatch={id => { setSelMatch(id); setSub("matchDetail"); }} onBack={() => setSub("dashboard")} />;
  if (loading) return <div className="stats-panel"><div className="stats-panel__loading">{t("stats.loading")}</div></div>;
  if (err && !data) return <div className="stats-panel"><div className="stats-panel__empty">{t("stats.connectionError")}</div></div>;
  if (!data) return <div className="stats-panel"><div className="stats-panel__loading">{t("stats.loading")}</div></div>;

  if (!hasCompetitive && !hasDeathmatch) return (
    <div className="stats-panel">
      <div className="stats-board-header">
        <h1>{t("stats.globalHistory")}</h1>
        <div className="stats-mode-switch">
          <button className="active" onClick={() => setMode("competitive")}>{t("stats.competitive")}<small>0</small></button>
          <button onClick={() => setMode("deathmatch")}>{t("stats.deathmatch")}<small>0</small></button>
          <button className="stats-mode-switch__settings" onClick={openConfig} title={t("stats.editSteamId")} aria-label={t("stats.editSteamId")}><Settings2 size={16} /></button>
        </div>
      </div>
      <div className="stats-empty-board">
        <BarChart3 size={28} aria-hidden="true" />
        <strong>{t("stats.noData")}</strong>
        <span>{t("stats.emptyAll")}</span>
      </div>
    </div>
  );

  return (
    <div className="stats-panel">
      <div className="stats-board-header">
        <h1>{t("stats.globalHistory")}</h1>
        <div className="stats-mode-switch">
          <button className={mode === "competitive" ? "active" : ""} onClick={() => setMode("competitive")}>{t("stats.competitive")}<small>{comp.length}</small></button>
          <button className={mode === "deathmatch" ? "active" : ""} onClick={() => setMode("deathmatch")}>{t("stats.deathmatch")}<small>{dms.length}</small></button>
          <button className="stats-mode-switch__all" onClick={() => setSub("history")}>{t("stats.allMatches")} →</button>
          <button className="stats-mode-switch__settings" onClick={openConfig} title={t("stats.editSteamId")} aria-label={t("stats.editSteamId")}><Settings2 size={16} /></button>
        </div>
      </div>

      {mode === "competitive" && pid && hasCompetitive ? (<>
        <div className="stats-hero">
          <div><span className="stats-hero__eyebrow">{t("stats.playerDossier")}</span><h1>{po?.name ?? pid}</h1></div>
          <div className="stats-hero__rating"><small>{t("stats.offlineRating")}</small><strong style={{ color: rcol(rating) }}>{rating.toFixed(2)}</strong></div>
        </div>
        <div className="stats-cards">
          {[[t("stats.matches"), po?.matches ?? 0], ["KAST", `${kast}%`, kast >= 75 ? HI : kast >= 65 ? MID : LO], ["ADR", adr.toFixed(1), adr >= 85 ? HI : adr >= 70 ? MID : LO], ["K/D", (tk / Math.max(1, td)).toFixed(2), (tk / Math.max(1, td)) >= 1.2 ? HI : (tk / Math.max(1, td)) >= 1 ? MID : LO], ["KDA", ((tk + ta) / Math.max(1, td)).toFixed(2)], ["KPR", (tr > 0 ? (tk / tr).toFixed(2) : "0.00")], ["HS%", `${(tk > 0 ? Math.round(ths / tk * 100) : 0)}%`], [t("stats.clutches"), `${po?.clutchesWon ?? 0}/${po?.clutchAttempts ?? 0}`]].map(([l, v, c]) => (
            <div className="stats-card" key={l}><span className="stats-card__label">{l}</span><span className="stats-card__value" style={c ? { color: c } as React.CSSProperties : undefined}>{v}</span></div>
          ))}
        </div>
        <div className="stats-impact">
          {[[t("stats.tradeKills"), po?.tradeKills ?? 0], [t("stats.twoKillRounds"), po?.multikill2 ?? 0], [t("stats.threeKillRounds"), po?.multikill3 ?? 0], [t("stats.fourKillRounds"), po?.multikill4 ?? 0], [t("stats.ace"), po?.multikill5 ?? 0]].map(([l, v]) => (
            <div key={l}><span>{l}</span><b>{String(v)}</b></div>
          ))}
        </div>
        <div className="stats-charts">
          <div className="stats-panel-block">
            <div className="stats-panel-block__title"><div><span>{t("stats.trend")}</span><h2>{t("stats.ratingTrend")}</h2></div></div>
            {trend.length > 0 ? <ResponsiveContainer width="100%" height={200}><LineChart data={trend}><CartesianGrid strokeDasharray="3 3" /><XAxis dataKey="i" tick={{ fontSize: 11 }} /><YAxis domain={[0, "auto"]} tick={{ fontSize: 11 }} /><Tooltip /><Line type="monotone" dataKey="r" stroke="#8e5cb8" strokeWidth={2.2} dot={{ r: 3, fill: "#8e5cb8" }} /></LineChart></ResponsiveContainer> : <p style={{ color: "var(--text-secondary)", textAlign: "center", padding: 32 }}>{t("stats.insufficientData")}</p>}
          </div>
          <div className="stats-panel-block">
            <div className="stats-panel-block__title"><div><span>{t("stats.maps")}</span><h2>{t("stats.mapPerformance")}</h2></div></div>
            {mpperf.length > 0 ? <ResponsiveContainer width="100%" height={200}><BarChart data={mpperf} layout="vertical"><CartesianGrid strokeDasharray="3 3" /><XAxis type="number" domain={[0, "auto"]} tick={{ fontSize: 11 }} /><YAxis type="category" dataKey="map" tick={{ fontSize: 11 }} width={72} /><Tooltip /><Bar dataKey="r" radius={[0, 4, 4, 0]}>{mpperf.map((_entry, i) => <Cell key={i} fill={["#5d9cec","#3498db","#2ecc71","#f39c12","#9b59b6"][i % 5]} />)}</Bar></BarChart></ResponsiveContainer> : <p style={{ color: "var(--text-secondary)", textAlign: "center", padding: 32 }}>{t("stats.noMapData")}</p>}
          </div>
        </div>
        <div className="stats-panel-block">
          <div className="stats-panel-block__title"><div><span>{t("stats.recent")}</span><h2>{t("stats.recentMatches")}</h2></div></div>
          {recent.length > 0 ? (
<table className="stats-table"><thead><tr><th>{t("stats.map")}</th><th>{t("stats.date")}</th><th>{t("stats.score")}</th><th>K/D/A</th><th>ADR</th><th>{t("stats.rating")}</th></tr></thead>
              <tbody>{recent.map(m => (
                <tr key={m.matchId} onClick={() => { setSelMatch(m.matchId); setSub("matchDetail"); }} style={{ cursor: "pointer" }}>
                  <td style={{ fontWeight: 600 }}>{cs2ssMapLabel(m.map)}</td><td style={{ color: "var(--text-secondary)", fontSize: 12 }}>{fmtD(m.startedAt)}</td>
                  <td className="stats-table__score">{m.teamAScore} : {m.teamBScore}</td>
                  <td>{m.totalKills}/{m.totalDeaths}/{m.totalAssists}</td><td>{cs2ssCalcAdr(m.totalDamage, m.roundsPlayed).toFixed(1)}</td>
                  <td><span style={{ fontWeight: 700, color: rcol(m.r) }}>{m.r.toFixed(2)}</span></td>
                </tr>
              ))}</tbody></table>
          ) : <div className="stats-panel__empty">{t("stats.noMatches")}</div>}
        </div>
      </>) : mode === "deathmatch" && dm ? (<>
        <div className="stats-hero" style={{ background: "linear-gradient(125deg, #151923, #283448 58%, #df6b35)" }}>
          <div>
            <span className="stats-hero__eyebrow">{t("stats.dmTrainingLog")}</span>
            <h1>{dm.sessionCount} {t("stats.sessions")}</h1>
            <p style={{ color: "rgba(255,255,255,.65)", fontSize: 13 }}>
              {t("stats.totalMinutes", { minutes: Math.round(dm.totalSessionSec / 60) })} · KPM {dm.totalKills > 0 ? (dm.totalKills / Math.max(1, dm.totalSessionSec) * 60).toFixed(2) : "0"} · DPM {dm.totalDamage > 0 ? Math.round(dm.totalDamage / Math.max(1, dm.totalSessionSec) * 60) : 0}
            </p>
          </div>
          <div className="stats-hero__rating">
            <small>{t("stats.highScore")}</small>
            <strong style={{ color: "#f29968" }}>{highScoreSession?.score ?? 0}</strong>
            <p style={{ color: "rgba(255,255,255,.65)", fontSize: 11, margin: 0 }}>{highScoreSession ? cs2ssMapLabel(highScoreSession.map) : ""}</p>
          </div>
        </div>

        <div className="stats-cards">
          {[
            [t("stats.matches"), dm.sessionCount],
            [t("stats.averageKpm"), (dm.totalKills / Math.max(1, dm.totalSessionSec) * 60).toFixed(2)],
            [t("stats.averageDpm"), Math.round(dm.totalDamage / Math.max(1, dm.totalSessionSec) * 60)],
            [t("stats.averageKd"), dm.totalDeaths > 0 ? (dm.totalKills / dm.totalDeaths).toFixed(2) : dm.totalKills],
            ["HS%", dm.totalKills > 0 ? Math.round(dm.totalHeadshots / dm.totalKills * 100) + "%" : "0%"],
            [t("stats.maxStreak"), dm.maxStreak],
            [t("stats.longestLife"), Math.round(dm.maxLongestLife) + "s"],
            [t("stats.averageLife"), dm.totalSpawns > 0 ? (dm.totalAliveSec / dm.totalSpawns).toFixed(1) + "s" : "0s"],
          ].map(([l, v]: any) => (
            <div className="stats-card" key={l as string}>
              <span className="stats-card__label">{l as string}</span>
              <span className="stats-card__value" style={{ fontSize: 22, color: "#c14e21" }}>{v}</span>
            </div>
          ))}
        </div>

        <div className="stats-impact">
          {[
            ["5s 2K", dm.totalBurst5_2 ?? 0], ["5s 3K", dm.totalBurst5_3 ?? 0], ["5s 4K+", dm.totalBurst5_4 ?? 0],
            ["10s 2K", dm.totalBurst10_2 ?? 0], ["10s 3K", dm.totalBurst10_3 ?? 0], ["10s 4K+", dm.totalBurst10_4 ?? 0],
            [t("stats.averageKills"), Math.round((dm.totalKills ?? 0) / Math.max(1, dm.sessionCount))], [t("stats.averageDeaths"), Math.round((dm.totalDeaths ?? 0) / Math.max(1, dm.sessionCount))],
            [t("stats.averageScore"), Math.round((dm.totalScore ?? 0) / Math.max(1, dm.sessionCount))], ["HS%", ((dm.totalHeadshots ?? 0) / Math.max(1, dm.totalKills ?? 1) * 100).toFixed(0) + "%"]
          ].map(([l, v]: any) => (
            <div key={l as string}><span>{l as string}</span><b>{String(v)}</b></div>
          ))}
        </div>

        <div className="stats-charts">
          <div className="stats-panel-block">
            <div className="stats-panel-block__title"><div><span>{t("stats.trend")}</span><h2>{t("stats.kpmTrend")}</h2></div></div>
            {[...(dm.sessions ?? [])].reverse().slice(-20).length > 0 ? (
              <ResponsiveContainer width="100%" height={200}>
                <LineChart data={[...(dm.sessions ?? [])].reverse().slice(-20).map((s, i) => ({ i: i + 1, kpm: s.kpm, kd: s.kd }))}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="i" tick={{ fontSize: 11 }} />
                  <YAxis domain={[0, "auto"]} tick={{ fontSize: 11 }} />
                  <Tooltip />
                  <Line type="monotone" dataKey="kpm" stroke="#df6b35" strokeWidth={2.2} dot={{ r: 3, fill: "#df6b35" }} />
                </LineChart>
              </ResponsiveContainer>
            ) : <p style={{ color: "var(--text-secondary)", textAlign: "center", padding: 32 }}>{t("stats.noData")}</p>}
          </div>
          <div className="stats-panel-block">
            <div className="stats-panel-block__title"><div><span>{t("stats.maps")}</span><h2>{t("stats.scoreByMap")}</h2></div></div>
            {(dm.perMap ?? []).length > 0 ? (
              <ResponsiveContainer width="100%" height={200}>
                <BarChart data={(dm.perMap ?? []).map(m => ({ map: cs2ssMapLabel(m.map), score: m.avgKpm }))} layout="vertical">
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis type="number" domain={[0, "auto"]} tick={{ fontSize: 11 }} />
                  <YAxis type="category" dataKey="map" tick={{ fontSize: 11 }} width={72} />
                  <Tooltip />
                  <Bar dataKey="score" radius={[0, 4, 4, 0]} fill="#df6b35" />
                </BarChart>
              </ResponsiveContainer>
            ) : <p style={{ color: "var(--text-secondary)", textAlign: "center", padding: 32 }}>{t("stats.noMapData")}</p>}
          </div>
        </div>

        <div className="stats-panel-block">
          <div className="stats-panel-block__title"><div><span>{t("stats.sessions")}</span><h2>{t("stats.dmSessions")}</h2></div><p>{(dm.sessions ?? []).length} {t("stats.sessions")}</p></div>
          {(dm.sessions ?? []).length > 0 ? (
            <table className="stats-table"><thead><tr><th>{t("stats.map")}</th><th>{t("stats.date")}</th><th>{t("stats.mode")}</th><th>{t("stats.score")}</th><th style={{ textAlign: "right" }}>K/D</th><th style={{ textAlign: "right" }}>KPM</th><th style={{ textAlign: "right" }}>DPM</th><th style={{ textAlign: "right" }}>HS%</th><th style={{ textAlign: "right" }}>{t("stats.streak")}</th></tr></thead>
              <tbody>{(dm.sessions ?? []).map(s => (
                <tr key={s.matchId} onClick={() => { setSelMatch(s.matchId); setSub("matchDetail"); }} style={{ cursor: "pointer" }}>
                  <td style={{ fontWeight: 600 }}>{cs2ssMapLabel(s.map)} <span className="dm-tag">DM</span></td>
                  <td style={{ color: "var(--text-secondary)", fontSize: 12 }}>{fmtD(s.startedAt)}</td>
                  <td style={{ textTransform: "uppercase", fontSize: 11, color: "#df6b35", fontWeight: 700 }}>{s.ruleset}</td>
                  <td style={{ fontWeight: 700 }}>{s.score}</td>
                  <td style={{ textAlign: "right" }}>{s.kd.toFixed(2)}</td>
                  <td style={{ textAlign: "right" }}>{s.kpm.toFixed(1)}</td>
                  <td style={{ textAlign: "right" }}>{Math.round(s.dpm)}</td>
                  <td style={{ textAlign: "right" }}>{Math.round(s.headshotPct)}%</td>
                  <td style={{ textAlign: "right", fontWeight: 600 }}>{s.streak}</td>
                </tr>
              ))}</tbody></table>
          ) : <div className="stats-panel__empty">{t("stats.noDmSessions")}</div>}
        </div>
      </>) : mode === "deathmatch" ? (
        <div className="stats-panel-block"><div className="stats-panel__empty" style={{ padding: "60px 0", textAlign: "center" }}>{t("stats.emptyDeathmatch")}</div></div>
      ) : (
        <div className="stats-panel-block"><div className="stats-panel__empty" style={{ padding: "60px 0", textAlign: "center" }}>{t("stats.emptyCompetitive")}</div></div>
      )}
    </div>
  );
}
