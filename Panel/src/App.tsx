import { useCallback, useEffect, useRef, useState } from "react";
import {
  BarChart3,
  BookOpenText,
  Command,
  Crosshair,
  History,
  LayoutDashboard,
  Settings2,
  SlidersHorizontal,
  Sticker,
  Swords,
  type LucideIcon,
} from "lucide-react";
import TitleBar from "./components/TitleBar";
import ErrorModal from "./components/ErrorModal";
import Modal from "./components/Modal";
import OverviewDashboard, { type DashboardTarget } from "./panels/OverviewDashboard";
import PresetsPanel from "./panels/PresetsPanel";
import CommandsPanel from "./panels/CommandsPanel";
import WeaponPresetsPanel from "./panels/WeaponPresetsPanel";
import MatchPanel from "./panels/MatchPanel";
import MatchHistoryPanel from "./panels/MatchHistoryPanel";
import StatsDashboard from "./panels/StatsDashboard";
import GuideView from "./panels/GuideView";
import SettingsView from "./panels/settings/SettingsView";
import StickersPanel from "./panels/StickersPanel";
import FirstRunLanguages from "./panels/settings/FirstRunLanguages";
import { useStore } from "./state/store";
import { useT, type I18nKey } from "./i18n";
import appLogo from "./assets/app-logo.png";
import { stickerFeatureEnabled } from "./lib/stickerEditor";
import { APP_DISPLAY_VERSION } from "./lib/version";
import { api } from "./lib/api";
import { openExternalUrl } from "./lib/platform";
import { useAppearance } from "./state/appearance";
import "./App.css";

type View = "main" | "stickers" | DashboardTarget;

const VIEWS: View[] = ["main", "match", "matchHistory", "stats", "settings", "presets", "commands", "weaponPresets", "stickers", "guide"];
const VIEW_KEY = "cs2bi.view";
const WELCOME_STORY_URL = "https://api.hypcvgm.top/la";

