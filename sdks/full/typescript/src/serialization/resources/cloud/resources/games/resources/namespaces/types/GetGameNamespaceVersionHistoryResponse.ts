/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../../../index";
import * as Rivet from "../../../../../../../../api/index";
import * as core from "../../../../../../../../core";
import { NamespaceVersion as cloud_common$$namespaceVersion } from "../../../../common/types/NamespaceVersion";
import { cloud } from "../../../../../../index";

export const GetGameNamespaceVersionHistoryResponse: core.serialization.ObjectSchema<
    serializers.cloud.games.namespaces.GetGameNamespaceVersionHistoryResponse.Raw,
    Rivet.cloud.games.namespaces.GetGameNamespaceVersionHistoryResponse
> = core.serialization.object({
    versions: core.serialization.list(cloud_common$$namespaceVersion),
});

export declare namespace GetGameNamespaceVersionHistoryResponse {
    interface Raw {
        versions: cloud.NamespaceVersion.Raw[];
    }
}
