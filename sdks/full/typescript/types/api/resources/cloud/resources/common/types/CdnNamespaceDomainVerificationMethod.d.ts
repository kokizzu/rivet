/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as Rivet from "../../../../..";
/**
 * A union representing the verification method used for this CDN domain.
 */
export interface CdnNamespaceDomainVerificationMethod {
    invalid?: Rivet.EmptyObject;
    http?: Rivet.cloud.CdnNamespaceDomainVerificationMethodHttp;
}
