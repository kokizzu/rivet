/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../../../../../index";
import * as Rivet from "../../../../../../../../../../api/index";
import * as core from "../../../../../../../../../../core";

export const LobbyGroupIdleLobbiesConfig: core.serialization.ObjectSchema<
    serializers.cloud.version.matchmaker.LobbyGroupIdleLobbiesConfig.Raw,
    Rivet.cloud.version.matchmaker.LobbyGroupIdleLobbiesConfig
> = core.serialization.object({
    minIdleLobbies: core.serialization.property("min_idle_lobbies", core.serialization.number()),
    maxIdleLobbies: core.serialization.property("max_idle_lobbies", core.serialization.number()),
});

export declare namespace LobbyGroupIdleLobbiesConfig {
    export interface Raw {
        min_idle_lobbies: number;
        max_idle_lobbies: number;
    }
}
