using System.Text.Json;
using System.Text.Json.Serialization;
using BotHiderApi;
using CounterStrikeSharp.API;
using CounterStrikeSharp.API.Core;
using CounterStrikeSharp.API.Core.Attributes;
using CounterStrikeSharp.API.Core.Capabilities;
using CounterStrikeSharp.API.Modules.Utils;
using Microsoft.Extensions.Logging;

namespace TeamLineupInjector;

[MinimumApiVersion(301)]
public sealed class TeamLineupInjectorPlugin : BasePlugin
{
    private static readonly PluginCapability<IBotHiderApi> BotHiderCapability = new("bothider:api");

    public override string ModuleName => "Team Lineup Injector";
    public override string ModuleVersion => "1.0.0";
    public override string ModuleAuthor => "Local-Arena contributors";
    public override string ModuleDescription => "Auto-injects team lineup bots and identity after human picks a side";

    private IBotHiderApi? _botHider;
    private string? _csgoRoot;
    private string? _currentMap;
    private bool _injected;

    public override void Load(bool hotReload)
    {
        RegisterListener<Listeners.OnMapStart>(OnMapStart);
        RegisterEventHandler<EventPlayerTeam>(OnPlayerTeam);
        Logger.LogInformation("[TeamLineup] Plugin loaded");
    }

    private void OnMapStart(string mapName)
    {
        _currentMap = mapName;
        _injected = false;
        Logger.LogInformation("[TeamLineup] Map started: {Map}, waiting for human to pick a side", mapName);
    }

    private HookResult OnPlayerTeam(EventPlayerTeam @event, GameEventInfo info)
    {
        if (_injected) return HookResult.Continue;

        var player = @event.Userid;
        if (player is not { IsValid: true, IsBot: false }) return HookResult.Continue;

        var team = @event.Team;
        if (team != (byte)CsTeam.Terrorist && team != (byte)CsTeam.CounterTerrorist)
            return HookResult.Continue;

        if (MatchSessionActive())
        {
            Logger.LogInformation("[TeamLineup] Active match session detected, not interfering with the Match roster");
            ClearTeamIdentity();
            return HookResult.Continue;
        }

        var config = ReadLineupConfig();
        if (config is not { Enabled: true })
        {
            Logger.LogInformation("[TeamLineup] Lineup disabled or config missing, not interfering");
            ClearTeamIdentity();
            RestoreBotQuota();
            return HookResult.Continue;
        }

        if (config.FriendlyTeam == null && config.EnemyTeam == null)
        {
            Logger.LogInformation("[TeamLineup] Lineup enabled but no teams configured, not interfering");
            return HookResult.Continue;
        }

        _injected = true;
        Logger.LogInformation("[TeamLineup] Human player joined team {Team}, injecting lineup", team);

        AddTimer(0.5f, () =>
        {
            var current = ReadLineupConfig();
            if (current is not { Enabled: true } || MatchSessionActive())
            {
                _injected = false;
                return;
            }
            Server.ExecuteCommand("bot_kick");
            Server.ExecuteCommand("bot_quota 0");
        });

        AddTimer(1.5f, () =>
        {
            var current = ReadLineupConfig();
            if (current is not { Enabled: true } || MatchSessionActive())
            {
                _injected = false;
                return;
            }
            ExecuteLineup(current, team == (byte)CsTeam.CounterTerrorist);
        });

        return HookResult.Continue;
    }

    private LineupConfig? ReadLineupConfig()
    {
        var configPath = LineupConfigPath();
        if (!File.Exists(configPath))
        {
            Logger.LogInformation("[TeamLineup] No lineup config found at {Path}", configPath);
            return null;
        }

        try
        {
            var json = File.ReadAllText(configPath);
            return JsonSerializer.Deserialize<LineupConfig>(json);
        }
        catch (Exception ex)
        {
            Logger.LogWarning(ex, "[TeamLineup] Failed to read lineup config");
            return null;
        }
    }

    private void ExecuteLineup(LineupConfig config, bool humanIsCt)
    {
        var addedNames = new List<string>();
        var friendlyBotCmd = humanIsCt ? "bot_add_ct" : "bot_add_t";
        var enemyBotCmd = humanIsCt ? "bot_add_t" : "bot_add_ct";
        var friendlyTeamNum = humanIsCt ? 1 : 2;
        var enemyTeamNum = humanIsCt ? 2 : 1;

        if (config.FriendlyTeam != null)
        {
            var friendlyPlayers = config.FriendlyTeam.Players
                .Where(p => p != config.ExcludedPlayer)
                .ToArray();
            foreach (var player in friendlyPlayers)
            {
                Server.ExecuteCommand($"{friendlyBotCmd} \"{player}\"");
                addedNames.Add(player);
            }

            if (!string.IsNullOrWhiteSpace(config.FriendlyTeam.Logo))
                Server.ExecuteCommand($"mp_teamlogo_{friendlyTeamNum} {config.FriendlyTeam.Logo}");
            if (!string.IsNullOrWhiteSpace(config.FriendlyTeam.Name))
                Server.ExecuteCommand($"mp_teamname_{friendlyTeamNum} {config.FriendlyTeam.Name}");

            Logger.LogInformation("[TeamLineup] Added {Count} friendly bots ({Cmd})", friendlyPlayers.Length, friendlyBotCmd);
        }

        if (config.EnemyTeam != null)
        {
            foreach (var player in config.EnemyTeam.Players)
            {
                Server.ExecuteCommand($"{enemyBotCmd} \"{player}\"");
                addedNames.Add(player);
            }

            if (!string.IsNullOrWhiteSpace(config.EnemyTeam.Logo))
                Server.ExecuteCommand($"mp_teamlogo_{enemyTeamNum} {config.EnemyTeam.Logo}");
            if (!string.IsNullOrWhiteSpace(config.EnemyTeam.Name))
                Server.ExecuteCommand($"mp_teamname_{enemyTeamNum} {config.EnemyTeam.Name}");

            Logger.LogInformation("[TeamLineup] Added {Count} enemy bots ({Cmd})", config.EnemyTeam.Players.Length, enemyBotCmd);
        }

        Server.ExecuteCommand("mp_restartgame 3");

        AddTimer(1.0f, () =>
        {
            BindBotIdentities(addedNames);
        });
    }

