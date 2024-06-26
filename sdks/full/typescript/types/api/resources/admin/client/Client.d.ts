/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as environments from "../../../../environments";
import * as core from "../../../../core";
import * as Rivet from "../../..";
import { Clusters } from "../resources/clusters/client/Client";
export declare namespace Admin {
    interface Options {
        environment?: core.Supplier<environments.RivetEnvironment | string>;
        token?: core.Supplier<core.BearerToken | undefined>;
        fetcher?: core.FetchFunction;
    }
    interface RequestOptions {
        timeoutInSeconds?: number;
        maxRetries?: number;
    }
}
export declare class Admin {
    protected readonly _options: Admin.Options;
    constructor(_options?: Admin.Options);
    /**
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    login(request: Rivet.admin.LoginRequest, requestOptions?: Admin.RequestOptions): Promise<Rivet.admin.LoginResponse>;
    protected _clusters: Clusters | undefined;
    get clusters(): Clusters;
    protected _getAuthorizationHeader(): Promise<string | undefined>;
}
