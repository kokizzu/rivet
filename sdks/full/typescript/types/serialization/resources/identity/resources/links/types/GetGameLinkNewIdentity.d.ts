/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { common, identity } from "../../../../index";
export declare const GetGameLinkNewIdentity: core.serialization.ObjectSchema<serializers.identity.GetGameLinkNewIdentity.Raw, Rivet.identity.GetGameLinkNewIdentity>;
export declare namespace GetGameLinkNewIdentity {
    interface Raw {
        identity_token: common.Jwt.Raw;
        identity_token_expire_ts: common.Timestamp.Raw;
        identity: identity.Profile.Raw;
    }
}
