/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as serializers from "../../../index";
import * as Rivet from "../../../../api/index";
import * as core from "../../../../core";
import { identity } from "../../index";
export declare const SearchResponse: core.serialization.ObjectSchema<serializers.identity.SearchResponse.Raw, Rivet.identity.SearchResponse>;
export declare namespace SearchResponse {
    interface Raw {
        identities: identity.Handle.Raw[];
        anchor?: string | null;
    }
}