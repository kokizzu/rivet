/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Rivet from "../../../../index";

/**
 * @example
 *     {
 *         gameActivity: {
 *             message: "string",
 *             publicMetadata: {
 *                 "key": "value"
 *             },
 *             mutualMetadata: {
 *                 "key": "value"
 *             }
 *         }
 *     }
 */
export interface SetGameActivityRequest {
    gameActivity: Rivet.identity.UpdateGameActivity;
}
