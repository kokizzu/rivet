/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../../../index";
import * as Rivet from "../../../../../../../../api/index";
import * as core from "../../../../../../../../core";
export declare const CompleteEmailVerificationRequest: core.serialization.ObjectSchema<serializers.auth.identity.CompleteEmailVerificationRequest.Raw, Rivet.auth.identity.CompleteEmailVerificationRequest>;
export declare namespace CompleteEmailVerificationRequest {
    interface Raw {
        verification_id: string;
        code: string;
    }
}
