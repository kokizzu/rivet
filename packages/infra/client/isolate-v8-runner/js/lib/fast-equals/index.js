// DO NOT MODIFY
//
// Generated with scripts/sdk_actor/compile_bridge.ts

import { createEqualityComparator, createEqualityComparatorConfig, createInternalEqualityComparator, createIsEqual, } from "./comparator.js";
import { sameValueZeroEqual } from "./utils.js";
export { sameValueZeroEqual };
export * from "./internalTypes.js";
/**
 * Whether the items passed are deeply-equal in value.
 */
export const deepEqual = createCustomEqual();
/**
 * Whether the items passed are deeply-equal in value based on strict comparison.
 */
export const strictDeepEqual = createCustomEqual({ strict: true });
/**
 * Whether the items passed are deeply-equal in value, including circular references.
 */
export const circularDeepEqual = createCustomEqual({ circular: true });
/**
 * Whether the items passed are deeply-equal in value, including circular references,
 * based on strict comparison.
 */
export const strictCircularDeepEqual = createCustomEqual({
    circular: true,
    strict: true,
});
/**
 * Whether the items passed are shallowly-equal in value.
 */
export const shallowEqual = createCustomEqual({
    createInternalComparator: () => sameValueZeroEqual,
});
/**
 * Whether the items passed are shallowly-equal in value based on strict comparison
 */
export const strictShallowEqual = createCustomEqual({
    strict: true,
    createInternalComparator: () => sameValueZeroEqual,
});
/**
 * Whether the items passed are shallowly-equal in value, including circular references.
 */
export const circularShallowEqual = createCustomEqual({
    circular: true,
    createInternalComparator: () => sameValueZeroEqual,
});
/**
 * Whether the items passed are shallowly-equal in value, including circular references,
 * based on strict comparison.
 */
export const strictCircularShallowEqual = createCustomEqual({
    circular: true,
    createInternalComparator: () => sameValueZeroEqual,
    strict: true,
});
/**
 * Create a custom equality comparison method.
 *
 * This can be done to create very targeted comparisons in extreme hot-path scenarios
 * where the standard methods are not performant enough, but can also be used to provide
 * support for legacy environments that do not support expected features like
 * `RegExp.prototype.flags` out of the box.
 */
export function createCustomEqual(options = {}) {
    const { circular = false, createInternalComparator: createCustomInternalComparator, createState, strict = false, } = options;
    const config = createEqualityComparatorConfig(options);
    const comparator = createEqualityComparator(config);
    const equals = createCustomInternalComparator
        ? createCustomInternalComparator(comparator)
        : createInternalEqualityComparator(comparator);
    return createIsEqual({ circular, comparator, createState, equals, strict });
}