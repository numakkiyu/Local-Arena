import { useCallback, useEffect, useState } from "react";
import Section from "../components/Section";
import Segmented from "../components/Segmented";
import SubPage from "../components/SubPage";
import Toggle, { type Tone } from "../components/Toggle";
import Dropdown from "../components/Dropdown";
import { useStore } from "../state/store";
import { useT, type I18nKey } from "../i18n";
import type { AimValue, BotItemKey, NadesValue } from "../lib/api";
import type { Status } from "../components/StatusDot";
import { TEAMS } from "../data/commands";
import "./PresetsPanel.css";
import "./BotItemsPanel.css";

const AIM: { value: AimValue; labelKey: I18nKey; descriptionKey: I18nKey }[] = [
  { value: "head", labelKey: "pre.aimHead", descriptionKey: "pre.aimHeadDesc" },
  { value: "mixed", labelKey: "pre.aimMixed", descriptionKey: "pre.aimMixedDesc" },
  { value: "body", labelKey: "pre.aimBody", descriptionKey: "pre.aimBodyDesc" },
];
const NADES: { value: NadesValue; labelKey: I18nKey; descriptionKey: I18nKey }[] = [
  { value: "max", labelKey: "pre.nadesMax", descriptionKey: "pre.nadesMaxDesc" },
  { value: "more", labelKey: "pre.nadesMore", descriptionKey: "pre.nadesMoreDesc" },
  { value: "normal", labelKey: "pre.nadesNormal", descriptionKey: "pre.nadesNormalDesc" },
  { value: "less", labelKey: "pre.nadesLess", descriptionKey: "pre.nadesLessDesc" },
  { value: "off", labelKey: "pre.nadesOff", descriptionKey: "pre.nadesOffDesc" },
];

const ITEMS: { key: BotItemKey; labelKey: I18nKey }[] = [
  { key: "skins", labelKey: "bi.skins" },
  { key: "profiles", labelKey: "bi.profiles" },
  { key: "agents", labelKey: "bi.agents" },
  { key: "music", labelKey: "bi.music" },
];

