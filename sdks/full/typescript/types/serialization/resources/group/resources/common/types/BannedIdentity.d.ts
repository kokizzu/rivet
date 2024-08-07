/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { identity, common } from "../../../../index";
export declare const BannedIdentity: core.serialization.ObjectSchema<serializers.group.BannedIdentity.Raw, Rivet.group.BannedIdentity>;
export declare namespace BannedIdentity {
    interface Raw {
        identity: identity.Handle.Raw;
        ban_ts: common.Timestamp.Raw;
    }
}
