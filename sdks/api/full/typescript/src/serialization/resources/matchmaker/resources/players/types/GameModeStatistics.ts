/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { Identifier } from "../../../../common/types/Identifier";
import { RegionStatistics } from "./RegionStatistics";

export const GameModeStatistics: core.serialization.ObjectSchema<
    serializers.matchmaker.GameModeStatistics.Raw,
    Rivet.matchmaker.GameModeStatistics
> = core.serialization.object({
    playerCount: core.serialization.property("player_count", core.serialization.number()),
    regions: core.serialization.record(Identifier, RegionStatistics),
});

export declare namespace GameModeStatistics {
    export interface Raw {
        player_count: number;
        regions: Record<Identifier.Raw, RegionStatistics.Raw>;
    }
}
