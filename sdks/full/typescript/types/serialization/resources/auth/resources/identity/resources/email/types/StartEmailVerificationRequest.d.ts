/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../../..";
import * as Rivet from "../../../../../../../../api";
import * as core from "../../../../../../../../core";
export declare const StartEmailVerificationRequest: core.serialization.ObjectSchema<serializers.auth.identity.StartEmailVerificationRequest.Raw, Rivet.auth.identity.StartEmailVerificationRequest>;
export declare namespace StartEmailVerificationRequest {
    interface Raw {
        email: string;
        captcha?: serializers.captcha.Config.Raw | null;
        game_id?: string | null;
    }
}
