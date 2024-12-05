/**
 * This file re-exports types from our internal bindings as a module dependency.
 *
 * An alternative approach is to use `declare module Rivet` then have developers
 * use `import "path/to/types.d.ts"`. However, this is rather annoying to do in
 * every file that uses Rivet. Additionally, it's not something most JS
 * developers are used to.
 */

import { ACTOR_CONTEXT } from "./types/90_rivet_ns.d.ts";

export type ActorContext = typeof ACTOR_CONTEXT;