/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../..";
import * as Rivet from "../../../../api";
import * as core from "../../../../core";
export declare const ListFollowersResponse: core.serialization.ObjectSchema<serializers.identity.ListFollowersResponse.Raw, Rivet.identity.ListFollowersResponse>;
export declare namespace ListFollowersResponse {
    interface Raw {
        identities: serializers.identity.Handle.Raw[];
        anchor?: string | null;
        watch: serializers.WatchResponse.Raw;
    }
}