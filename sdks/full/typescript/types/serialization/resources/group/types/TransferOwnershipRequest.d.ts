/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../index";
import * as Rivet from "../../../../api/index";
import * as core from "../../../../core";
export declare const TransferOwnershipRequest: core.serialization.ObjectSchema<serializers.group.TransferOwnershipRequest.Raw, Rivet.group.TransferOwnershipRequest>;
export declare namespace TransferOwnershipRequest {
    interface Raw {
        new_owner_identity_id: string;
    }
}
