/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
export declare const ConfigTurnstile: core.serialization.ObjectSchema<serializers.captcha.ConfigTurnstile.Raw, Rivet.captcha.ConfigTurnstile>;
export declare namespace ConfigTurnstile {
    interface Raw {
        client_response: string;
    }
}