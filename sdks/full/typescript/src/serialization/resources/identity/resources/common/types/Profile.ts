/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../index";
import * as Rivet from "../../../../../../api/index";
import * as core from "../../../../../../core";
import { DisplayName as common$$displayName } from "../../../../common/types/DisplayName";
import { AccountNumber as common$$accountNumber } from "../../../../common/types/AccountNumber";
import { ExternalLinks as identity_common$$externalLinks } from "./ExternalLinks";
import { DevState as identity_common$$devState } from "./DevState";
import { Timestamp as common$$timestamp } from "../../../../common/types/Timestamp";
import { Bio as common$$bio } from "../../../../common/types/Bio";
import { LinkedAccount as identity_common$$linkedAccount } from "./LinkedAccount";
import { Group as identity_common$$group } from "./Group";
import { StatSummary as game_common$$statSummary } from "../../../../game/resources/common/types/StatSummary";
import { common, identity, game } from "../../../../index";

export const Profile: core.serialization.ObjectSchema<serializers.identity.Profile.Raw, Rivet.identity.Profile> =
    core.serialization.object({
        identityId: core.serialization.property("identity_id", core.serialization.string()),
        displayName: core.serialization.property("display_name", common$$displayName),
        accountNumber: core.serialization.property("account_number", common$$accountNumber),
        avatarUrl: core.serialization.property("avatar_url", core.serialization.string()),
        isRegistered: core.serialization.property("is_registered", core.serialization.boolean()),
        external: identity_common$$externalLinks,
        isAdmin: core.serialization.property("is_admin", core.serialization.boolean()),
        isGameLinked: core.serialization.property("is_game_linked", core.serialization.boolean().optional()),
        devState: core.serialization.property("dev_state", identity_common$$devState.optional()),
        followerCount: core.serialization.property("follower_count", core.serialization.number()),
        followingCount: core.serialization.property("following_count", core.serialization.number()),
        following: core.serialization.boolean(),
        isFollowingMe: core.serialization.property("is_following_me", core.serialization.boolean()),
        isMutualFollowing: core.serialization.property("is_mutual_following", core.serialization.boolean()),
        joinTs: core.serialization.property("join_ts", common$$timestamp),
        bio: common$$bio,
        linkedAccounts: core.serialization.property(
            "linked_accounts",
            core.serialization.list(identity_common$$linkedAccount)
        ),
        groups: core.serialization.list(identity_common$$group),
        games: core.serialization.list(game_common$$statSummary),
        awaitingDeletion: core.serialization.property("awaiting_deletion", core.serialization.boolean().optional()),
    });

export declare namespace Profile {
    interface Raw {
        identity_id: string;
        display_name: common.DisplayName.Raw;
        account_number: common.AccountNumber.Raw;
        avatar_url: string;
        is_registered: boolean;
        external: identity.ExternalLinks.Raw;
        is_admin: boolean;
        is_game_linked?: boolean | null;
        dev_state?: identity.DevState.Raw | null;
        follower_count: number;
        following_count: number;
        following: boolean;
        is_following_me: boolean;
        is_mutual_following: boolean;
        join_ts: common.Timestamp.Raw;
        bio: common.Bio.Raw;
        linked_accounts: identity.LinkedAccount.Raw[];
        groups: identity.Group.Raw[];
        games: game.StatSummary.Raw[];
        awaiting_deletion?: boolean | null;
    }
}
