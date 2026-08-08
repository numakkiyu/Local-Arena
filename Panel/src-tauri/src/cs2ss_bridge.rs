use crate::{write_json_atomic, AppError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn cs2ss_root(csgo: &str) -> PathBuf {
    PathBuf::from(csgo).join(".csbip").join("cs2ss")
}

fn cs2ss_db_path(csgo: &str) -> PathBuf {
    cs2ss_root(csgo).join("telemetry.db")
}

fn cs2ss_config_path(csgo: &str) -> PathBuf {
    cs2ss_root(csgo).join("config.json")
}

fn open_db(csgo: &str) -> Result<rusqlite::Connection> {
    let root = cs2ss_root(csgo);
    std::fs::create_dir_all(&root).ok();
    let path = cs2ss_db_path(csgo);
    if !path.exists() {
        return Err(AppError::invalid(format!(
            "暂无本地统计数据 ({}). 请检查 Local Arena 安装状态，并在完成一场比赛后重试。",
            path.display()
        )));
    }
    rusqlite::Connection::open(&path)
        .map_err(|e| AppError::invalid(format!("Cannot open CS2SS database: {e}")))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssPlayerOverview {
    #[serde(rename = "steamId")]
    pub steam_id: String,
    pub name: String,
    pub matches: i64,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub damage: i64,
    pub headshots: i64,
    #[serde(rename = "totalRounds")]
    pub total_rounds: i64,
    #[serde(rename = "kastRounds")]
    pub kast_rounds: i64,
    #[serde(rename = "tradeKills")]
    pub trade_kills: i64,
    pub multikill2: i64,
    pub multikill3: i64,
    pub multikill4: i64,
    pub multikill5: i64,
    #[serde(rename = "clutchAttempts")]
    pub clutch_attempts: i64,
    #[serde(rename = "clutchesWon")]
    pub clutches_won: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssOverviewResponse {
    #[serde(rename = "matchCount")]
    pub match_count: i64,
    pub players: Vec<Cs2ssPlayerOverview>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssMatchSummary {
    #[serde(rename = "matchId")]
    pub match_id: i64,
    pub map: String,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "endedAt")]
    pub ended_at: Option<String>,
    #[serde(rename = "endReason")]
    pub end_reason: Option<String>,
    #[serde(rename = "roundsPlayed")]
    pub rounds_played: i64,
    #[serde(rename = "ctScore")]
    pub ct_score: i64,
    #[serde(rename = "tScore")]
    pub t_score: i64,
    #[serde(rename = "teamAScore")]
    pub team_a_score: i64,
    #[serde(rename = "teamBScore")]
    pub team_b_score: i64,
    #[serde(rename = "modeFamily")]
    pub mode_family: String,
    pub ruleset: String,
    #[serde(rename = "gameType")]
    pub game_type: i64,
    #[serde(rename = "gameMode")]
    pub game_mode: i64,
    #[serde(rename = "durationSeconds")]
    pub duration_seconds: i64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssRoundSummary {
    #[serde(rename = "roundId")]
    pub round_id: i64,
    #[serde(rename = "matchId")]
    pub match_id: i64,
    #[serde(rename = "roundNumber")]
    pub round_number: i64,
    #[serde(rename = "capturedAt")]
    pub captured_at: String,
    pub source: String,
    #[serde(rename = "winnerTeam")]
    pub winner_team: Option<String>,
    #[serde(rename = "endReason")]
    pub end_reason: Option<i64>,
    #[serde(rename = "ctScore")]
    pub ct_score: i64,
    #[serde(rename = "tScore")]
    pub t_score: i64,
    #[serde(rename = "teamAScore")]
    pub team_a_score: i64,
    #[serde(rename = "teamBScore")]
    pub team_b_score: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssRoundPlayer {
    #[serde(rename = "roundPlayerId")]
    pub round_player_id: i64,
    #[serde(rename = "roundId")]
    pub round_id: i64,
    #[serde(rename = "matchId")]
    pub match_id: i64,
    #[serde(rename = "steamId")]
    pub steam_id: String,
    pub name: String,
    pub team: String,
    #[serde(rename = "isBot")]
    pub is_bot: bool,
    pub alive: bool,
    pub health: i64,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub damage: i64,
    #[serde(rename = "headshotKills")]
    pub headshot_kills: i64,
    #[serde(rename = "totalKills")]
    pub total_kills: i64,
    #[serde(rename = "totalDeaths")]
    pub total_deaths: i64,
    #[serde(rename = "totalDamage")]
    pub total_damage: i64,
    pub score: i64,
    pub money: i64,
    pub kast: bool,
    pub survived: bool,
    pub traded: bool,
    #[serde(rename = "tradeKills")]
    pub trade_kills: i64,
    #[serde(rename = "eventKills")]
    pub event_kills: i64,
    pub multikill: i64,
    #[serde(rename = "clutchAttempt")]
    pub clutch_attempt: bool,
    #[serde(rename = "clutchWon")]
    pub clutch_won: bool,
    #[serde(rename = "clutchSize")]
    pub clutch_size: i64,
    #[serde(rename = "roundNumber")]
    pub round_number: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssMatchPlayer {
    #[serde(rename = "matchPlayerId")]
    pub match_player_id: i64,
    #[serde(rename = "matchId")]
    pub match_id: i64,
    #[serde(rename = "steamId")]
    pub steam_id: String,
    pub name: String,
    pub team: String,
    #[serde(rename = "isBot")]
    pub is_bot: bool,
    pub alive: bool,
    pub health: i64,
    #[serde(rename = "totalKills")]
    pub total_kills: i64,
    #[serde(rename = "totalDeaths")]
    pub total_deaths: i64,
    #[serde(rename = "totalAssists")]
    pub total_assists: i64,
    #[serde(rename = "totalDamage")]
    pub total_damage: i64,
    #[serde(rename = "totalHeadshotKills")]
    pub total_headshot_kills: i64,
    pub score: i64,
    pub money: i64,
    #[serde(rename = "kastRounds")]
    pub kast_rounds: i64,
    #[serde(rename = "tradeKills")]
    pub trade_kills: i64,
    pub multikill2: i64,
    pub multikill3: i64,
    pub multikill4: i64,
    pub multikill5: i64,
    #[serde(rename = "clutchAttempts")]
    pub clutch_attempts: i64,
    #[serde(rename = "clutchesWon")]
    pub clutches_won: i64,
    #[serde(rename = "dmSpawnCount")]
    pub dm_spawn_count: i64,
    #[serde(rename = "dmCompletedLives")]
    pub dm_completed_lives: i64,
    #[serde(rename = "dmMaxKillStreak")]
    pub dm_max_kill_streak: i64,
    #[serde(rename = "dmAliveSeconds")]
    pub dm_alive_seconds: i64,
    #[serde(rename = "dmLongestLifeSeconds")]
    pub dm_longest_life_seconds: i64,
    #[serde(rename = "dmBurst5s2")]
    pub dm_burst_5s_2: i64,
    #[serde(rename = "dmBurst5s3")]
    pub dm_burst_5s_3: i64,
    #[serde(rename = "dmBurst5s4")]
    pub dm_burst_5s_4: i64,
    #[serde(rename = "dmBurst10s2")]
    pub dm_burst_10s_2: i64,
    #[serde(rename = "dmBurst10s3")]
    pub dm_burst_10s_3: i64,
    #[serde(rename = "dmBurst10s4")]
    pub dm_burst_10s_4: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssDeathmatchLife {
    #[serde(rename = "lifeId")]
    pub life_id: i64,
    #[serde(rename = "matchId")]
    pub match_id: i64,
    #[serde(rename = "steamId")]
    pub steam_id: String,
    #[serde(rename = "lifeIndex")]
    pub life_index: i64,
    #[serde(rename = "spawnedAt")]
    pub spawned_at: String,
    #[serde(rename = "endedAt")]
    pub ended_at: String,
    #[serde(rename = "endKind")]
    pub end_kind: String,
    #[serde(rename = "durationSeconds")]
    pub duration_seconds: f64,
    pub kills: i64,
    pub damage: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssMatchDetailResponse {
    pub r#match: Cs2ssMatchSummary,
    pub rounds: Vec<Cs2ssRoundSummary>,
    #[serde(rename = "roundPlayers")]
    pub round_players: Vec<Cs2ssRoundPlayer>,
    #[serde(rename = "matchPlayers")]
    pub match_players: Vec<Cs2ssMatchPlayer>,
    #[serde(rename = "deathmatchLives")]
    pub deathmatch_lives: Vec<Cs2ssDeathmatchLife>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssPlayerMatchSummary {
    #[serde(rename = "matchId")]
    pub match_id: i64,
    pub map: String,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "roundsPlayed")]
    pub rounds_played: i64,
    #[serde(rename = "ctScore")]
    pub ct_score: i64,
    #[serde(rename = "tScore")]
    pub t_score: i64,
    #[serde(rename = "teamAScore")]
    pub team_a_score: i64,
    #[serde(rename = "teamBScore")]
    pub team_b_score: i64,
    pub team: String,
    #[serde(rename = "initialTeam")]
    pub initial_team: String,
    #[serde(rename = "totalKills")]
    pub total_kills: i64,
    #[serde(rename = "totalDeaths")]
    pub total_deaths: i64,
    #[serde(rename = "totalAssists")]
    pub total_assists: i64,
    #[serde(rename = "totalDamage")]
    pub total_damage: i64,
    #[serde(rename = "totalHeadshotKills")]
    pub total_headshot_kills: i64,
    pub score: i64,
    pub money: i64,
    #[serde(rename = "kastRounds")]
    pub kast_rounds: i64,
    #[serde(rename = "tradeKills")]
    pub trade_kills: i64,
    pub multikill2: i64,
    pub multikill3: i64,
    pub multikill4: i64,
    pub multikill5: i64,
    #[serde(rename = "clutchAttempts")]
    pub clutch_attempts: i64,
    #[serde(rename = "clutchesWon")]
    pub clutches_won: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssMapStat {
    pub map: String,
    pub matches: i64,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub damage: i64,
    pub headshots: i64,
    pub rounds: i64,
    #[serde(rename = "kastRounds")]
    pub kast_rounds: i64,
    #[serde(rename = "tradeKills")]
    pub trade_kills: i64,
    pub multikill2: i64,
    pub multikill3: i64,
    pub multikill4: i64,
    pub multikill5: i64,
    #[serde(rename = "clutchAttempts")]
    pub clutch_attempts: i64,
    #[serde(rename = "clutchesWon")]
    pub clutches_won: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssPlayerTotal {
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub damage: i64,
    pub headshots: i64,
    pub rounds: i64,
    #[serde(rename = "kastRounds")]
    pub kast_rounds: i64,
    #[serde(rename = "tradeKills")]
    pub trade_kills: i64,
    pub multikill2: i64,
    pub multikill3: i64,
    pub multikill4: i64,
    pub multikill5: i64,
    #[serde(rename = "clutchAttempts")]
    pub clutch_attempts: i64,
    #[serde(rename = "clutchesWon")]
    pub clutches_won: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssPlayerDetailResponse {
    #[serde(rename = "steamId")]
    pub steam_id: String,
    pub name: String,
    pub total: Cs2ssPlayerTotal,
    pub matches: Vec<Cs2ssPlayerMatchSummary>,
    #[serde(rename = "mapStats")]
    pub map_stats: Vec<Cs2ssMapStat>,
}

#[tauri::command]
pub fn get_cs2ss_overview(csgo: String) -> Result<Cs2ssOverviewResponse> {
    let conn = open_db(&csgo)?;

    let match_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM matches WHERE status = 'completed' AND mode_family = 'competitive'", [], |row| row.get(0))
        .unwrap_or(0);

    let mut stmt = conn.prepare(
        "SELECT mp.steam_id, mp.name,
            COUNT(DISTINCT mp.match_id) as matches,
            SUM(mp.total_kills) as kills, SUM(mp.total_deaths) as deaths,
            SUM(mp.total_assists) as assists, SUM(mp.total_damage) as damage,
            SUM(mp.total_headshot_kills) as headshots,
            SUM(m.rounds_played) as total_rounds,
            SUM(mp.kast_rounds) as kast_rounds, SUM(mp.trade_kills) as trade_kills,
            SUM(mp.multikill_2) as mk2, SUM(mp.multikill_3) as mk3,
            SUM(mp.multikill_4) as mk4, SUM(mp.multikill_5) as mk5,
            SUM(mp.clutch_attempts) as ca, SUM(mp.clutches_won) as cw
         FROM match_players mp
         JOIN matches m ON mp.match_id = m.match_id
         WHERE m.status = 'completed' AND m.mode_family = 'competitive'
         GROUP BY mp.steam_id, mp.name
         ORDER BY matches DESC"
    ).map_err(|e| AppError::invalid(format!("Query error: {e}")))?;

    let players: Vec<Cs2ssPlayerOverview> = stmt
        .query_map([], |row| {
            Ok(Cs2ssPlayerOverview {
                steam_id: row.get(0)?,
                name: row.get(1)?,
                matches: row.get(2)?,
                kills: row.get(3)?,
                deaths: row.get(4)?,
                assists: row.get(5)?,
                damage: row.get(6)?,
                headshots: row.get(7)?,
                total_rounds: row.get(8)?,
                kast_rounds: row.get(9)?,
                trade_kills: row.get(10)?,
                multikill2: row.get(11)?,
                multikill3: row.get(12)?,
                multikill4: row.get(13)?,
                multikill5: row.get(14)?,
                clutch_attempts: row.get(15)?,
                clutches_won: row.get(16)?,
            })
        })
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Cs2ssOverviewResponse {
        match_count,
        players,
    })
}

// ---- Deathmatch Overview ----

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssDmMapStat {
    pub map: String,
    pub sessions: i64,
    #[serde(rename = "avgKpm")]
    pub avg_kpm: f64,
    #[serde(rename = "avgDpm")]
    pub avg_dpm: f64,
    #[serde(rename = "avgKd")]
    pub avg_kd: f64,
    #[serde(rename = "maxStreak")]
    pub max_streak: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssDmSessionPoint {
    #[serde(rename = "matchId")]
    pub match_id: i64,
    pub map: String,
    pub ruleset: String,
    pub kills: i64,
    pub deaths: i64,
    pub damage: i64,
    pub score: i64,
    pub kpm: f64,
    pub dpm: f64,
    pub kd: f64,
    #[serde(rename = "headshotPct")]
    pub headshot_pct: f64,
    pub streak: i64,
    #[serde(rename = "durationSeconds")]
    pub duration_seconds: i64,
    #[serde(rename = "startedAt")]
    pub started_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssDmOverview {
    #[serde(rename = "sessionCount")]
    pub session_count: i64,
    #[serde(rename = "totalKills")]
    pub total_kills: i64,
    #[serde(rename = "totalDeaths")]
    pub total_deaths: i64,
    #[serde(rename = "totalDamage")]
    pub total_damage: i64,
    #[serde(rename = "totalHeadshots")]
    pub total_headshots: i64,
    #[serde(rename = "totalScore")]
    pub total_score: i64,
    #[serde(rename = "totalSpawns")]
    pub total_spawns: i64,
    #[serde(rename = "totalAliveSec")]
    pub total_alive_sec: i64,
    #[serde(rename = "totalSessionSec")]
    pub total_session_sec: i64,
    #[serde(rename = "maxStreak")]
    pub max_streak: i64,
    #[serde(rename = "maxLongestLife")]
    pub max_longest_life: i64,
    #[serde(rename = "totalBurst5_2")]
    pub total_burst5_2: i64,
    #[serde(rename = "totalBurst5_3")]
    pub total_burst5_3: i64,
    #[serde(rename = "totalBurst5_4")]
    pub total_burst5_4: i64,
    #[serde(rename = "totalBurst10_2")]
    pub total_burst10_2: i64,
    #[serde(rename = "totalBurst10_3")]
    pub total_burst10_3: i64,
    #[serde(rename = "totalBurst10_4")]
    pub total_burst10_4: i64,
    #[serde(rename = "perMap")]
    pub per_map: Vec<Cs2ssDmMapStat>,
    pub sessions: Vec<Cs2ssDmSessionPoint>,
}

#[tauri::command]
pub fn get_cs2ss_dm_overview(csgo: String, steam_id: String) -> Result<Cs2ssDmOverview> {
    let conn = open_db(&csgo)?;
    let cnt = conn.query_row(
        "SELECT COUNT(*), COALESCE(SUM(mp.total_kills),0), COALESCE(SUM(mp.total_deaths),0),
                COALESCE(SUM(mp.total_damage),0), COALESCE(SUM(mp.total_headshot_kills),0),
                COALESCE(SUM(mp.score),0), COALESCE(SUM(mp.dm_spawn_count),0),
                COALESCE(SUM(mp.dm_alive_seconds),0), COALESCE(SUM(m.duration_seconds),0),
                COALESCE(MAX(mp.dm_max_kill_streak),0), COALESCE(MAX(mp.dm_longest_life_seconds),0),
                COALESCE(SUM(mp.dm_burst_5s_2),0), COALESCE(SUM(mp.dm_burst_5s_3),0),
                COALESCE(SUM(mp.dm_burst_5s_4),0), COALESCE(SUM(mp.dm_burst_10s_2),0),
                COALESCE(SUM(mp.dm_burst_10s_3),0), COALESCE(SUM(mp.dm_burst_10s_4),0)
         FROM match_players mp JOIN matches m ON mp.match_id = m.match_id
         WHERE mp.steam_id = ?1 AND m.mode_family = 'deathmatch' AND m.status = 'completed'",
        [&steam_id],
        |row| Ok((row.get(0)?,row.get(1)?,row.get(2)?,row.get(3)?,row.get(4)?,row.get(5)?,
                 row.get(6)?,row.get(7)?,row.get(8)?,row.get(9)?,row.get(10)?,row.get(11)?,
                 row.get(12)?,row.get(13)?,row.get(14)?,row.get(15)?,row.get(16)?,))
    ).map_err(|e| AppError::invalid(format!("Cannot query CS2SS deathmatch totals: {e}")))?;

    let mut pm = conn.prepare(
        "SELECT m.map, COUNT(*), AVG(1.0*mp.total_kills/MAX(m.duration_seconds,1)*60),
                AVG(1.0*mp.total_damage/MAX(m.duration_seconds,1)*60),
                AVG(1.0*mp.total_kills/MAX(mp.total_deaths,1)), MAX(mp.dm_max_kill_streak)
         FROM match_players mp JOIN matches m ON mp.match_id = m.match_id
         WHERE mp.steam_id = ?1 AND m.mode_family = 'deathmatch' AND m.status = 'completed'
         GROUP BY m.map ORDER BY COUNT(*) DESC"
    ).map_err(|e| AppError::invalid(format!("Cannot prepare CS2SS deathmatch map query: {e}")))?;
    let per_map = pm.query_map([&steam_id], |row| Ok(Cs2ssDmMapStat {
        map: row.get(0)?, sessions: row.get(1)?, avg_kpm: row.get(2)?, avg_dpm: row.get(3)?,
        avg_kd: row.get(4)?, max_streak: row.get(5)?,
    }))
        .map_err(|e| AppError::invalid(format!("Cannot query CS2SS deathmatch maps: {e}")))?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| AppError::invalid(format!("Cannot read CS2SS deathmatch map row: {e}")))?;

    let mut ss = conn.prepare(
        "SELECT m.match_id, m.map, m.ruleset, mp.total_kills, mp.total_deaths,
                mp.total_damage, mp.score,
                1.0*mp.total_kills/MAX(m.duration_seconds,1)*60,
                1.0*mp.total_damage/MAX(m.duration_seconds,1)*60,
                1.0*mp.total_kills/MAX(mp.total_deaths,1),
                1.0*mp.total_headshot_kills/MAX(mp.total_kills,1)*100,
                mp.dm_max_kill_streak, m.duration_seconds, m.started_at
         FROM match_players mp JOIN matches m ON mp.match_id = m.match_id
         WHERE mp.steam_id = ?1 AND m.mode_family = 'deathmatch' AND m.status = 'completed'
         ORDER BY m.started_at DESC"
    ).map_err(|e| AppError::invalid(format!("Cannot prepare CS2SS deathmatch session query: {e}")))?;
    let sessions = ss.query_map([&steam_id], |row| Ok(Cs2ssDmSessionPoint {
        match_id: row.get(0)?, map: row.get(1)?, ruleset: row.get(2)?, kills: row.get(3)?,
        deaths: row.get(4)?, damage: row.get(5)?, score: row.get(6)?, kpm: row.get(7)?,
        dpm: row.get(8)?, kd: row.get(9)?, headshot_pct: row.get(10)?, streak: row.get(11)?,
        duration_seconds: row.get(12)?, started_at: row.get(13)?,
    }))
        .map_err(|e| AppError::invalid(format!("Cannot query CS2SS deathmatch sessions: {e}")))?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| AppError::invalid(format!("Cannot read CS2SS deathmatch session row: {e}")))?;

    Ok(Cs2ssDmOverview {
        session_count: cnt.0, total_kills: cnt.1, total_deaths: cnt.2,
        total_damage: cnt.3, total_headshots: cnt.4, total_score: cnt.5,
        total_spawns: cnt.6, total_alive_sec: cnt.7, total_session_sec: cnt.8,
        max_streak: cnt.9, max_longest_life: cnt.10,
        total_burst5_2: cnt.11, total_burst5_3: cnt.12, total_burst5_4: cnt.13,
        total_burst10_2: cnt.14, total_burst10_3: cnt.15, total_burst10_4: cnt.16,
        per_map, sessions,
    })
}

#[tauri::command]
pub fn list_cs2ss_matches(csgo: String) -> Result<Vec<Cs2ssMatchSummary>> {
    let conn = open_db(&csgo)?;
    let mut stmt = conn
        .prepare(
            "SELECT match_id, map, started_at, ended_at, end_reason, rounds_played,
                    ct_score, t_score, team_a_score, team_b_score,
                    mode_family, ruleset, game_type, game_mode, duration_seconds, status
             FROM matches
             ORDER BY started_at DESC"
        )
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?;

    let matches: Vec<Cs2ssMatchSummary> = stmt
        .query_map([], |row| {
            Ok(Cs2ssMatchSummary {
                match_id: row.get(0)?,
                map: row.get(1)?,
                started_at: row.get(2)?,
                ended_at: row.get(3)?,
                end_reason: row.get(4)?,
                rounds_played: row.get(5)?,
                ct_score: row.get(6)?,
                t_score: row.get(7)?,
                team_a_score: row.get(8)?,
                team_b_score: row.get(9)?,
                mode_family: row.get(10)?,
                ruleset: row.get(11)?,
                game_type: row.get(12)?,
                game_mode: row.get(13)?,
                duration_seconds: row.get(14)?,
                status: row.get(15)?,
            })
        })
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(matches)
}

#[tauri::command]
pub fn get_cs2ss_match_detail(csgo: String, match_id: i64) -> Result<Cs2ssMatchDetailResponse> {
    let conn = open_db(&csgo)?;

    let m: Cs2ssMatchSummary = conn
        .query_row(
            "SELECT match_id, map, started_at, ended_at, end_reason, rounds_played,
                    ct_score, t_score, team_a_score, team_b_score,
                    mode_family, ruleset, game_type, game_mode, duration_seconds, status
             FROM matches WHERE match_id = ?1",
            [match_id],
            |row| {
                Ok(Cs2ssMatchSummary {
                    match_id: row.get(0)?,
                    map: row.get(1)?,
                    started_at: row.get(2)?,
                    ended_at: row.get(3)?,
                    end_reason: row.get(4)?,
                    rounds_played: row.get(5)?,
                    ct_score: row.get(6)?,
                    t_score: row.get(7)?,
                    team_a_score: row.get(8)?,
                    team_b_score: row.get(9)?,
                    mode_family: row.get(10)?,
                    ruleset: row.get(11)?,
                    game_type: row.get(12)?,
                    game_mode: row.get(13)?,
                    duration_seconds: row.get(14)?,
                    status: row.get(15)?,
                })
            },
        )
        .map_err(|e| AppError::invalid(format!("Match {match_id} not found: {e}")))?;

    let mut rs = conn
        .prepare(
            "SELECT round_id, match_id, round_number, captured_at, source,
                    winner_team, end_reason, ct_score, t_score, team_a_score, team_b_score
             FROM rounds WHERE match_id = ?1 ORDER BY round_number"
        )
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?;

    let rounds: Vec<Cs2ssRoundSummary> = rs
        .query_map([match_id], |row| {
            Ok(Cs2ssRoundSummary {
                round_id: row.get(0)?,
                match_id: row.get(1)?,
                round_number: row.get(2)?,
                captured_at: row.get(3)?,
                source: row.get(4)?,
                winner_team: row.get(5)?,
                end_reason: row.get(6)?,
                ct_score: row.get(7)?,
                t_score: row.get(8)?,
                team_a_score: row.get(9)?,
                team_b_score: row.get(10)?,
            })
        })
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?
        .filter_map(|r| r.ok())
        .collect();

    let mut rps = conn
        .prepare(
            "SELECT rp.round_player_id, rp.round_id, rp.match_id, rp.steam_id, rp.name, rp.team, rp.is_bot,
                    rp.alive, rp.health, rp.kills, rp.deaths, rp.assists, rp.damage, rp.headshot_kills,
                    rp.total_kills, rp.total_deaths, rp.total_damage, rp.score, rp.money,
                    rp.kast, rp.survived, rp.traded, rp.trade_kills, rp.event_kills, rp.multikill,
                    rp.clutch_attempt, rp.clutch_won, rp.clutch_size, r.round_number
             FROM round_players rp JOIN rounds r ON rp.round_id = r.round_id
             WHERE rp.match_id = ?1 ORDER BY r.round_number"
        )
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?;

    let round_players: Vec<Cs2ssRoundPlayer> = rps
        .query_map([match_id], |row| {
            Ok(Cs2ssRoundPlayer {
                round_player_id: row.get(0)?,
                round_id: row.get(1)?,
                match_id: row.get(2)?,
                steam_id: row.get(3)?,
                name: row.get(4)?,
                team: row.get(5)?,
                is_bot: row.get(6)?,
                alive: row.get(7)?,
                health: row.get(8)?,
                kills: row.get(9)?,
                deaths: row.get(10)?,
                assists: row.get(11)?,
                damage: row.get(12)?,
                headshot_kills: row.get(13)?,
                total_kills: row.get(14)?,
                total_deaths: row.get(15)?,
                total_damage: row.get(16)?,
                score: row.get(17)?,
                money: row.get(18)?,
                kast: row.get(19)?,
                survived: row.get(20)?,
                traded: row.get(21)?,
                trade_kills: row.get(22)?,
                event_kills: row.get(23)?,
                multikill: row.get(24)?,
                clutch_attempt: row.get(25)?,
                clutch_won: row.get(26)?,
                clutch_size: row.get(27)?,
                round_number: row.get(28)?,
            })
        })
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?
        .filter_map(|r| r.ok())
        .collect();

    let mut mps = conn
        .prepare(
            "SELECT match_player_id, match_id, steam_id, name, team, is_bot, alive, health,
                    total_kills, total_deaths, total_assists, total_damage, total_headshot_kills,
                    score, money, kast_rounds, trade_kills,
                    multikill_2, multikill_3, multikill_4, multikill_5,
                    clutch_attempts, clutches_won,
                    dm_spawn_count, dm_completed_lives, dm_max_kill_streak,
                    dm_alive_seconds, dm_longest_life_seconds,
                    dm_burst_5s_2, dm_burst_5s_3, dm_burst_5s_4,
                    dm_burst_10s_2, dm_burst_10s_3, dm_burst_10s_4
             FROM match_players WHERE match_id = ?1"
        )
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?;

    let match_players: Vec<Cs2ssMatchPlayer> = mps
        .query_map([match_id], |row| {
            Ok(Cs2ssMatchPlayer {
                match_player_id: row.get(0)?,
                match_id: row.get(1)?,
                steam_id: row.get(2)?,
                name: row.get(3)?,
                team: row.get(4)?,
                is_bot: row.get(5)?,
                alive: row.get(6)?,
                health: row.get(7)?,
                total_kills: row.get(8)?,
                total_deaths: row.get(9)?,
                total_assists: row.get(10)?,
                total_damage: row.get(11)?,
                total_headshot_kills: row.get(12)?,
                score: row.get(13)?,
                money: row.get(14)?,
                kast_rounds: row.get(15)?,
                trade_kills: row.get(16)?,
                multikill2: row.get(17)?,
                multikill3: row.get(18)?,
                multikill4: row.get(19)?,
                multikill5: row.get(20)?,
                clutch_attempts: row.get(21)?,
                clutches_won: row.get(22)?,
                dm_spawn_count: row.get(23)?,
                dm_completed_lives: row.get(24)?,
                dm_max_kill_streak: row.get(25)?,
                dm_alive_seconds: row.get(26)?,
                dm_longest_life_seconds: row.get(27)?,
                dm_burst_5s_2: row.get(28)?,
                dm_burst_5s_3: row.get(29)?,
                dm_burst_5s_4: row.get(30)?,
                dm_burst_10s_2: row.get(31)?,
                dm_burst_10s_3: row.get(32)?,
                dm_burst_10s_4: row.get(33)?,
            })
        })
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?
        .filter_map(|r| r.ok())
        .collect();

    let mut dls = conn
        .prepare(
            "SELECT life_id, match_id, steam_id, life_index, spawned_at, ended_at,
                    end_kind, duration_seconds, kills, damage
             FROM deathmatch_lives WHERE match_id = ?1 ORDER BY life_index"
        )
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?;

    let deathmatch_lives: Vec<Cs2ssDeathmatchLife> = dls
        .query_map([match_id], |row| {
            Ok(Cs2ssDeathmatchLife {
                life_id: row.get(0)?,
                match_id: row.get(1)?,
                steam_id: row.get(2)?,
                life_index: row.get(3)?,
                spawned_at: row.get(4)?,
                ended_at: row.get(5)?,
                end_kind: row.get(6)?,
                duration_seconds: row.get(7)?,
                kills: row.get(8)?,
                damage: row.get(9)?,
            })
        })
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Cs2ssMatchDetailResponse {
        r#match: m,
        rounds,
        round_players,
        match_players,
        deathmatch_lives,
    })
}

#[tauri::command]
pub fn get_cs2ss_player_detail(csgo: String, steam_id: String) -> Result<Cs2ssPlayerDetailResponse> {
    let conn = open_db(&csgo)?;

    let (name, total): (String, Cs2ssPlayerTotal) = conn
        .query_row(
            "SELECT mp.name,
                    SUM(mp.total_kills) as kills, SUM(mp.total_deaths) as deaths,
                    SUM(mp.total_assists) as assists, SUM(mp.total_damage) as damage,
                    SUM(mp.total_headshot_kills) as hs,
                    SUM(m.rounds_played) as rounds,
                    SUM(mp.kast_rounds) as kast_r, SUM(mp.trade_kills) as tk,
                    SUM(mp.multikill_2) as mk2, SUM(mp.multikill_3) as mk3,
                    SUM(mp.multikill_4) as mk4, SUM(mp.multikill_5) as mk5,
                    SUM(mp.clutch_attempts) as ca, SUM(mp.clutches_won) as cw
             FROM match_players mp
             JOIN matches m ON mp.match_id = m.match_id
             WHERE mp.steam_id = ?1 AND m.mode_family = 'competitive'
             GROUP BY mp.name",
            [&steam_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    Cs2ssPlayerTotal {
                        kills: row.get(1)?,
                        deaths: row.get(2)?,
                        assists: row.get(3)?,
                        damage: row.get(4)?,
                        headshots: row.get(5)?,
                        rounds: row.get(6)?,
                        kast_rounds: row.get(7)?,
                        trade_kills: row.get(8)?,
                        multikill2: row.get(9)?,
                        multikill3: row.get(10)?,
                        multikill4: row.get(11)?,
                        multikill5: row.get(12)?,
                        clutch_attempts: row.get(13)?,
                        clutches_won: row.get(14)?,
                    },
                ))
            },
        )
        .map_err(|e| AppError::invalid(format!("Player {steam_id} not found: {e}")))?;

    let mut ms = conn
        .prepare(
            "SELECT mp.match_id, m.map, m.started_at, m.rounds_played,
                    m.ct_score, m.t_score, m.team_a_score, m.team_b_score, mp.team,
                    COALESCE((SELECT rp0.team FROM round_players rp0 WHERE rp0.steam_id = mp.steam_id AND rp0.match_id = m.match_id ORDER BY rp0.round_player_id LIMIT 1), mp.team, '') as initial_team,
                    mp.total_kills, mp.total_deaths, mp.total_assists,
                    mp.total_damage, mp.total_headshot_kills, mp.score, mp.money,
                    mp.kast_rounds, mp.trade_kills,
                    mp.multikill_2, mp.multikill_3, mp.multikill_4, mp.multikill_5,
                    mp.clutch_attempts, mp.clutches_won
             FROM match_players mp
             JOIN matches m ON mp.match_id = m.match_id
             WHERE mp.steam_id = ?1 AND m.status = 'completed' AND m.mode_family = 'competitive'
             ORDER BY m.started_at DESC"
        )
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?;

    let player_matches: Vec<Cs2ssPlayerMatchSummary> = ms
        .query_map([&steam_id], |row| {
            Ok(Cs2ssPlayerMatchSummary {
                match_id: row.get(0)?,
                map: row.get(1)?,
                started_at: row.get(2)?,
                rounds_played: row.get(3)?,
                ct_score: row.get(4)?,
                t_score: row.get(5)?,
                team_a_score: row.get(6)?,
                team_b_score: row.get(7)?,
                team: row.get(8)?,
                initial_team: row.get(9)?,
                total_kills: row.get(10)?,
                total_deaths: row.get(11)?,
                total_assists: row.get(12)?,
                total_damage: row.get(13)?,
                total_headshot_kills: row.get(14)?,
                score: row.get(15)?,
                money: row.get(16)?,
                kast_rounds: row.get(17)?,
                trade_kills: row.get(18)?,
                multikill2: row.get(19)?,
                multikill3: row.get(20)?,
                multikill4: row.get(21)?,
                multikill5: row.get(22)?,
                clutch_attempts: row.get(23)?,
                clutches_won: row.get(24)?,
            })
        })
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?
        .filter_map(|r| r.ok())
        .collect();

    let mut maps = conn
        .prepare(
            "SELECT m.map, COUNT(DISTINCT m.match_id) as matches,
                    SUM(mp.total_kills), SUM(mp.total_deaths), SUM(mp.total_assists),
                    SUM(mp.total_damage), SUM(mp.total_headshot_kills),
                    SUM(m.rounds_played) as rounds,
                    SUM(mp.kast_rounds), SUM(mp.trade_kills),
                    SUM(mp.multikill_2), SUM(mp.multikill_3), SUM(mp.multikill_4), SUM(mp.multikill_5),
                    SUM(mp.clutch_attempts), SUM(mp.clutches_won)
             FROM match_players mp
             JOIN matches m ON mp.match_id = m.match_id
             WHERE mp.steam_id = ?1 AND m.status = 'completed' AND m.mode_family = 'competitive'
             GROUP BY m.map
             ORDER BY matches DESC"
        )
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?;

    let map_stats: Vec<Cs2ssMapStat> = maps
        .query_map([&steam_id], |row| {
            Ok(Cs2ssMapStat {
                map: row.get(0)?,
                matches: row.get(1)?,
                kills: row.get(2)?,
                deaths: row.get(3)?,
                assists: row.get(4)?,
                damage: row.get(5)?,
                headshots: row.get(6)?,
                rounds: row.get(7)?,
                kast_rounds: row.get(8)?,
                trade_kills: row.get(9)?,
                multikill2: row.get(10)?,
                multikill3: row.get(11)?,
                multikill4: row.get(12)?,
                multikill5: row.get(13)?,
                clutch_attempts: row.get(14)?,
                clutches_won: row.get(15)?,
            })
        })
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Cs2ssPlayerDetailResponse {
        steam_id: steam_id.clone(),
        name,
        total,
        matches: player_matches,
        map_stats,
    })
}

// ---- Match list with player stats (single query, no N+1) ----

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cs2ssMatchWithStats {
    #[serde(flatten)]
    pub match_summary: Cs2ssMatchSummary,
    #[serde(rename = "playerTeam")]
    pub player_team: String,
    #[serde(rename = "playerInitialTeam")]
    pub player_initial_team: String,
    #[serde(rename = "playerKills")]
    pub player_kills: i64,
    #[serde(rename = "playerDeaths")]
    pub player_deaths: i64,
    #[serde(rename = "playerAssists")]
    pub player_assists: i64,
    #[serde(rename = "playerDamage")]
    pub player_damage: i64,
    #[serde(rename = "playerHeadshots")]
    pub player_headshots: i64,
    #[serde(rename = "playerScore")]
    pub player_score: i64,
    #[serde(rename = "playerKastRounds")]
    pub player_kast_rounds: i64,
    #[serde(rename = "playerTradeKills")]
    pub player_trade_kills: i64,
    #[serde(rename = "playerMk2")]
    pub player_mk2: i64,
    #[serde(rename = "playerMk3")]
    pub player_mk3: i64,
    #[serde(rename = "playerMk4")]
    pub player_mk4: i64,
    #[serde(rename = "playerMk5")]
    pub player_mk5: i64,
    #[serde(rename = "playerClutchAttempts")]
    pub player_clutch_attempts: i64,
    #[serde(rename = "playerClutchesWon")]
    pub player_clutches_won: i64,
    #[serde(rename = "playerDmSpawnCount")]
    pub player_dm_spawn_count: i64,
    #[serde(rename = "playerDmMaxKillStreak")]
    pub player_dm_max_kill_streak: i64,
}

#[tauri::command]
pub fn list_cs2ss_matches_with_stats(csgo: String) -> Result<Vec<Cs2ssMatchWithStats>> {
    let conn = open_db(&csgo)?;
    let mut stmt = conn
        .prepare(
            "SELECT m.match_id, m.map, m.started_at, m.ended_at, m.end_reason,
                    m.rounds_played, m.ct_score, m.t_score, m.team_a_score, m.team_b_score,
                    m.mode_family, m.ruleset, m.game_type, m.game_mode, m.duration_seconds, m.status,
                    COALESCE(mp.team, '') as player_team,
                    COALESCE((SELECT rp0.team FROM round_players rp0 WHERE rp0.steam_id = mp.steam_id AND rp0.match_id = m.match_id ORDER BY rp0.round_player_id LIMIT 1), mp.team, '') as player_initial_team,
                    COALESCE(mp.total_kills, 0), COALESCE(mp.total_deaths, 0),
                    COALESCE(mp.total_assists, 0), COALESCE(mp.total_damage, 0),
                    COALESCE(mp.total_headshot_kills, 0), COALESCE(mp.score, 0),
                    COALESCE(mp.kast_rounds, 0), COALESCE(mp.trade_kills, 0),
                    COALESCE(mp.multikill_2, 0), COALESCE(mp.multikill_3, 0),
                    COALESCE(mp.multikill_4, 0), COALESCE(mp.multikill_5, 0),
                    COALESCE(mp.clutch_attempts, 0), COALESCE(mp.clutches_won, 0),
                    COALESCE(mp.dm_spawn_count, 0), COALESCE(mp.dm_max_kill_streak, 0)
             FROM matches m
             LEFT JOIN match_players mp ON m.match_id = mp.match_id AND mp.is_bot = 0
             WHERE m.status = 'completed'
             ORDER BY m.started_at DESC"
        )
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?;

    let list: Vec<Cs2ssMatchWithStats> = stmt
        .query_map([], |row| {
            Ok(Cs2ssMatchWithStats {
                match_summary: Cs2ssMatchSummary {
                    match_id: row.get(0)?, map: row.get(1)?, started_at: row.get(2)?,
                    ended_at: row.get(3)?, end_reason: row.get(4)?, rounds_played: row.get(5)?,
                    ct_score: row.get(6)?, t_score: row.get(7)?, team_a_score: row.get(8)?,
                    team_b_score: row.get(9)?, mode_family: row.get(10)?, ruleset: row.get(11)?,
                    game_type: row.get(12)?, game_mode: row.get(13)?, duration_seconds: row.get(14)?,
                    status: row.get(15)?,
                },
                player_team: row.get(16)?,
                player_initial_team: row.get(17)?,
                player_kills: row.get(18)?, player_deaths: row.get(19)?,
                player_assists: row.get(20)?, player_damage: row.get(21)?,
                player_headshots: row.get(22)?, player_score: row.get(23)?,
                player_kast_rounds: row.get(24)?, player_trade_kills: row.get(25)?,
                player_mk2: row.get(26)?, player_mk3: row.get(27)?,
                player_mk4: row.get(28)?, player_mk5: row.get(29)?,
                player_clutch_attempts: row.get(30)?,
                player_clutches_won: row.get(31)?,
                player_dm_spawn_count: row.get(32)?,
                player_dm_max_kill_streak: row.get(33)?,
            })
        })
        .map_err(|e| AppError::invalid(format!("Query error: {e}")))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(list)
}

// ---- Steam ID config ----

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Cs2ssConfig {
    #[serde(rename = "steamId", default)]
    pub steam_id: String,
}

fn is_valid_steam_id64(value: &str) -> bool {
    value.len() == 17
        && value.starts_with("7656119")
        && value.bytes().all(|byte| byte.is_ascii_digit())
}

#[tauri::command]
pub fn get_cs2ss_config(csgo: String) -> Result<Cs2ssConfig> {
    let path = cs2ss_config_path(&csgo);
    if path.exists() {
        let bytes = std::fs::read_to_string(&path)
            .map_err(|e| AppError::invalid(format!("Cannot read CS2SS config: {e}")))?;
        serde_json::from_str::<Cs2ssConfig>(&bytes)
            .map_err(|e| AppError::invalid(format!("Cannot parse CS2SS config: {e}")))
    } else {
        Ok(Cs2ssConfig::default())
    }
}

#[tauri::command]
pub fn save_cs2ss_config(csgo: String, mut config: Cs2ssConfig) -> Result<()> {
    config.steam_id = config.steam_id.trim().to_string();
    if !is_valid_steam_id64(&config.steam_id) {
        return Err(AppError::invalid(
            "SteamID64 must be 17 digits and start with 7656119".to_string(),
        ));
    }
    let path = cs2ss_config_path(&csgo);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AppError::invalid(format!("Cannot create CS2SS config directory: {e}")))?;
    }
    write_json_atomic(&path, &config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_root() -> std::path::PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("cs2ss-test-{suffix}"))
    }

    fn setup_test_db(root: &std::path::Path) {
        let db_dir = root.join(".csbip").join("cs2ss");
        std::fs::create_dir_all(&db_dir).unwrap();
        let db_path = db_dir.join("telemetry.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch(
            "CREATE TABLE matches (
                match_id INTEGER PRIMARY KEY AUTOINCREMENT,
                map TEXT NOT NULL,
                started_at TEXT NOT NULL,
                ended_at TEXT,
                end_reason TEXT,
                rounds_played INTEGER,
                ct_score INTEGER NOT NULL DEFAULT 0,
                t_score INTEGER NOT NULL DEFAULT 0,
                team_a_score INTEGER NOT NULL DEFAULT 0,
                team_b_score INTEGER NOT NULL DEFAULT 0,
                mode_family TEXT NOT NULL DEFAULT 'competitive',
                ruleset TEXT NOT NULL DEFAULT 'round_based',
                game_type INTEGER NOT NULL DEFAULT 0,
                game_mode INTEGER NOT NULL DEFAULT 1,
                duration_seconds INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'in_progress'
            );
            CREATE TABLE match_players (
                match_player_id INTEGER PRIMARY KEY AUTOINCREMENT,
                match_id INTEGER NOT NULL REFERENCES matches(match_id),
                steam_id TEXT NOT NULL,
                name TEXT NOT NULL,
                team TEXT NOT NULL,
                is_bot INTEGER NOT NULL,
                alive INTEGER NOT NULL,
                health INTEGER NOT NULL,
                total_kills INTEGER NOT NULL,
                total_deaths INTEGER NOT NULL,
                total_assists INTEGER NOT NULL,
                total_damage INTEGER NOT NULL,
                total_headshot_kills INTEGER NOT NULL,
                score INTEGER NOT NULL,
                money INTEGER NOT NULL,
                kast_rounds INTEGER NOT NULL DEFAULT 0,
                trade_kills INTEGER NOT NULL DEFAULT 0,
                multikill_2 INTEGER NOT NULL DEFAULT 0,
                multikill_3 INTEGER NOT NULL DEFAULT 0,
                multikill_4 INTEGER NOT NULL DEFAULT 0,
                multikill_5 INTEGER NOT NULL DEFAULT 0,
                clutch_attempts INTEGER NOT NULL DEFAULT 0,
                clutches_won INTEGER NOT NULL DEFAULT 0,
                dm_spawn_count INTEGER NOT NULL DEFAULT 0,
                dm_completed_lives INTEGER NOT NULL DEFAULT 0,
                dm_max_kill_streak INTEGER NOT NULL DEFAULT 0,
                dm_alive_seconds INTEGER NOT NULL DEFAULT 0,
                dm_longest_life_seconds INTEGER NOT NULL DEFAULT 0,
                dm_burst_5s_2 INTEGER NOT NULL DEFAULT 0,
                dm_burst_5s_3 INTEGER NOT NULL DEFAULT 0,
                dm_burst_5s_4 INTEGER NOT NULL DEFAULT 0,
                dm_burst_10s_2 INTEGER NOT NULL DEFAULT 0,
                dm_burst_10s_3 INTEGER NOT NULL DEFAULT 0,
                dm_burst_10s_4 INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE rounds (
                round_id INTEGER PRIMARY KEY AUTOINCREMENT,
                match_id INTEGER NOT NULL REFERENCES matches(match_id),
                round_number INTEGER NOT NULL,
                captured_at TEXT NOT NULL,
                source TEXT NOT NULL,
                winner_team TEXT,
                end_reason INTEGER,
                ct_score INTEGER NOT NULL,
                t_score INTEGER NOT NULL,
                team_a_score INTEGER NOT NULL DEFAULT 0,
                team_b_score INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE round_players (
                round_player_id INTEGER PRIMARY KEY AUTOINCREMENT,
                round_id INTEGER NOT NULL,
                match_id INTEGER NOT NULL,
                steam_id TEXT NOT NULL,
                name TEXT NOT NULL,
                team TEXT NOT NULL,
                is_bot INTEGER NOT NULL,
                alive INTEGER NOT NULL,
                health INTEGER NOT NULL,
                kills INTEGER NOT NULL,
                deaths INTEGER NOT NULL,
                assists INTEGER NOT NULL,
                damage INTEGER NOT NULL,
                headshot_kills INTEGER NOT NULL,
                total_kills INTEGER NOT NULL,
                total_deaths INTEGER NOT NULL,
                total_damage INTEGER NOT NULL,
                score INTEGER NOT NULL,
                money INTEGER NOT NULL,
                kast INTEGER NOT NULL DEFAULT 0,
                survived INTEGER NOT NULL DEFAULT 0,
                traded INTEGER NOT NULL DEFAULT 0,
                trade_kills INTEGER NOT NULL DEFAULT 0,
                event_kills INTEGER NOT NULL DEFAULT 0,
                multikill INTEGER NOT NULL DEFAULT 0,
                clutch_attempt INTEGER NOT NULL DEFAULT 0,
                clutch_won INTEGER NOT NULL DEFAULT 0,
                clutch_size INTEGER NOT NULL DEFAULT 0,
                round_number INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE deathmatch_lives (
                life_id INTEGER PRIMARY KEY AUTOINCREMENT,
                match_id INTEGER NOT NULL,
                steam_id TEXT NOT NULL,
                life_index INTEGER NOT NULL,
                spawned_at TEXT NOT NULL,
                ended_at TEXT NOT NULL,
                end_kind TEXT NOT NULL,
                duration_seconds REAL NOT NULL,
                kills INTEGER NOT NULL,
                damage INTEGER NOT NULL
            );"
        ).unwrap();
    }

    fn insert_completed_match(conn: &rusqlite::Connection) -> i64 {
        conn.execute(
            "INSERT INTO matches (map, started_at, ended_at, status, rounds_played, ct_score, t_score, mode_family, ruleset, game_type, game_mode, duration_seconds)
             VALUES ('de_dust2', '2025-01-01T00:00:00Z', '2025-01-01T00:30:00Z', 'completed', 24, 13, 11, 'competitive', 'round_based', 0, 1, 1800)",
            [],
        ).unwrap();
        conn.last_insert_rowid()
    }

    fn insert_player(conn: &rusqlite::Connection, match_id: i64, steam_id: &str, name: &str) {
        conn.execute(
            "INSERT INTO match_players (match_id, steam_id, name, team, is_bot, alive, health, total_kills, total_deaths, total_assists, total_damage, total_headshot_kills, score, money)
             VALUES (?1, ?2, ?3, 'CT', 0, 1, 100, 20, 15, 5, 1800, 8, 45, 16000)",
            rusqlite::params![match_id, steam_id, name],
        ).unwrap();
    }

    #[test]
    fn list_with_stats_filters_out_in_progress() {
        let root = test_root();
        setup_test_db(&root);
        let csgo = root.to_str().unwrap().to_string();

        let conn = open_db(&csgo).unwrap();
        conn.execute(
            "INSERT INTO matches (map, started_at, status, rounds_played, ct_score, t_score, mode_family, ruleset, game_type, game_mode) VALUES ('de_inferno', '2025-02-01T00:00:00Z', 'in_progress', 0, 0, 0, 'competitive', 'round_based', 0, 1)",
            [],
        ).unwrap();

        let result = list_cs2ss_matches_with_stats(csgo).unwrap();
        assert!(result.is_empty(), "in_progress matches should be excluded");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn list_with_stats_includes_completed() {
        let root = test_root();
        setup_test_db(&root);
        let csgo = root.to_str().unwrap().to_string();

        let conn = open_db(&csgo).unwrap();
        let match_id = insert_completed_match(&conn);
        insert_player(&conn, match_id, "76561198000000001", "Player1");

        let result = list_cs2ss_matches_with_stats(csgo.clone()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].match_summary.map, "de_dust2");
        assert_eq!(result[0].match_summary.status, "completed");
        assert_eq!(result[0].player_kills, 20);

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn list_with_stats_handles_empty_match_players() {
        let root = test_root();
        setup_test_db(&root);
        let csgo = root.to_str().unwrap().to_string();

        let conn = open_db(&csgo).unwrap();
        conn.execute(
            "INSERT INTO matches (map, started_at, ended_at, status, rounds_played, ct_score, t_score, mode_family, ruleset, game_type, game_mode, duration_seconds) VALUES ('de_nuke', '2025-03-01T00:00:00Z', '2025-03-01T00:30:00Z', 'completed', 30, 16, 14, 'competitive', 'round_based', 0, 1, 2000)",
            [],
        ).unwrap();

        let result = list_cs2ss_matches_with_stats(csgo.clone()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].player_kills, 0);
        assert_eq!(result[0].player_team, "");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn list_with_stats_handles_abandoned_cleanup() {
        let root = test_root();
        setup_test_db(&root);
        let csgo = root.to_str().unwrap().to_string();

        let conn = open_db(&csgo).unwrap();
        conn.execute(
            "INSERT INTO matches (map, started_at, status, rounds_played, ct_score, t_score, mode_family, ruleset, game_type, game_mode) VALUES ('de_ancient', '2025-04-01T00:00:00Z', 'abandoned', 0, 0, 0, 'competitive', 'round_based', 0, 1)",
            [],
        ).unwrap();

        let result = list_cs2ss_matches_with_stats(csgo).unwrap();
        assert!(result.is_empty(), "abandoned matches should be excluded");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn match_detail_handles_in_progress() {
        let root = test_root();
        setup_test_db(&root);
        let csgo = root.to_str().unwrap().to_string();

        let conn = open_db(&csgo).unwrap();
        conn.execute(
            "INSERT INTO matches (map, started_at, status, rounds_played, ct_score, t_score, mode_family, ruleset, game_type, game_mode) VALUES ('de_mirage', '2025-05-01T00:00:00Z', 'in_progress', 0, 0, 0, 'competitive', 'round_based', 0, 1)",
            [],
        ).unwrap();
        let match_id = conn.last_insert_rowid();

        let detail = get_cs2ss_match_detail(csgo, match_id).unwrap();
        assert_eq!(detail.r#match.match_id, match_id);
        assert_eq!(detail.r#match.status, "in_progress");
        assert!(detail.match_players.is_empty());
        assert!(detail.rounds.is_empty());

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn dm_overview_returns_data_without_panicking() {
        let root = test_root();
        setup_test_db(&root);
        let csgo = root.to_str().unwrap().to_string();
        let conn = open_db(&csgo).unwrap();
        conn.execute(
            "INSERT INTO matches (map, started_at, ended_at, status, rounds_played, ct_score, t_score, mode_family, ruleset, game_type, game_mode, duration_seconds)
             VALUES ('de_dust2', '2025-06-01T00:00:00Z', '2025-06-01T00:10:00Z', 'completed', 0, 0, 0, 'deathmatch', 'ffa', 1, 2, 600)",
            [],
        ).unwrap();
        let match_id = conn.last_insert_rowid();
        conn.execute(
            "INSERT INTO match_players (
                match_id, steam_id, name, team, is_bot, alive, health,
                total_kills, total_deaths, total_assists, total_damage,
                total_headshot_kills, score, money, dm_spawn_count,
                dm_max_kill_streak, dm_alive_seconds, dm_longest_life_seconds
             ) VALUES (?1, '76561198000000001', 'Player1', 'CT', 0, 1, 100,
                42, 20, 0, 5600, 21, 1200, 0, 21, 8, 480, 44)",
            [match_id],
        ).unwrap();

        let result = get_cs2ss_dm_overview(csgo, "76561198000000001".to_string()).unwrap();
        assert_eq!(result.session_count, 1);
        assert_eq!(result.total_kills, 42);
        assert_eq!(result.sessions.len(), 1);
        assert_eq!(result.per_map.len(), 1);

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn dm_overview_returns_error_for_legacy_schema() {
        let root = test_root();
        let db_dir = root.join(".csbip").join("cs2ss");
        std::fs::create_dir_all(&db_dir).unwrap();
        let conn = rusqlite::Connection::open(db_dir.join("telemetry.db")).unwrap();
        conn.execute_batch(
            "CREATE TABLE matches (
                match_id INTEGER PRIMARY KEY,
                map TEXT NOT NULL,
                started_at TEXT NOT NULL,
                mode_family TEXT NOT NULL,
                status TEXT NOT NULL,
                duration_seconds INTEGER NOT NULL
            );
            CREATE TABLE match_players (
                match_id INTEGER NOT NULL,
                steam_id TEXT NOT NULL,
                total_kills INTEGER NOT NULL,
                total_deaths INTEGER NOT NULL,
                total_damage INTEGER NOT NULL,
                total_headshot_kills INTEGER NOT NULL,
                score INTEGER NOT NULL
            );"
        ).unwrap();
        drop(conn);

        let result = get_cs2ss_dm_overview(
            root.to_str().unwrap().to_string(),
            "76561198000000001".to_string(),
        );
        assert!(result.is_err(), "legacy schemas must return an AppError instead of panicking");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn steam_id_config_is_validated_and_roundtrips() {
        assert!(is_valid_steam_id64("76561198000000001"));
        assert!(!is_valid_steam_id64("player-name"));
        assert!(!is_valid_steam_id64("7656119800000000"));

        let root = test_root();
        let csgo = root.to_str().unwrap().to_string();
        let invalid = save_cs2ss_config(
            csgo.clone(),
            Cs2ssConfig { steam_id: "player-name".to_string() },
        );
        assert!(invalid.is_err());

        save_cs2ss_config(
            csgo.clone(),
            Cs2ssConfig { steam_id: " 76561198000000001 ".to_string() },
        ).unwrap();
        assert_eq!(get_cs2ss_config(csgo).unwrap().steam_id, "76561198000000001");

        std::fs::remove_dir_all(&root).ok();
    }
}

#[tauri::command]
pub fn delete_cs2ss_matches(csgo: String, match_ids: Vec<i64>) -> Result<usize> {
    if match_ids.is_empty() {
        return Err(AppError::invalid("No match IDs provided for deletion"));
    }
    let conn = open_db(&csgo)?;

    let placeholders: Vec<String> = match_ids.iter().enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect();
    let in_clause = placeholders.join(",");

    let tx = conn.unchecked_transaction()
        .map_err(|e| AppError::invalid(format!("Cannot begin transaction: {e}")))?;

    tx.execute_batch("PRAGMA foreign_keys = ON")
        .map_err(|e| AppError::invalid(format!("Cannot enable foreign keys: {e}")))?;

    tx.execute(
        &format!("DELETE FROM deathmatch_lives WHERE match_id IN ({in_clause})"),
        rusqlite::params_from_iter(match_ids.iter()),
    ).map_err(|e| AppError::invalid(format!("Delete deathmatch_lives failed: {e}")))?;

    tx.execute(
        &format!("DELETE FROM round_players WHERE match_id IN ({in_clause})"),
        rusqlite::params_from_iter(match_ids.iter()),
    ).map_err(|e| AppError::invalid(format!("Delete round_players failed: {e}")))?;

    tx.execute(
        &format!("DELETE FROM rounds WHERE match_id IN ({in_clause})"),
        rusqlite::params_from_iter(match_ids.iter()),
    ).map_err(|e| AppError::invalid(format!("Delete rounds failed: {e}")))?;

    tx.execute(
        &format!("DELETE FROM match_players WHERE match_id IN ({in_clause})"),
        rusqlite::params_from_iter(match_ids.iter()),
    ).map_err(|e| AppError::invalid(format!("Delete match_players failed: {e}")))?;

    let deleted = tx.execute(
        &format!("DELETE FROM matches WHERE match_id IN ({in_clause})"),
        rusqlite::params_from_iter(match_ids.iter()),
    ).map_err(|e| AppError::invalid(format!("Delete matches failed: {e}")))?;

    tx.commit()
        .map_err(|e| AppError::invalid(format!("Commit transaction failed: {e}")))?;

    Ok(deleted)
}
