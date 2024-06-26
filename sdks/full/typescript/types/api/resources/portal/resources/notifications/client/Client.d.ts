/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as environments from "../../../../../../environments";
import * as core from "../../../../../../core";
import * as Rivet from "../../../../..";
export declare namespace Notifications {
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
export declare class Notifications {
    protected readonly _options: Notifications.Options;
    constructor(_options?: Notifications.Options);
    /**
     * Registers push notifications for the current identity.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    registerNotifications(request: Rivet.portal.RegisterNotificationsRequest, requestOptions?: Notifications.RequestOptions): Promise<void>;
    /**
     * Unregister push notification for the current identity.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    unregisterNotifications(request: Rivet.portal.UnregisterNotificationsRequest, requestOptions?: Notifications.RequestOptions): Promise<void>;
    protected _getAuthorizationHeader(): Promise<string | undefined>;
}
