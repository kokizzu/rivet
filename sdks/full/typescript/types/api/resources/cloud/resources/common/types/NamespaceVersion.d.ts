/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as Rivet from "../../../../..";
/**
 * A previously deployed namespace version.
 */
export interface NamespaceVersion {
    /** A universally unique identifier. */
    namespaceId: string;
    /** A universally unique identifier. */
    versionId: string;
    deployTs: Rivet.Timestamp;
}