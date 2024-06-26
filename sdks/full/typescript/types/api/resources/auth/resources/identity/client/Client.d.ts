/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as environments from "../../../../../../environments";
import * as core from "../../../../../../core";
import { AccessToken } from "../resources/accessToken/client/Client";
import { Email } from "../resources/email/client/Client";
export declare namespace Identity {
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
export declare class Identity {
    protected readonly _options: Identity.Options;
    constructor(_options?: Identity.Options);
    protected _accessToken: AccessToken | undefined;
    get accessToken(): AccessToken;
    protected _email: Email | undefined;
    get email(): Email;
}
