import { describe, expect, test } from "vitest";
import {
	normalizeRuntimeSqlExecuteResult,
	type RuntimeSqlBindParam,
	type RuntimeSqlBindParams,
	type RuntimeSqlExecuteResult,
} from "./runtime";

describe("runtime SQL boundary", () => {
	test("accepts exact bind param variants", () => {
		const blob = new Uint8Array([1, 2, 3]);
		const params = [
			{ kind: "null" },
			{ kind: "int", intValue: 1 },
			{ kind: "float", floatValue: 1.5 },
			{ kind: "text", textValue: "text" },
			{ kind: "blob", blobValue: blob },
		] satisfies RuntimeSqlBindParams;

		expect(params).toEqual([
			{ kind: "null" },
			{ kind: "int", intValue: 1 },
			{ kind: "float", floatValue: 1.5 },
			{ kind: "text", textValue: "text" },
			{ kind: "blob", blobValue: blob },
		]);
	});

	test("rejects bind params with mismatched value fields at typecheck time", () => {
		const invalidIntParamCandidate = {
			kind: "int",
			intValue: 1,
			textValue: "extra",
		} as const;
		// @ts-expect-error Runtime SQL int params must only carry intValue.
		const invalidIntParam: RuntimeSqlBindParam = invalidIntParamCandidate;

		expect(invalidIntParam.kind).toBe("int");
	});

	test("normalizes exact execute result routes", () => {
		const base = {
			columns: ["value"],
			rows: [[1]],
			changes: 1,
			lastInsertRowId: null,
		};

		expect(
			normalizeRuntimeSqlExecuteResult({ ...base, route: "read" }).route,
		).toBe("read");
		expect(
			normalizeRuntimeSqlExecuteResult({ ...base, route: "write" }).route,
		).toBe("write");
		expect(
			normalizeRuntimeSqlExecuteResult({
				...base,
				route: "writeFallback",
			}).route,
		).toBe("writeFallback");
		expect(() =>
			normalizeRuntimeSqlExecuteResult({ ...base, route: "custom" }),
		).toThrow("unsupported runtime sqlite execute route: custom");
	});

	test("rejects custom execute result routes at typecheck time", () => {
		const invalidRouteResultCandidate = {
			columns: [],
			rows: [],
			changes: 0,
			route: "custom",
		} as const;
		// @ts-expect-error Runtime SQL execute routes are exact.
		const invalidRouteResult: RuntimeSqlExecuteResult =
			invalidRouteResultCandidate;

		expect(invalidRouteResult.route).toBe("custom");
	});
});
