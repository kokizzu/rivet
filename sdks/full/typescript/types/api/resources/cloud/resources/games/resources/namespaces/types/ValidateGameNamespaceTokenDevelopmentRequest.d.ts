/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as Rivet from "../../../../../../..";
export interface ValidateGameNamespaceTokenDevelopmentRequest {
    hostname: string;
    /** A list of docker ports. */
    lobbyPorts: Rivet.cloud.version.matchmaker.LobbyGroupRuntimeDockerPort[];
}