export default function App() {
  const { error, clearError, ready, config, exportDiagnostics, updateConfig, reportError } = useStore();
  const { appearance } = useAppearance();
  const t = useT();
  const storyPromptAttempted = useRef(false);
  const [storyOpen, setStoryOpen] = useState(false);
  // Remember the open view in the portable Panel memory.
  const [view, setView] = useState<View>(() => {
    const stored = localStorage.getItem(VIEW_KEY);
    const saved = (stored === "botItems" ? "presets" : stored) as View | null;
    return saved && VIEWS.includes(saved) ? saved : "main";
  });
  useEffect(() => {
    localStorage.setItem(VIEW_KEY, view);
  }, [view]);
  // Anchor inside the guide requested by the error modal deep link.
  const [guideAnchor, setGuideAnchor] = useState<string | null>(null);
  const clearGuideAnchor = useCallback(() => setGuideAnchor(null), []);
  const openGuide = useCallback((anchor: string) => {
    setGuideAnchor(anchor);
    setView("guide");
  }, []);
  const firstRun = ready && !!config && !config.first_run_done;
  const stickersVisible = stickerFeatureEnabled(config);

  useEffect(() => {
    if (
      !ready
      || !config?.first_run_done
      || config.welcome_story_prompt_presented
      || storyPromptAttempted.current
    ) return;
    storyPromptAttempted.current = true;
    void api.shouldPresentWelcomeStory()
      .then(async (eligible) => {
        if (!eligible) return;
        if (await updateConfig({ welcome_story_prompt_presented: true })) setStoryOpen(true);
      })
      .catch(reportError);
  }, [config?.first_run_done, config?.welcome_story_prompt_presented, ready, reportError, updateConfig]);

  const openWelcomeStory = async () => {
    setStoryOpen(false);
    try { await openExternalUrl(WELCOME_STORY_URL); }
    catch (error) { reportError(error); }
  };

  useEffect(() => {
    if (view === "stickers" && !stickersVisible) setView("main");
  }, [stickersVisible, view]);

  const NAV: { view: View; key: I18nKey; icon: LucideIcon }[] = [
    { view: "main", key: "nav.overview", icon: LayoutDashboard },
    { view: "match", key: "match.title", icon: Swords },
    { view: "matchHistory", key: "match.history", icon: History },
    { view: "stats", key: "stats.globalHistory", icon: BarChart3 },
    { view: "presets", key: "pre.title", icon: SlidersHorizontal },
    { view: "commands", key: "cmd.title", icon: Command },
    { view: "weaponPresets", key: "weapons.title", icon: Crosshair },
    ...(stickersVisible ? [{ view: "stickers" as View, key: "stickers.title" as I18nKey, icon: Sticker }] : []),
    { view: "guide", key: "nav.guide", icon: BookOpenText },
    { view: "settings", key: "set.title", icon: Settings2 },
  ];

  return (
    <div className="shell">
      <TitleBar
        title={`${appearance.brand_name} v${APP_DISPLAY_VERSION}`}
        showSettings
        onSettings={() => setView((v) => (v === "settings" ? "main" : "settings"))}
      />

      <div className="shell__frame">
        <aside className="sidebar">
          <div className="sidebar__brand">
            <img
              className={`sidebar__mark sidebar__mark--${appearance.logo?.shape ?? "rounded"}`}
              src={appearance.logo?.data_url || appLogo}
              style={{ objectFit: appearance.logo?.fit ?? "cover" }}
              alt=""
              aria-hidden="true"
            />
            <span className="sidebar__brand-copy">
              <strong>{appearance.brand_name}</strong>
            </span>
          </div>

          <span className="sidebar__label">{t("nav.workspace")}</span>
          <nav className="sidebar__nav" aria-label={t("nav.workspace")}>
            {NAV.map(({ view: target, key, icon: Icon }) => (
              <button
                key={target}
                className={`sidebar__item ${view === target ? "is-active" : ""}`}
                onClick={() => setView(target)}
                aria-current={view === target ? "page" : undefined}
              >
                <Icon size={18} strokeWidth={1.9} />
                <span>{t(key)}</span>
              </button>
            ))}
          </nav>

          <div className="sidebar__footer">
            <span>Local Arena</span>
            <small>v{APP_DISPLAY_VERSION}</small>
          </div>
        </aside>

        <main className="workspace">
          {view === "settings" ? (
            <SettingsView />
          ) : view === "presets" ? (
            <PresetsPanel />
          ) : view === "commands" ? (
            <CommandsPanel />
          ) : view === "weaponPresets" ? (
            <WeaponPresetsPanel />
          ) : view === "stickers" ? (
            <StickersPanel />
          ) : view === "match" ? (
            <MatchPanel onOpenInstallation={() => setView("settings")} onOpenHistory={() => setView("matchHistory")} onOpenLineup={() => setView("presets")} />
          ) : view === "matchHistory" ? (
            <MatchHistoryPanel />
          ) : view === "stats" ? (
            <StatsDashboard />
          ) : view === "guide" ? (
            <GuideView anchor={guideAnchor} onAnchorHandled={clearGuideAnchor} />
          ) : (
            <OverviewDashboard onNavigate={setView} />
          )}
        </main>
      </div>

      <ErrorModal error={error} onClose={clearError} onExport={exportDiagnostics} onOpenGuide={openGuide} />
      {firstRun && <FirstRunLanguages />}
      <Modal
        open={storyOpen}
        title={t("first.storyTitle")}
        onClose={() => setStoryOpen(false)}
        footer={<>
          <button className="welcome-story__secondary" onClick={() => void openWelcomeStory()}>{t("first.storyListen")}</button>
          <button className="welcome-story__primary" onClick={() => setStoryOpen(false)}>{t("first.storyDecline")}</button>
        </>}
      >
        <p className="welcome-story__copy">{t("first.storyBody")}</p>
      </Modal>
    </div>
  );
}
