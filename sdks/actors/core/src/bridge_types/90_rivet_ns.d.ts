// DO NOT MODIFY
//
// Generated from sdks/actors-bridge/

import type { Metadata } from "./types/metadata.d.ts";
export type { Metadata } from "./types/metadata.d.ts";
export declare function deepFreeze(object: Record<any, any>): Readonly<Record<any, any>>;
export declare const ACTOR_CONTEXT: {
    metadata: Metadata;
    kv: {
        get: (key: import("./40_rivet_kv.d.ts").Key, options?: import("./40_rivet_kv.d.ts").GetOptions) => Promise<import("./40_rivet_kv.d.ts").Entry | null>;
        getBatch: (keys: import("./40_rivet_kv.d.ts").Key[], options?: import("./40_rivet_kv.d.ts").GetBatchOptions) => Promise<Map<import("./40_rivet_kv.d.ts").Key, import("./40_rivet_kv.d.ts").Entry>>;
        list: (options?: import("./40_rivet_kv.d.ts").ListOptions) => Promise<Map<import("./40_rivet_kv.d.ts").Key, import("./40_rivet_kv.d.ts").Entry>>;
        put: (key: import("./40_rivet_kv.d.ts").Key, value: import("./40_rivet_kv.d.ts").Entry | ArrayBuffer, options?: import("./40_rivet_kv.d.ts").PutOptions) => Promise<void>;
        putBatch: (obj: Map<import("./40_rivet_kv.d.ts").Key, import("./40_rivet_kv.d.ts").Entry | ArrayBuffer>, options?: import("./40_rivet_kv.d.ts").PutBatchOptions) => Promise<void>;
        delete: (key: import("./40_rivet_kv.d.ts").Key) => Promise<void>;
        deleteBatch: (keys: import("./40_rivet_kv.d.ts").Key[]) => Promise<void>;
        deleteAll: () => Promise<void>;
    };
};
