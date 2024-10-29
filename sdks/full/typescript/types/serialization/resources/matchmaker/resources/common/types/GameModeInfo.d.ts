/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { common } from "../../../../index";
export declare const GameModeInfo: core.serialization.ObjectSchema<serializers.matchmaker.GameModeInfo.Raw, Rivet.matchmaker.GameModeInfo>;
export declare namespace GameModeInfo {
    interface Raw {
        game_mode_id: common.Identifier.Raw;
    }
}