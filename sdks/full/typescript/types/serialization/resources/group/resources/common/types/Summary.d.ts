/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../../..";
import * as Rivet from "../../../../../../api";
import * as core from "../../../../../../core";
export declare const Summary: core.serialization.ObjectSchema<serializers.group.Summary.Raw, Rivet.group.Summary>;
export declare namespace Summary {
    interface Raw {
        group_id: string;
        display_name: serializers.DisplayName.Raw;
        avatar_url?: string | null;
        external: serializers.group.ExternalLinks.Raw;
        is_developer: boolean;
        bio: serializers.Bio.Raw;
        is_current_identity_member: boolean;
        publicity: serializers.group.Publicity.Raw;
        member_count: number;
        owner_identity_id: string;
    }
}
