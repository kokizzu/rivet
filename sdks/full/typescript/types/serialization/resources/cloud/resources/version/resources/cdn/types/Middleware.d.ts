/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../../../index";
import * as Rivet from "../../../../../../../../api/index";
import * as core from "../../../../../../../../core";
import { cloud } from "../../../../../../index";
export declare const Middleware: core.serialization.ObjectSchema<serializers.cloud.version.cdn.Middleware.Raw, Rivet.cloud.version.cdn.Middleware>;
export declare namespace Middleware {
    interface Raw {
        kind: cloud.version.cdn.MiddlewareKind.Raw;
    }
}
