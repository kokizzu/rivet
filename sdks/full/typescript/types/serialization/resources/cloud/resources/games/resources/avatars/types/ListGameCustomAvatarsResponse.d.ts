/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../../..";
import * as Rivet from "../../../../../../../../api";
import * as core from "../../../../../../../../core";
export declare const ListGameCustomAvatarsResponse: core.serialization.ObjectSchema<serializers.cloud.games.ListGameCustomAvatarsResponse.Raw, Rivet.cloud.games.ListGameCustomAvatarsResponse>;
export declare namespace ListGameCustomAvatarsResponse {
    interface Raw {
        custom_avatars: serializers.cloud.CustomAvatarSummary.Raw[];
    }
}
