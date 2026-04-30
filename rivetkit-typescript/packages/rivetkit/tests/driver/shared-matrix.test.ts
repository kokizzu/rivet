import { afterEach, describe, expect, test } from "vitest";
import {
	getDriverMatrixCells,
	SQLITE_DRIVER_MATRIX_OPTIONS,
} from "./shared-matrix";

const previousRuntimeEnv = process.env.RIVETKIT_DRIVER_TEST_RUNTIME;
const previousSqliteEnv = process.env.RIVETKIT_DRIVER_TEST_SQLITE;
const previousEncodingEnv = process.env.RIVETKIT_DRIVER_TEST_ENCODING;

function restoreMatrixEnv() {
	if (previousRuntimeEnv === undefined) {
		delete process.env.RIVETKIT_DRIVER_TEST_RUNTIME;
	} else {
		process.env.RIVETKIT_DRIVER_TEST_RUNTIME = previousRuntimeEnv;
	}

	if (previousSqliteEnv === undefined) {
		delete process.env.RIVETKIT_DRIVER_TEST_SQLITE;
	} else {
		process.env.RIVETKIT_DRIVER_TEST_SQLITE = previousSqliteEnv;
	}

	if (previousEncodingEnv === undefined) {
		delete process.env.RIVETKIT_DRIVER_TEST_ENCODING;
	} else {
		process.env.RIVETKIT_DRIVER_TEST_ENCODING = previousEncodingEnv;
	}
}

describe("driver matrix cells", () => {
	afterEach(() => {
		restoreMatrixEnv();
	});

	test("excludes wasm with local SQLite from the normal matrix", () => {
		const cells = getDriverMatrixCells(SQLITE_DRIVER_MATRIX_OPTIONS);

		expect(
			cells.some(
				(cell) =>
					cell.runtime === "wasm" && cell.sqliteBackend === "local",
			),
		).toBe(false);
		expect(
			cells.some(
				(cell) =>
					cell.runtime === "wasm" &&
					cell.sqliteBackend === "remote" &&
					cell.skipReason === undefined,
			),
		).toBe(true);
	});

	test("keeps the expected SQLite driver matrix cells", () => {
		const cells = getDriverMatrixCells(SQLITE_DRIVER_MATRIX_OPTIONS);

		expect(
			cells.map(
				(cell) =>
					`${cell.runtime}/${cell.sqliteBackend}/${cell.encoding}`,
			),
		).toEqual([
			"native/local/bare",
			"native/local/cbor",
			"native/local/json",
			"native/remote/bare",
			"native/remote/cbor",
			"native/remote/json",
			"wasm/remote/bare",
			"wasm/remote/cbor",
			"wasm/remote/json",
		]);
	});

	test("fails fast when env explicitly selects wasm with local SQLite", () => {
		process.env.RIVETKIT_DRIVER_TEST_RUNTIME = "wasm";
		process.env.RIVETKIT_DRIVER_TEST_SQLITE = "local";

		expect(() => getDriverMatrixCells()).toThrow(
			/WebAssembly runtime cannot use local SQLite/,
		);
	});
});
