/**
 * This file was auto-generated by Fern from our API Definition.
 */
/**
 * **Deprecated**
 * The registration requirement for a user when joining/finding/creating a lobby. "None" allows for connections without an identity.
 */
export declare type GameModeIdentityRequirement = "none" | "guest" | "registered";
export declare const GameModeIdentityRequirement: {
    readonly None: "none";
    readonly Guest: "guest";
    readonly Registered: "registered";
};