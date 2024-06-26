/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../..";
import * as Rivet from "../../../../../api";
import * as core from "../../../../../core";
export declare const SetupRequest: core.serialization.Schema<serializers.identity.SetupRequest.Raw, Rivet.identity.SetupRequest>;
export declare namespace SetupRequest {
    interface Raw {
        existing_identity_token?: serializers.Jwt.Raw | null;
    }
}
