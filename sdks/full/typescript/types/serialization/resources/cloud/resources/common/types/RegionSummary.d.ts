/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../..";
import * as Rivet from "../../../../../../api";
import * as core from "../../../../../../core";
export declare const RegionSummary: core.serialization.ObjectSchema<serializers.cloud.RegionSummary.Raw, Rivet.cloud.RegionSummary>;
export declare namespace RegionSummary {
    interface Raw {
        region_id: string;
        region_name_id: string;
        provider: string;
        universal_region: serializers.cloud.UniversalRegion.Raw;
        provider_display_name: serializers.DisplayName.Raw;
        region_display_name: serializers.DisplayName.Raw;
    }
}
