/**
 * This file was auto-generated by Fern from our API Definition.
 */

/**
 * A public lobby in the lobby list.
 */
export interface LobbyInfo {
    regionId: string;
    gameModeId: string;
    lobbyId: string;
    maxPlayersNormal: number;
    maxPlayersDirect: number;
    maxPlayersParty: number;
    totalPlayerCount: number;
    state?: unknown;
}