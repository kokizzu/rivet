/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { CdnNamespaceConfig as cloud_common$$cdnNamespaceConfig } from "./CdnNamespaceConfig";
import { MatchmakerNamespaceConfig as cloud_common$$matchmakerNamespaceConfig } from "./MatchmakerNamespaceConfig";
import { KvNamespaceConfig as cloud_common$$kvNamespaceConfig } from "./KvNamespaceConfig";
import { IdentityNamespaceConfig as cloud_common$$identityNamespaceConfig } from "./IdentityNamespaceConfig";
import { cloud } from "../../../../index";

export const NamespaceConfig: core.serialization.ObjectSchema<
    serializers.cloud.NamespaceConfig.Raw,
    Rivet.cloud.NamespaceConfig
> = core.serialization.object({
    cdn: cloud_common$$cdnNamespaceConfig,
    matchmaker: cloud_common$$matchmakerNamespaceConfig,
    kv: cloud_common$$kvNamespaceConfig,
    identity: cloud_common$$identityNamespaceConfig,
});

export declare namespace NamespaceConfig {
    interface Raw {
        cdn: cloud.CdnNamespaceConfig.Raw;
        matchmaker: cloud.MatchmakerNamespaceConfig.Raw;
        kv: cloud.KvNamespaceConfig.Raw;
        identity: cloud.IdentityNamespaceConfig.Raw;
    }
}
