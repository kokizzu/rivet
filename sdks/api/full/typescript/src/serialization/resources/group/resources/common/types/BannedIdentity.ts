/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { Handle } from "../../../../identity/resources/common/types/Handle";
import { Timestamp } from "../../../../common/types/Timestamp";

export const BannedIdentity: core.serialization.ObjectSchema<
    serializers.group.BannedIdentity.Raw,
    Rivet.group.BannedIdentity
> = core.serialization.object({
    identity: Handle,
    banTs: core.serialization.property("ban_ts", Timestamp),
});

export declare namespace BannedIdentity {
    export interface Raw {
        identity: Handle.Raw;
        ban_ts: Timestamp.Raw;
    }
}
