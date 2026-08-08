export interface Cs2ssRatingExtras {
  kastRounds?: number;
  tradeKills?: number;
  multikill2?: number;
  multikill3?: number;
  multikill4?: number;
  multikill5?: number;
  clutchAttempts?: number;
  clutchesWon?: number;
}

export function cs2ssCalcRating(
  kills: number,
  deaths: number,
  assists: number,
  damage: number,
  _headshotKills: number,
  rounds: number,
  extras: Cs2ssRatingExtras = {},
): number {
  if (rounds <= 0) return 0;
  const kpr = kills / rounds;
  const dpr = deaths / rounds;
  const apr = assists / rounds;
  const adr = damage / rounds;
  const kast = (extras.kastRounds ?? 0) / rounds;
  const tradeRate = (extras.tradeKills ?? 0) / rounds;
  const multikillImpact = (
    (extras.multikill2 ?? 0) * 0.03 +
    (extras.multikill3 ?? 0) * 0.08 +
    (extras.multikill4 ?? 0) * 0.15 +
    (extras.multikill5 ?? 0) * 0.28
  );
  const clutchImpact = (extras.clutchesWon ?? 0) * 0.22;
  const clutchConversion = extras.clutchAttempts
    ? Math.min(0.06, (extras.clutchesWon ?? 0) / extras.clutchAttempts * 0.06)
    : 0;
  const rating = 1
    + (kpr - 0.72) * 0.52
    + (adr - 68) / 100 * 0.75
    - (dpr - 0.68) * 0.32
    + (apr - 0.18) * 0.14
    + (kast - 0.70) * 0.30
    + tradeRate * 0.16
    + multikillImpact
    + clutchImpact
    + clutchConversion;
  return Math.round(Math.max(0, rating) * 100) / 100;
}

export function cs2ssCalcKd(kills: number, deaths: number): number {
  return Math.round((kills / Math.max(1, deaths)) * 100) / 100;
}

export function cs2ssCalcKda(kills: number, deaths: number, assists: number): number {
  return Math.round(((kills + assists) / Math.max(1, deaths)) * 100) / 100;
}

export function cs2ssCalcAdr(damage: number, rounds: number): number {
  if (rounds <= 0) return 0;
  return Math.round((damage / rounds) * 10) / 10;
}

export function cs2ssCalcKast(kastRounds: number, rounds: number): number {
  if (rounds <= 0) return 0;
  return Math.round(kastRounds / rounds * 1000) / 10;
}

export function cs2ssCalcHsPct(headshotKills: number, kills: number): number {
  if (kills <= 0) return 0;
  return Math.round((headshotKills / kills) * 100);
}

export function cs2ssCalcWinRate(wins: number, matches: number): number {
  if (matches <= 0) return 0;
  return Math.round((wins / matches) * 1000) / 10;
}