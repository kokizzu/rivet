/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../../..";
import * as Rivet from "../../../../../../../../api";
import * as core from "../../../../../../../../core";
export declare const CustomHeadersMiddleware: core.serialization.ObjectSchema<serializers.cloud.version.cdn.CustomHeadersMiddleware.Raw, Rivet.cloud.version.cdn.CustomHeadersMiddleware>;
export declare namespace CustomHeadersMiddleware {
    interface Raw {
        headers: serializers.cloud.version.cdn.Header.Raw[];
    }
}
