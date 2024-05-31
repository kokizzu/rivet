/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../../../../..";
import * as Rivet from "../../../../../../../../../../api";
import * as core from "../../../../../../../../../../core";
export declare const Captcha: core.serialization.ObjectSchema<serializers.cloud.version.matchmaker.Captcha.Raw, Rivet.cloud.version.matchmaker.Captcha>;
export declare namespace Captcha {
    interface Raw {
        requests_before_reverify: number;
        verification_ttl: number;
        hcaptcha?: serializers.cloud.version.matchmaker.CaptchaHcaptcha.Raw | null;
        turnstile?: serializers.cloud.version.matchmaker.CaptchaTurnstile.Raw | null;
    }
}