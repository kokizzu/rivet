/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as Rivet from "../../../../../index";
/**
 * A union representing the state of a lobby.
 */
export interface LogsLobbyStatus {
    running: Rivet.EmptyObject;
    stopped?: Rivet.cloud.LogsLobbyStatusStopped;
}