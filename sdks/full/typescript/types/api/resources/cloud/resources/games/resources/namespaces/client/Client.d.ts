/**
 * This file was auto-generated by Fern from our API Definition.
 */
import * as environments from "../../../../../../../../environments";
import * as core from "../../../../../../../../core";
import * as Rivet from "../../../../../../..";
import { Analytics } from "../resources/analytics/client/Client";
import { Logs } from "../resources/logs/client/Client";
export declare namespace Namespaces {
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
export declare class Namespaces {
    protected readonly _options: Namespaces.Options;
    constructor(_options?: Namespaces.Options);
    /**
     * Creates a new namespace for the given game.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    createGameNamespace(gameId: string, request: Rivet.cloud.games.namespaces.CreateGameNamespaceRequest, requestOptions?: Namespaces.RequestOptions): Promise<Rivet.cloud.games.namespaces.CreateGameNamespaceResponse>;
    /**
     * Validates information used to create a new game namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    validateGameNamespace(gameId: string, request: Rivet.cloud.games.namespaces.ValidateGameNamespaceRequest, requestOptions?: Namespaces.RequestOptions): Promise<Rivet.cloud.games.namespaces.ValidateGameNamespaceResponse>;
    /**
     * Gets a game namespace by namespace ID.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    getGameNamespaceById(gameId: string, namespaceId: string, requestOptions?: Namespaces.RequestOptions): Promise<Rivet.cloud.games.namespaces.GetGameNamespaceByIdResponse>;
    /**
     * Adds an authenticated user to the given game namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    updateNamespaceCdnAuthUser(gameId: string, namespaceId: string, request: Rivet.cloud.games.namespaces.UpdateNamespaceCdnAuthUserRequest, requestOptions?: Namespaces.RequestOptions): Promise<void>;
    /**
     * Removes an authenticated user from the given game namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    removeNamespaceCdnAuthUser(gameId: string, namespaceId: string, user: string, requestOptions?: Namespaces.RequestOptions): Promise<void>;
    /**
     * Updates the CDN authentication type of the given game namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    setNamespaceCdnAuthType(gameId: string, namespaceId: string, request: Rivet.cloud.games.namespaces.SetNamespaceCdnAuthTypeRequest, requestOptions?: Namespaces.RequestOptions): Promise<void>;
    /**
     * Toggles whether or not to allow authentication based on domain for the given game namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    toggleNamespaceDomainPublicAuth(gameId: string, namespaceId: string, request: Rivet.cloud.games.namespaces.ToggleNamespaceDomainPublicAuthRequest, requestOptions?: Namespaces.RequestOptions): Promise<void>;
    /**
     * Adds a domain to the given game namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    addNamespaceDomain(gameId: string, namespaceId: string, request: Rivet.cloud.games.namespaces.AddNamespaceDomainRequest, requestOptions?: Namespaces.RequestOptions): Promise<void>;
    /**
     * Removes a domain from the given game namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    removeNamespaceDomain(gameId: string, namespaceId: string, domain: string, requestOptions?: Namespaces.RequestOptions): Promise<void>;
    /**
     * Updates matchmaker config for the given game namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    updateGameNamespaceMatchmakerConfig(gameId: string, namespaceId: string, request: Rivet.cloud.games.namespaces.UpdateGameNamespaceMatchmakerConfigRequest, requestOptions?: Namespaces.RequestOptions): Promise<void>;
    /**
     * Gets the version history for a given namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    getGameNamespaceVersionHistoryList(gameId: string, namespaceId: string, request?: Rivet.cloud.games.namespaces.GetGameNamespaceVersionHistoryRequest, requestOptions?: Namespaces.RequestOptions): Promise<Rivet.cloud.games.namespaces.GetGameNamespaceVersionHistoryResponse>;
    /**
     * Validates information used to update a game namespace's matchmaker config.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    validateGameNamespaceMatchmakerConfig(gameId: string, namespaceId: string, request: Rivet.cloud.games.namespaces.ValidateGameNamespaceMatchmakerConfigRequest, requestOptions?: Namespaces.RequestOptions): Promise<Rivet.cloud.games.namespaces.ValidateGameNamespaceMatchmakerConfigResponse>;
    /**
     * Creates a development token for the given namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    createGameNamespaceTokenDevelopment(gameId: string, namespaceId: string, request: Rivet.cloud.games.namespaces.CreateGameNamespaceTokenDevelopmentRequest, requestOptions?: Namespaces.RequestOptions): Promise<Rivet.cloud.games.namespaces.CreateGameNamespaceTokenDevelopmentResponse>;
    /**
     * Validates information used to create a new game namespace development token.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    validateGameNamespaceTokenDevelopment(gameId: string, namespaceId: string, request: Rivet.cloud.games.namespaces.ValidateGameNamespaceTokenDevelopmentRequest, requestOptions?: Namespaces.RequestOptions): Promise<Rivet.cloud.games.namespaces.ValidateGameNamespaceTokenDevelopmentResponse>;
    /**
     * Creates a public token for the given namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    createGameNamespaceTokenPublic(gameId: string, namespaceId: string, requestOptions?: Namespaces.RequestOptions): Promise<Rivet.cloud.games.namespaces.CreateGameNamespaceTokenPublicResponse>;
    /**
     * Updates the version of a game namespace.
     * @throws {@link Rivet.InternalError}
     * @throws {@link Rivet.RateLimitError}
     * @throws {@link Rivet.ForbiddenError}
     * @throws {@link Rivet.UnauthorizedError}
     * @throws {@link Rivet.NotFoundError}
     * @throws {@link Rivet.BadRequestError}
     */
    updateGameNamespaceVersion(gameId: string, namespaceId: string, request: Rivet.cloud.games.namespaces.UpdateGameNamespaceVersionRequest, requestOptions?: Namespaces.RequestOptions): Promise<void>;
    protected _analytics: Analytics | undefined;
    get analytics(): Analytics;
    protected _logs: Logs | undefined;
    get logs(): Logs;
    protected _getAuthorizationHeader(): Promise<string | undefined>;
}
