import * as log from "@std/log";
//import { getEnv as crossGetEnv } from "@cross/env";
import { LOGGER_NAME as CLIENT } from "../../client/src/log.ts";
import { LOGGER_NAME as MANAGER } from "../../manager/src/log.ts";
import {
	INSTANCE_LOGGER_NAME as INSTANCE,
	LOGGER_NAME as RUNTIME,
} from "../../runtime/src/log.ts";
import { type LogEntry, castToLogValue, stringify } from "./logfmt.ts";

export function getLogger(name: string): log.Logger {
	return log.getLogger(name);
}

export function setupLogging() {
	const loggerConfig: log.LoggerConfig = {
		level: (getEnv("LOG_LEVEL") as log.LevelName) ?? "DEBUG",
		handlers: ["default"],
	};

	log.setup({
		handlers: {
			default: new log.ConsoleHandler("INFO", {
				formatter,
				useColors: false,
			}),
		},
		// Enable logging for all actor SDKs
		loggers: {
			default: loggerConfig,
			[CLIENT]: loggerConfig,
			[MANAGER]: loggerConfig,
			[RUNTIME]: loggerConfig,
			[INSTANCE]: loggerConfig,
		},
	});
}

function formatter(log: log.LogRecord): string {
	const args: LogEntry[] = [];
	for (let i = 0; i < log.args.length; i++) {
		const logArg = log.args[i];
		if (logArg && typeof logArg === "object") {
			// Spread object
			for (const k in logArg) {
				// biome-ignore lint/suspicious/noExplicitAny: Unknown type
				const v = (logArg as any)[k];

				pushArg(k, v, args);
			}
		} else {
			pushArg(`arg${i}`, logArg, args);
		}
	}

	return stringify(
		//["ts", formatTimestamp(log.datetime)],
		["level", log.levelName],
		//["target", log.loggerName],
		["msg", log.msg],
		...args,
	);
}

function pushArg(k: string, v: unknown, args: LogEntry[]) {
	args.push([k, castToLogValue(v)]);
}

function getEnv(name: string): string | undefined {
	if (typeof window !== "undefined" && window.localStorage) {
		return window.localStorage.getItem(name) || undefined;
	} else {
		return undefined;
		// TODO(ACTR-9): Add back env config once node compat layer works
		//return crossGetEnv(name);
	}
}