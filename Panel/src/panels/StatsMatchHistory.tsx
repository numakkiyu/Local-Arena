import { useState, useEffect, useMemo, useCallback } from "react";
import { api } from "../lib/api";
import type { Cs2ssMatchWithStats } from "../data/cs2ssTypes";
import { cs2ssCalcRating, cs2ssCalcAdr } from "../data/cs2ssRating";
import { cs2ssMapLabel } from "../data/cs2ssMaps";
import { useStore } from "../state/store";
import { useT } from "../i18n";
import "./StatsPanel.css";

interface Props { csgo: string; onOpenMatch?: (id: number) => void; onBack?: () => void; }

function fmtDT(iso: string) { try { const d = new Date(iso); return `${d.getMonth() + 1}/${d.getDate()} ${d.getHours()}:${String(d.getMinutes()).padStart(2, "0")}`; } catch { return iso; } }
function rcol(r: number) { return r >= 1.1 ? "#20b486" : r >= 0.9 ? "#e67e22" : "#e05d75"; }

export default function StatsMatchHistory({ csgo, onOpenMatch, onBack }: Props) {
  const { reportError } = useStore();
  const t = useT();
  const [matches, setMatches] = useState<Cs2ssMatchWithStats[]>([]);
  const [loading, setLoading] = useState(true);
  const [err, setErr] = useState("");
  const [mapF, setMapF] = useState("");
  const [modeF, setModeF] = useState("all");
  const [dateF, setDateF] = useState("");
  const [dateT, setDateT] = useState("");
  const [deleting, setDeleting] = useState(false);
  const [selected, setSelected] = useState<Set<number>>(new Set());

  const load = useCallback(() => {
    setLoading(true);
    api.listCs2ssMatchesWithStats(csgo).then(ms => { setMatches(ms ?? []); setLoading(false); }).catch(e => { setErr(String(e)); setLoading(false); reportError(e); });
  }, [csgo, reportError]);

  useEffect(() => { load(); }, [load]);

  const maps = useMemo(() => [...new Set(matches.map(m => m.map))].sort(), [matches]);
  const filtered = useMemo(() => {
    let r = matches;
    if (mapF) r = r.filter(m => m.map === mapF);
    if (modeF !== "all") r = r.filter(m => m.modeFamily === modeF);
    if (dateF) { const t = new Date(dateF).getTime(); r = r.filter(m => new Date(m.startedAt).getTime() >= t); }
    if (dateT) { const t = new Date(dateT + "T23:59:59").getTime(); r = r.filter(m => new Date(m.startedAt).getTime() <= t); }
    return r;
  }, [matches, mapF, modeF, dateF, dateT]);

  const toggleSelect = (id: number) => {
    setSelected(prev => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id); else next.add(id);
      return next;
    });
  };

  const toggleSelectAll = () => {
    if (selected.size === filtered.length) {
      setSelected(new Set());
    } else {
      setSelected(new Set(filtered.map(m => m.matchId)));
    }
  };

  const handleDelete = async () => {
    if (selected.size === 0) return;
    const ids = Array.from(selected);
    try {
      await api.deleteCs2ssMatches(csgo, ids);
      setSelected(new Set());
      setDeleting(false);
      load();
    } catch (e) {
      reportError(e);
    }
  };

  if (loading) return <div className="stats-panel"><div className="stats-panel__loading">{t("stats.loading")}</div></div>;
  if (err) return <div className="stats-panel"><div className="stats-panel__error">{err}</div></div>;

  return (
    <div className="stats-panel">
      <div style={{ display: "flex", alignItems: "center", gap: 12, marginBottom: 22 }}>
        {onBack && <button className="stats-back" onClick={onBack}>← {t("stats.back")}</button>}
        <span style={{ fontSize: 13, color: "var(--text-secondary)", fontWeight: 600 }}>{t("stats.matchesShown", { shown: filtered.length, total: matches.length })}</span>
        <div style={{ flex: 1 }} />
        {deleting ? (
          <>
            <span style={{ fontSize: 12, color: "var(--c-red)", fontWeight: 600 }}>{t("stats.selected", { n: selected.size })}</span>
            <button className="stats-delete__confirm" onClick={handleDelete} disabled={selected.size === 0}>{t("stats.confirmDelete")}</button>
            <button className="stats-delete__cancel" onClick={() => { setDeleting(false); setSelected(new Set()); }}>{t("stats.cancel")}</button>
          </>
        ) : (
          <button className="stats-delete__enter" onClick={() => setDeleting(true)}>{t("stats.deleteMatches")}</button>
        )}
      </div>

      <div className="stats-filters">
        <label><span>{t("stats.map")}</span><select value={mapF} onChange={e => setMapF(e.target.value)}><option value="">{t("stats.all")}</option>{maps.map(m => <option key={m} value={m}>{cs2ssMapLabel(m)}</option>)}</select></label>
        <label><span>{t("stats.mode")}</span><select value={modeF} onChange={e => setModeF(e.target.value)}><option value="all">{t("stats.all")}</option><option value="competitive">{t("stats.competitive")}</option><option value="deathmatch">{t("stats.deathmatch")}</option></select></label>
        <label><span>{t("stats.from")}</span><input type="date" value={dateF} onChange={e => setDateF(e.target.value)} /></label>
        <label><span>{t("stats.to")}</span><input type="date" value={dateT} onChange={e => setDateT(e.target.value)} /></label>
        <button onClick={() => { setMapF(""); setModeF("all"); setDateF(""); setDateT(""); }}>{t("stats.reset")}</button>
      </div>

      <div className="stats-panel-block" style={{ padding: 0 }}>
        <table className="stats-table">
          <thead><tr>
            {deleting && <th style={{ width: 40 }}><input type="checkbox" checked={selected.size === filtered.length && filtered.length > 0} onChange={toggleSelectAll} /></th>}
            <th>{t("stats.map")}</th><th>{t("stats.date")}</th><th>{t("stats.score")}</th><th>{t("stats.rounds")}</th><th style={{ textAlign: "right" }}>K/D/A</th><th style={{ textAlign: "right" }}>ADR</th><th style={{ textAlign: "right" }}>{t("stats.rating")}</th>
          </tr></thead>
          <tbody>
            {filtered.length === 0 ? (
              <tr><td colSpan={deleting ? 8 : 7} style={{ textAlign: "center", padding: 40, color: "var(--text-secondary)" }}>{t("stats.noFilteredMatches")}</td></tr>
            ) : filtered.map(m => {
              const dm = m.modeFamily === "deathmatch";

              const rating = !dm && m.roundsPlayed > 0
                ? cs2ssCalcRating(m.playerKills, m.playerDeaths, m.playerAssists, m.playerDamage, m.playerHeadshots, m.roundsPlayed, {
                    kastRounds: m.playerKastRounds, tradeKills: m.playerTradeKills,
                    multikill2: m.playerMk2, multikill3: m.playerMk3, multikill4: m.playerMk4, multikill5: m.playerMk5,
                    clutchAttempts: m.playerClutchAttempts, clutchesWon: m.playerClutchesWon,
                  })
                : 0;
              const adr = !dm ? cs2ssCalcAdr(m.playerDamage, m.roundsPlayed) : 0;

              const durMin = Math.max(1, m.durationSeconds / 60);
              const dpm = dm ? Math.round(m.playerDamage / durMin) : 0;
              const kpm = dm ? Math.round(m.playerKills / durMin * 100) / 100 : 0;

              return (
                <tr key={m.matchId} onClick={() => { if (deleting) toggleSelect(m.matchId); else onOpenMatch?.(m.matchId); }} style={{ cursor: "pointer" }}>
                  {deleting && <td onClick={e => e.stopPropagation()}><input type="checkbox" checked={selected.has(m.matchId)} onChange={() => toggleSelect(m.matchId)} /></td>}
                  <td style={{ fontWeight: 600 }}>{cs2ssMapLabel(m.map)}{dm && <span className="dm-tag">DM</span>}</td>
                  <td style={{ color: "var(--text-secondary)", fontSize: 12, whiteSpace: "nowrap" }}>{fmtDT(m.startedAt)}</td>
                  <td>
                    {dm ? (
                      <span style={{ color: "#df6b35", fontWeight: 700 }}>{t("stats.minutesShort", { count: Math.round(m.durationSeconds / 60) })}</span>
                    ) : (
                      <span className="stats-table__score">{m.teamAScore} : {m.teamBScore}</span>
                    )}
                  </td>
                  <td>{dm ? t("stats.minutesShort", { count: Math.round(m.durationSeconds / 60) }) : t("stats.roundsShort", { count: m.roundsPlayed })}</td>
                  <td style={{ textAlign: "right", fontVariantNumeric: "tabular-nums" }}>
                    {dm ? `${m.playerKills}/${m.playerDeaths}/${m.playerAssists}` : `${m.playerKills}/${m.playerDeaths}/${m.playerAssists}`}
                  </td>
                  <td style={{ textAlign: "right", fontVariantNumeric: "tabular-nums", color: "var(--text-secondary)" }}>
                    {dm ? `${dpm} DPM` : adr.toFixed(1)}
                  </td>
                  <td style={{ textAlign: "right", fontWeight: 700, fontVariantNumeric: "tabular-nums", color: dm ? "#df6b35" : rcol(rating) }}>
                    {dm ? `${kpm} KPM` : rating.toFixed(2)}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}