    private void BindBotIdentities(List<string> expectedNames)
    {
        if (!ResolveBotHiderApi())
        {
            Logger.LogWarning("[TeamLineup] BotHider API unavailable");
            return;
        }

        var bots = Utilities.GetPlayers()
            .Where(p => p is { IsValid: true, IsBot: true })
            .OrderBy(p => p.Slot)
            .ToArray();

        foreach (var bot in bots)
        {
            if (_botHider == null || !_botHider.IsManagedBot(bot.Slot))
                continue;

            var name = bot.PlayerName;
            _botHider.SetPersonaName(bot.Slot, name);
            Logger.LogInformation("[TeamLineup] Set persona name for '{Name}' slot {Slot}", name, bot.Slot);
        }
    }

    private bool ResolveBotHiderApi()
    {
        if (_botHider != null) return true;
        try
        {
            var api = BotHiderCapability.Get()
                ?? throw new InvalidOperationException("BotHider capability returned no API instance");
            if (!api.SetDisguise(true) || !api.SetNameSource(true))
                throw new InvalidOperationException("BotHider shared-memory commands were rejected");
            _botHider = api;
            Logger.LogInformation("[TeamLineup] BotHider API connected");
            return true;
        }
        catch (Exception error)
        {
            Logger.LogError(error, "[TeamLineup] BotHider API is unavailable");
            return false;
        }
    }

    private bool TryResolveCsgoRoot()
    {
        var gameDir = Server.GameDirectory;
        if (string.IsNullOrWhiteSpace(gameDir))
        {
            Logger.LogError("[TeamLineup] Server.GameDirectory is empty");
            return false;
        }

        var candidates = new List<string>();
        var reported = Path.GetFullPath(gameDir).TrimEnd(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar);
        var leaf = Path.GetFileName(reported);

        if (leaf.Equals("csgo", StringComparison.OrdinalIgnoreCase))
            candidates.Add(reported);
        else if (leaf.Equals("game", StringComparison.OrdinalIgnoreCase))
            candidates.Add(Path.Combine(reported, "csgo"));
        candidates.Add(Path.Combine(reported, "game", "csgo"));
        candidates.Add(Path.Combine(reported, "csgo"));

        foreach (var candidate in candidates)
        {
            if (File.Exists(Path.Combine(candidate, "gameinfo.gi")))
            {
                _csgoRoot = candidate;
                return true;
            }
        }

        Logger.LogError("[TeamLineup] Cannot resolve csgo root from {Dir}", gameDir);
        return false;
    }

    /// <summary>True when the PLUS match coordinator owns the current roster (match-active.json present).</summary>
    private bool MatchSessionActive()
    {
        if (_csgoRoot == null && !TryResolveCsgoRoot()) return false;
        return File.Exists(Path.Combine(_csgoRoot ?? ".", ".csbip", "match-active.json"));
    }

    private void ClearTeamIdentity()
    {
        Server.ExecuteCommand("mp_teamname_1 \"\"");
        Server.ExecuteCommand("mp_teamname_2 \"\"");
        Server.ExecuteCommand("mp_teamlogo_1 \"\"");
        Server.ExecuteCommand("mp_teamlogo_2 \"\"");
    }

    private void RestoreBotQuota()
    {
        Server.ExecuteCommand("bot_quota_mode fill");
        Server.ExecuteCommand("bot_quota 10");
        Server.ExecuteCommand("mp_restartgame 1");
        Logger.LogInformation("[TeamLineup] Restored default bot quota and restarted game");
    }

    private string LineupConfigPath()
    {
        if (_csgoRoot == null) TryResolveCsgoRoot();
        return Path.Combine(_csgoRoot ?? ".", ".csbip", "team-lineup.json");
    }
}

public sealed class LineupConfig
{
    [JsonPropertyName("enabled")]
    public bool Enabled { get; set; }

    [JsonPropertyName("friendly_team")]
    public LineupTeam? FriendlyTeam { get; set; }

    [JsonPropertyName("enemy_team")]
    public LineupTeam? EnemyTeam { get; set; }

    [JsonPropertyName("excluded_player")]
    public string? ExcludedPlayer { get; set; }
}

public sealed class LineupTeam
{
    [JsonPropertyName("logo")]
    public string Logo { get; set; } = string.Empty;

    [JsonPropertyName("name")]
    public string Name { get; set; } = string.Empty;

    [JsonPropertyName("players")]
    public string[] Players { get; set; } = Array.Empty<string>();
}