/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../../../../..";
import * as Rivet from "../../../../../../../../../../api";
import * as core from "../../../../../../../../../../core";

export const LobbyGroupRegion: core.serialization.ObjectSchema<
    serializers.cloud.version.matchmaker.LobbyGroupRegion.Raw,
    Rivet.cloud.version.matchmaker.LobbyGroupRegion
> = core.serialization.object({
    regionId: core.serialization.property("region_id", core.serialization.string()),
    tierNameId: core.serialization.property("tier_name_id", core.serialization.string()),
    idleLobbies: core.serialization.property(
        "idle_lobbies",
        core.serialization
            .lazyObject(
                async () =>
                    (await import("../../../../../../../../..")).cloud.version.matchmaker.LobbyGroupIdleLobbiesConfig
            )
            .optional()
    ),
});

export declare namespace LobbyGroupRegion {
    interface Raw {
        region_id: string;
        tier_name_id: string;
        idle_lobbies?: serializers.cloud.version.matchmaker.LobbyGroupIdleLobbiesConfig.Raw | null;
    }
}