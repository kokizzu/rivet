/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../..";
import * as Rivet from "../../../../../../api";
import * as core from "../../../../../../core";
export declare const CreateInviteRequest: core.serialization.ObjectSchema<serializers.group.CreateInviteRequest.Raw, Rivet.group.CreateInviteRequest>;
export declare namespace CreateInviteRequest {
    interface Raw {
        ttl?: number | null;
        use_count?: number | null;
    }
}