import { faExclamationTriangle, Icon } from "@rivet-gg/icons";
import { useQuery } from "@tanstack/react-query";
import { Link, useMatch } from "@tanstack/react-router";
import { Button, cn } from "@/components";
import { useCloudProjectDataProvider } from "@/components/actors";
import { PLAN_LABELS } from "@/content/billing";
import { features } from "@/lib/features";
import { useHighestUsagePercent } from "./hooks";

export function BillingLimitAlert() {
	if (!features.billing) return null;
	return <BillingLimitAlertGuard />;
}

// Billing is project-scoped, but this banner renders from the shared route
// layout. `useMatch` with `shouldThrow: false` keeps it out of routes where
// `useCloudProjectDataProvider` would throw, and waiting on `loaderData`
// avoids reading the provider before the project loader resolves.
function BillingLimitAlertGuard() {
	const projectMatch = useMatch({
		from: "/_context/orgs/$organization/projects/$project",
		shouldThrow: false,
	});

	if (!projectMatch?.loaderData) return null;

	return <BillingLimitAlertInner />;
}

function BillingLimitAlertInner() {
	const dataProvider = useCloudProjectDataProvider();
	const { data: billingData } = useQuery({
		...dataProvider.currentProjectBillingDetailsQueryOptions(),
	});

	const usagePercent = useHighestUsagePercent();
	const plan = billingData?.billing.activePlan || "free";

	if (plan !== "free" || usagePercent < 80) {
		return null;
	}

	const atLimit = usagePercent >= 100;

	return (
		<div
			className={cn(
				"flex items-center gap-2 border-b px-3 py-1.5 text-xs",
				atLimit
					? "border-destructive/60 bg-destructive/15"
					: "border-warning/60 bg-warning/10",
			)}
		>
			<Icon
				icon={faExclamationTriangle}
				className={cn(
					"shrink-0",
					atLimit ? "text-destructive" : "text-warning",
				)}
			/>
			<p className="text-foreground font-medium">
				{atLimit
					? `${PLAN_LABELS[plan] ?? "Plan"} plan limit reached`
					: "Approaching your plan limit"}
			</p>
			<p className="text-muted-foreground min-w-0 truncate">
				{atLimit
					? "Upgrade your plan to avoid service interruptions."
					: `You have used ${usagePercent}% of your plan's free usage.`}
			</p>
			<Button
				size="sm"
				variant="ghost"
				className="ml-auto h-6 shrink-0 text-xs"
				asChild
			>
				<Link
					from="/orgs/$organization/projects/$project"
					to="/orgs/$organization/projects/$project/billing"
				>
					Upgrade
				</Link>
			</Button>
		</div>
	);
}