export default function PresetsPanel({ onBack }: { onBack?: () => void }) {
  const {
    presets,
    config,
    botItems,
    csgoPath,
    teamLineup,
    timescaleToggleEnabled,
    applyAim,
    applyNades,
    applyBotItem,
    applyTeamLineup,
    applyTimescaleToggle,
    aimPending,
    nadesPending,
    botItemsPending,
    teamLineupPending,
  } = useStore();
  const t = useT();

  const cfgPresent = presets?.cfg_present ?? false;
  const running = presets?.cs2_running ?? false;
  const disabled = !csgoPath || !cfgPresent;

  const [lineupEnabled, setLineupEnabled] = useState(false);
  const [friendlyIdx, setFriendlyIdx] = useState<string | null>(null);
  const [enemyIdx, setEnemyIdx] = useState<string | null>(null);
  const [excludedPlayer, setExcludedPlayer] = useState<string | null>(null);

  useEffect(() => {
    if (teamLineup) {
      setLineupEnabled(teamLineup.enabled);
      setFriendlyIdx(teamLineup.friendly_team_index);
      setEnemyIdx(teamLineup.enemy_team_index);
      setExcludedPlayer(teamLineup.excluded_player);
    }
  }, [teamLineup]);

  const saveLineup = useCallback(
    (enabled: boolean, friendly: string | null, enemy: string | null, excluded: string | null) => {
      if (!csgoPath) return;
      applyTeamLineup({
        enabled,
        friendly_team_index: friendly,
        enemy_team_index: enemy,
        excluded_player: excluded,
      });
    },
    [csgoPath, applyTeamLineup]
  );

  const aimSupported = presets?.aim_supported ?? false;
  const aimRuntimeActive = presets?.aim_active;

  const statusFor = (pending: boolean): Status =>
    !csgoPath ? "off" : !cfgPresent ? "red" : running && pending ? "yellow" : "green";
  const aim: AimValue = presets?.aim ?? ((config?.aim as AimValue | null) ?? "mixed");
  const nades: NadesValue =
    presets?.nades ?? ((config?.nades as NadesValue | null) ?? "normal");
  const aimOption = AIM.find(({ value }) => value === aim) ?? AIM[1];
  const aimRuntimeDetail = !aimSupported
    ? t("pre.aimUnavailable")
    : aimRuntimeActive === true
    ? t("pre.aimRuntimeActive", {
        count: presets?.aim_override_count ?? 0,
        errors: presets?.aim_error_count ?? 0,
      })
    : aimRuntimeActive === false
    ? t("pre.aimRuntimeInactive")
    : t("pre.aimRuntimeUnknown");
  const nadesOption = NADES.find(({ value }) => value === nades) ?? NADES[2];
  const botItemsCfgPresent = botItems?.cfg_present ?? false;
  const botItemsRunning = botItems?.cs2_running ?? false;
  const itemPending = (key: BotItemKey) => botItemsRunning && botItemsPending[key];
  const itemsStatus: Status = !csgoPath
    ? "off"
    : !botItemsCfgPresent
    ? "red"
    : ITEMS.some(({ key }) => itemPending(key))
    ? "yellow"
    : "green";

  const friendlyTeam = TEAMS.find((t) => String(t.index) === friendlyIdx) ?? null;

  const teamOptions = TEAMS.filter((t) => String(t.index) !== enemyIdx).map((t) => ({
    value: String(t.index),
    label: t.name,
  }));
  const enemyOptions = TEAMS.filter((t) => String(t.index) !== friendlyIdx).map((t) => ({
    value: String(t.index),
    label: t.name,
  }));

  return (
    <SubPage title={t("pre.title")} onBack={onBack}>
      <div className="presets__controls">
        <Section
          title={t("pre.aim")}
          status={!aimSupported ? "off" : aimRuntimeActive === false ? "red" : statusFor(aimPending)}
        >
          <Segmented
            ariaLabel={t("pre.aim")}
            value={aim}
            onChange={(v) => applyAim(v)}
            disabled={disabled || !aimSupported}
            options={AIM.map(({ value, labelKey }) => ({ value, label: t(labelKey) }))}
          />
          <p className="selection-detail" aria-live="polite">
            {aimSupported ? t(aimOption.descriptionKey) : aimRuntimeDetail}
          </p>
          <p className="selection-detail" aria-live="polite">
            {aimRuntimeDetail}
          </p>
          <p className="selection-detail">{t("pre.appliesNextLaunch")}</p>
        </Section>

        <Section title={t("pre.nades")} status={statusFor(nadesPending)}>
          <Segmented
            ariaLabel={t("pre.nades")}
            value={nades}
            onChange={(v) => applyNades(v)}
            disabled={disabled}
            options={NADES.map(({ value, labelKey }) => ({ value, label: t(labelKey) }))}
          />
          <p className="selection-detail" aria-live="polite">
            {t(nadesOption.descriptionKey)}
          </p>
        </Section>

        <Section title={t("bi.title")} status={itemsStatus}>
          <div className="botitems-grid">
            {ITEMS.map(({ key, labelKey }) => {
              const on = (botItems?.[key] as boolean | undefined) ?? false;
              const tone: Tone = !botItemsCfgPresent
                ? "red"
                : itemPending(key)
                ? "yellow"
                : "green";
              return (
                <div className="botitem" key={key}>
                  <span className="botitem__label">{t(labelKey)}</span>
                  <Toggle
                    ariaLabel={t(labelKey)}
                    checked={on}
                    tone={tone}
                    disabled={!csgoPath || !botItemsCfgPresent}
                    onChange={(next) => applyBotItem(key, next)}
                  />
                </div>
              );
            })}
          </div>
        </Section>

        <Section title={t("pre.timescaleToggle")}>
          <div className="botitem">
            <span className="botitem__label">{t("pre.timescaleToggleDesc")}</span>
            <Toggle
              ariaLabel={t("pre.timescaleToggle")}
              checked={timescaleToggleEnabled}
              tone={!cfgPresent ? "red" : "green"}
              disabled={!csgoPath || !cfgPresent}
              onChange={(next) => applyTimescaleToggle(next)}
            />
          </div>
          <p className="selection-detail">{t("pre.timescaleToggleHint")}</p>
        </Section>

        <div className="teamlineup-section">
        <Section title={t("pre.teamLineup")} status={statusFor(teamLineupPending)}>
          <div className="teamlineup">
            <div className="teamlineup__toggle">
              <span className="teamlineup__label">{t("pre.teamLineupToggle")}</span>
              <Toggle
                ariaLabel={t("pre.teamLineupToggle")}
                checked={lineupEnabled}
                tone={!cfgPresent ? "red" : running && teamLineupPending ? "yellow" : "green"}
                disabled={disabled}
                onChange={(next) => {
                  setLineupEnabled(next);
                  if (next) {
                    const f = friendlyIdx || "1";
                    const e = enemyIdx || "8";
                    const team = TEAMS.find(t => String(t.index) === f);
                    const ex = excludedPlayer || team?.players[0] || null;
                    if (f !== friendlyIdx) setFriendlyIdx(f);
                    if (e !== enemyIdx) setEnemyIdx(e);
                    if (ex !== excludedPlayer) setExcludedPlayer(ex);
                    saveLineup(next, f, e, ex);
                  } else {
                    saveLineup(false, friendlyIdx, enemyIdx, excludedPlayer);
                  }
                }}
              />
            </div>
            {lineupEnabled && (
              <div className="teamlineup__body">
                <div className="teamlineup__hint">{t("pre.teamLineupHint")}</div>

                <div className="teamlineup__select">
                  <span className="teamlineup__select-label">{t("pre.friendlyTeam")}</span>
                  <Dropdown
                    ariaLabel={t("pre.friendlyTeam")}
                    placeholder={t("pre.friendlyTeam")}
                    value={friendlyIdx}
                    disabled={disabled}
                    onChange={(v) => {
                      setFriendlyIdx(v);
                      const team = TEAMS.find(t => String(t.index) === v);
                      const firstPlayer = team?.players[0] ?? null;
                      setExcludedPlayer(firstPlayer);
                      saveLineup(lineupEnabled, v, enemyIdx, firstPlayer);
                    }}
                    options={teamOptions}
                  />
                </div>

                {friendlyTeam && (
                  <div className="teamlineup__exclusions">
                    <span className="teamlineup__select-label">{t("pre.excludePlayer")}</span>
                    <div className="teamlineup__players">
                      {friendlyTeam.players.map((player) => (
                        <button
                          key={player}
                          className={`teamlineup__player ${excludedPlayer === player ? "is-excluded" : ""}`}
                          disabled={disabled}
                          onClick={() => {
                            const next = excludedPlayer === player ? null : player;
                            setExcludedPlayer(next);
                            saveLineup(lineupEnabled, friendlyIdx, enemyIdx, next);
                          }}
                        >
                          {player}
                        </button>
                      ))}
                    </div>
                    <span className="teamlineup__sublabel">{t("pre.excludePlayerDesc")}</span>
                  </div>
                )}

                <div className="teamlineup__select">
                  <span className="teamlineup__select-label">{t("pre.enemyTeam")}</span>
                  <Dropdown
                    ariaLabel={t("pre.enemyTeam")}
                    placeholder={t("pre.enemyTeam")}
                    value={enemyIdx}
                    disabled={disabled}
                    onChange={(v) => {
                      setEnemyIdx(v);
                      saveLineup(lineupEnabled, friendlyIdx, v, excludedPlayer);
                    }}
                    options={enemyOptions}
                  />
                </div>

                <p className="selection-detail">{t("pre.appliesNextLaunch")}</p>
              </div>
            )}
          </div>
        </Section>
        </div>
      </div>
    </SubPage>
  );
}