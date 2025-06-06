/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { ValidationError } from "../../../../common/types/ValidationError";

export const ValidateGroupResponse: core.serialization.ObjectSchema<
    serializers.cloud.ValidateGroupResponse.Raw,
    Rivet.cloud.ValidateGroupResponse
> = core.serialization.object({
    errors: core.serialization.list(ValidationError),
});

export declare namespace ValidateGroupResponse {
    export interface Raw {
        errors: ValidationError.Raw[];
    }
}
