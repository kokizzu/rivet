/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { common, group } from "../../../../index";
export declare const Summary: core.serialization.ObjectSchema<serializers.game.Summary.Raw, Rivet.game.Summary>;
export declare namespace Summary {
    interface Raw {
        game_id: string;
        name_id: common.Identifier.Raw;
        display_name: common.DisplayName.Raw;
        logo_url?: string | null;
        banner_url?: string | null;
        url: string;
        developer: group.Handle.Raw;
        total_player_count: number;
    }
}