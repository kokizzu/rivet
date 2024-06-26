/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../../..";
import * as Rivet from "../../../../../../../../api";
import * as core from "../../../../../../../../core";

export const ReserveVersionNameResponse: core.serialization.ObjectSchema<
    serializers.cloud.games.ReserveVersionNameResponse.Raw,
    Rivet.cloud.games.ReserveVersionNameResponse
> = core.serialization.object({
    versionDisplayName: core.serialization.property(
        "version_display_name",
        core.serialization.lazy(async () => (await import("../../../../../../..")).DisplayName)
    ),
});

export declare namespace ReserveVersionNameResponse {
    interface Raw {
        version_display_name: serializers.DisplayName.Raw;
    }
}
