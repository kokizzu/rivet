/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as Rivet from "../../../../../index";
export interface CreateBuildResponse {
    build: string;
    imagePresignedRequest?: Rivet.upload.PresignedRequest;
    imagePresignedRequests?: Rivet.upload.PresignedRequest[];
}