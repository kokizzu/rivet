/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { common } from "../../../../index";
export declare const LeaderboardCategory: core.serialization.ObjectSchema<serializers.game.LeaderboardCategory.Raw, Rivet.game.LeaderboardCategory>;
export declare namespace LeaderboardCategory {
    interface Raw {
        display_name: common.DisplayName.Raw;
    }
}
