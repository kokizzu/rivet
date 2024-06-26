/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../../../..";
import * as Rivet from "../../../../../../../../api";
import * as core from "../../../../../../../../core";
export declare const CreateGameRequest: core.serialization.ObjectSchema<serializers.cloud.games.CreateGameRequest.Raw, Rivet.cloud.games.CreateGameRequest>;
export declare namespace CreateGameRequest {
    interface Raw {
        name_id?: string | null;
        display_name: string;
        developer_group_id: string;
    }
}
