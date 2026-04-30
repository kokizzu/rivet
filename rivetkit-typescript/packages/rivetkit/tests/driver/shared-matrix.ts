import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { afterAll, describe } from "vitest";
import {
	type DriverRegistryVariant,
	getDriverRegistryVariants,
} from "../driver-registry-variants";
import {
	createNativeDriverTestConfig,
	createWasmDriverTestConfig,
	releaseSharedEngine,
} from "./shared-harness";
import type {
	DriverRuntime,
	DriverSqliteBackend,
	DriverTestConfig,
} from "./shared-types";

const describeDriverSuite =
	process.env.RIVETKIT_DRIVER_TEST_PARALLEL === "1"
		? describe
		: describe.sequential;
const TEST_DIR = join(dirname(fileURLToPath(import.meta.url)), "..");

export interface DriverMatrixOptions {
	registryVariants?: DriverRegistryVariant["name"][];
	encodings?: Array<NonNullable<DriverTestConfig["encoding"]>>;
	runtimes?: DriverRuntime[];
	sqliteBackends?: DriverSqliteBackend[];
	config?: Pick<DriverTestConfig, "features" | "skip">;
}

export const SQLITE_DRIVER_MATRIX_OPTIONS = {
	runtimes: ["native", "wasm"],
	sqliteBackends: ["local", "remote"],
} as const satisfies Pick<DriverMatrixOptions, "runtimes" | "sqliteBackends">;

export interface DriverMatrixCell {
	runtime: DriverRuntime;
	sqliteBackend: DriverSqliteBackend;
	encoding: NonNullable<DriverTestConfig["encoding"]>;
	skipReason?: string;
}

function envList<T extends string>(
	name: string,
	allowed: readonly T[],
): T[] | undefined {
	const value = process.env[name];
	if (!value) {
		return undefined;
	}

	const values = value
		.split(",")
		.map((part) => part.trim())
		.filter(Boolean);
	for (const item of values) {
		if (!allowed.includes(item as T)) {
			throw new Error(
				`invalid ${name} value '${item}', expected one of ${allowed.join(", ")}`,
			);
		}
	}
	return values as T[];
}

function hasEnvMatrixOverride() {
	return (
		process.env.RIVETKIT_DRIVER_TEST_RUNTIME !== undefined ||
		process.env.RIVETKIT_DRIVER_TEST_SQLITE !== undefined ||
		process.env.RIVETKIT_DRIVER_TEST_ENCODING !== undefined
	);
}

export function getDriverMatrixCells(
	options: DriverMatrixOptions = {},
): DriverMatrixCell[] {
	const envEncodings = envList("RIVETKIT_DRIVER_TEST_ENCODING", [
		"bare",
		"cbor",
		"json",
	] as const);
	const envRuntimes = envList("RIVETKIT_DRIVER_TEST_RUNTIME", [
		"native",
		"wasm",
	] as const);
	const envSqliteBackends = envList("RIVETKIT_DRIVER_TEST_SQLITE", [
		"local",
		"remote",
	] as const);
	const encodings = envEncodings ??
		options.encodings ?? ["bare", "cbor", "json"];
	const runtimes = envRuntimes ?? options.runtimes ?? ["native"];
	const sqliteBackends = envSqliteBackends ??
		options.sqliteBackends ?? ["local"];
	const cells: DriverMatrixCell[] = [];

	if (envRuntimes?.includes("wasm") && envSqliteBackends?.includes("local")) {
		throw new Error(
			"invalid driver test matrix: WebAssembly runtime cannot use local SQLite. Set RIVETKIT_DRIVER_TEST_SQLITE=remote for wasm driver tests.",
		);
	}

	for (const runtime of runtimes) {
		for (const sqliteBackend of sqliteBackends) {
			if (runtime === "wasm" && sqliteBackend === "local") {
				continue;
			}

			for (const encoding of encodings) {
				cells.push({
					runtime,
					sqliteBackend,
					encoding,
				});
			}
		}
	}

	return cells;
}

export function describeDriverMatrix(
	suiteName: string,
	defineTests: (driverTestConfig: DriverTestConfig) => void,
	options: DriverMatrixOptions = {},
) {
	const registryVariantNames = new Set(options.registryVariants);
	const variants = getDriverRegistryVariants(TEST_DIR).filter(
		(variant) =>
			registryVariantNames.size === 0 ||
			registryVariantNames.has(variant.name),
	);
	const cells = getDriverMatrixCells(options);
	const includeSqliteDimensions =
		hasEnvMatrixOverride() ||
		options.runtimes !== undefined ||
		options.sqliteBackends !== undefined;

	describeDriverSuite(suiteName, () => {
		for (const variant of variants) {
			if (variant.skip) {
				describe.skip(`${variant.name} registry`, () => {});
				continue;
			}

			describeDriverSuite(`${variant.name} registry`, () => {
				afterAll(async () => {
					await releaseSharedEngine();
				});

				for (const cell of cells) {
					const suite = includeSqliteDimensions
						? `runtime (${cell.runtime}) / sqlite (${cell.sqliteBackend}) / encoding (${cell.encoding})`
						: `encoding (${cell.encoding})`;

					if (cell.skipReason) {
						describe.skip(`${suite}: ${cell.skipReason}`, () => {});
						continue;
					}

					describeDriverSuite(suite, () => {
						if (cell.runtime === "native") {
							defineTests(
								createNativeDriverTestConfig({
									variant,
									encoding: cell.encoding,
									sqliteBackend: cell.sqliteBackend,
									...options.config,
								}),
							);
						} else {
							defineTests(
								createWasmDriverTestConfig({
									variant,
									encoding: cell.encoding,
									...options.config,
								}),
							);
						}
					});
				}
			});
		}
	});
}
