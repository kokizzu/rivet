import { describe, expect, test } from "vitest";
import { describeDriverMatrix } from "./shared-matrix";
import { setupDriverTest } from "./shared-utils";

const BYPASS_HEADER = "x-rivet-bypass-connectable";
const BYPASS_PROTOCOL = "rivet_bypass_connectable";

function websocketProtocols(headers: Record<string, string>): string[] {
	return (headers["sec-websocket-protocol"] ?? "")
		.split(",")
		.map((protocol) => protocol.trim())
		.filter(Boolean);
}

describeDriverMatrix("Gateway Bypass Client", (driverTestConfig) => {
	describe("Gateway Bypass Client", () => {
		test("action calls can enable and disable gateway bypass", async (c) => {
			const { client } = await setupDriverTest(c, driverTestConfig);

			const enabledView = client.requestAccessActor.getOrCreate([
				"action-bypass-enabled",
			]);
			const enabledTracking = client.requestAccessActor.getOrCreate(
				["action-bypass-enabled"],
				{ params: { trackRequest: true } },
			);

			await enabledTracking.action({
				name: "ping",
				args: [],
				gateway: { bypassConnectable: true },
			});

			const enabledInfo = await enabledView.getRequestInfo();
			expect(
				enabledInfo.onBeforeConnect.requestHeaders[BYPASS_HEADER],
			).toBe("1");

			const disabledView = client.requestAccessActor.getOrCreate([
				"action-bypass-disabled",
			]);
			const disabledTracking = client.requestAccessActor.getOrCreate(
				["action-bypass-disabled"],
				{ params: { trackRequest: true } },
			);

			await disabledTracking.action({
				name: "ping",
				args: [],
				gateway: { bypassConnectable: false },
			});

			const disabledInfo = await disabledView.getRequestInfo();
			expect(
				disabledInfo.onBeforeConnect.requestHeaders[BYPASS_HEADER],
			).toBeUndefined();
		});

		test("client gateway bypass default can be overridden per action", async (c) => {
			const { client } = await setupDriverTest(c, driverTestConfig, {
				client: { gateway: { bypassConnectable: true } },
			});

			const defaultView = client.requestAccessActor.getOrCreate([
				"client-action-bypass-default",
			]);
			const defaultTracking = client.requestAccessActor.getOrCreate(
				["client-action-bypass-default"],
				{ params: { trackRequest: true } },
			);

			await defaultTracking.ping();

			const defaultInfo = await defaultView.getRequestInfo();
			expect(
				defaultInfo.onBeforeConnect.requestHeaders[BYPASS_HEADER],
			).toBe("1");

			const overrideView = client.requestAccessActor.getOrCreate([
				"client-action-bypass-override",
			]);
			const overrideTracking = client.requestAccessActor.getOrCreate(
				["client-action-bypass-override"],
				{ params: { trackRequest: true } },
			);

			await overrideTracking.action({
				name: "ping",
				args: [],
				gateway: { bypassConnectable: false },
			});

			const overrideInfo = await overrideView.getRequestInfo();
			expect(
				overrideInfo.onBeforeConnect.requestHeaders[BYPASS_HEADER],
			).toBeUndefined();
		});

		test("connect can enable gateway bypass for its websocket", async (c) => {
			const { client } = await setupDriverTest(c, driverTestConfig);

			const defaultConn = client.requestAccessActor
				.getOrCreate(["connect-bypass-default"], {
					params: { trackRequest: true },
				})
				.connect();

			const defaultInfo = await defaultConn.getRequestInfo();
			expect(
				websocketProtocols(defaultInfo.onBeforeConnect.requestHeaders),
			).not.toContain(BYPASS_PROTOCOL);
			await defaultConn.dispose();

			const bypassConn = client.requestAccessActor
				.getOrCreate(["connect-bypass-enabled"], {
					params: { trackRequest: true },
				})
				.connect(undefined, {
					gateway: { bypassConnectable: true },
				});

			const bypassInfo = await bypassConn.getRequestInfo();
			expect(
				websocketProtocols(bypassInfo.onBeforeConnect.requestHeaders),
			).toContain(BYPASS_PROTOCOL);
			await bypassConn.dispose();
		});

		test("client gateway bypass default can be overridden per connect", async (c) => {
			const { client } = await setupDriverTest(c, driverTestConfig, {
				client: { gateway: { bypassConnectable: true } },
			});

			const defaultConn = client.requestAccessActor
				.getOrCreate(["client-connect-bypass-default"], {
					params: { trackRequest: true },
				})
				.connect();

			const defaultInfo = await defaultConn.getRequestInfo();
			expect(
				websocketProtocols(defaultInfo.onBeforeConnect.requestHeaders),
			).toContain(BYPASS_PROTOCOL);
			await defaultConn.dispose();

			const overrideConn = client.requestAccessActor
				.getOrCreate(["client-connect-bypass-override"], {
					params: { trackRequest: true },
				})
				.connect(undefined, {
					gateway: { bypassConnectable: false },
				});

			const overrideInfo = await overrideConn.getRequestInfo();
			expect(
				websocketProtocols(overrideInfo.onBeforeConnect.requestHeaders),
			).not.toContain(BYPASS_PROTOCOL);
			await overrideConn.dispose();
		});
	});
});
