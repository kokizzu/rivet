/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { Timestamp } from "../../../../common/types/Timestamp";
import { CdnNamespaceDomainVerificationStatus } from "./CdnNamespaceDomainVerificationStatus";
import { CdnNamespaceDomainVerificationMethod } from "./CdnNamespaceDomainVerificationMethod";

export const CdnNamespaceDomain: core.serialization.ObjectSchema<
    serializers.cloud.CdnNamespaceDomain.Raw,
    Rivet.cloud.CdnNamespaceDomain
> = core.serialization.object({
    domain: core.serialization.string(),
    createTs: core.serialization.property("create_ts", Timestamp),
    verificationStatus: core.serialization.property("verification_status", CdnNamespaceDomainVerificationStatus),
    verificationMethod: core.serialization.property("verification_method", CdnNamespaceDomainVerificationMethod),
    verificationErrors: core.serialization.property(
        "verification_errors",
        core.serialization.list(core.serialization.string()),
    ),
});

export declare namespace CdnNamespaceDomain {
    export interface Raw {
        domain: string;
        create_ts: Timestamp.Raw;
        verification_status: CdnNamespaceDomainVerificationStatus.Raw;
        verification_method: CdnNamespaceDomainVerificationMethod.Raw;
        verification_errors: string[];
    }
}
