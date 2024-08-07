/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { kv } from "../../../../index";
export declare const Entry: core.serialization.ObjectSchema<serializers.kv.Entry.Raw, Rivet.kv.Entry>;
export declare namespace Entry {
    interface Raw {
        key: kv.Key.Raw;
        value?: kv.Value.Raw;
        deleted?: boolean | null;
    }
}
