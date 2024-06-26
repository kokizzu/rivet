/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../..";
import * as Rivet from "../../../../../../api";
import * as core from "../../../../../../core";
export declare const Pool: core.serialization.ObjectSchema<serializers.admin.Pool.Raw, Rivet.admin.Pool>;
export declare namespace Pool {
    interface Raw {
        pool_type: serializers.admin.PoolType.Raw;
        hardware: serializers.admin.Hardware.Raw[];
        desired_count: number;
        max_count: number;
        drain_timeout: number;
    }
}
