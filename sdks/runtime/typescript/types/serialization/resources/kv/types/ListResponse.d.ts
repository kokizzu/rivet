/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../index";
import * as Rivet from "../../../../api/index";
import * as core from "../../../../core";
import { kv } from "../../index";
export declare const ListResponse: core.serialization.ObjectSchema<serializers.kv.ListResponse.Raw, Rivet.kv.ListResponse>;
export declare namespace ListResponse {
    interface Raw {
        entries: kv.Entry.Raw[];
    }
}