/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../..";
import * as Rivet from "../../../../../../api";
import * as core from "../../../../../../core";
export declare const StatAggregationMethod: core.serialization.Schema<serializers.game.StatAggregationMethod.Raw, Rivet.game.StatAggregationMethod>;
export declare namespace StatAggregationMethod {
    type Raw = "sum" | "average" | "min" | "max";
}
