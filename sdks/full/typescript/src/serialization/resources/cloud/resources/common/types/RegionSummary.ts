/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { UniversalRegion as cloud_common$$universalRegion } from "./UniversalRegion";
import { DisplayName as common$$displayName } from "../../../../common/types/DisplayName";
import { cloud, common } from "../../../../index";

export const RegionSummary: core.serialization.ObjectSchema<
    serializers.cloud.RegionSummary.Raw,
    Rivet.cloud.RegionSummary
> = core.serialization.object({
    regionId: core.serialization.property("region_id", core.serialization.string()),
    regionNameId: core.serialization.property("region_name_id", core.serialization.string()),
    provider: core.serialization.string(),
    universalRegion: core.serialization.property("universal_region", cloud_common$$universalRegion),
    providerDisplayName: core.serialization.property("provider_display_name", common$$displayName),
    regionDisplayName: core.serialization.property("region_display_name", common$$displayName),
});

export declare namespace RegionSummary {
    interface Raw {
        region_id: string;
        region_name_id: string;
        provider: string;
        universal_region: cloud.UniversalRegion.Raw;
        provider_display_name: common.DisplayName.Raw;
        region_display_name: common.DisplayName.Raw;
    }
}
