import {
	faAws,
	faCloudflare,
	faGoogleCloud,
	faHetznerH,
	faKubernetes,
	faRailway,
	faRivet,
	faRocket,
	faServer,
	faSupabase,
	faVercel,
} from "@rivet-gg/icons";

export type Provider =
	| "rivet"
	| "vercel"
	| "cloudflare-workers"
	| "supabase-functions"
	| "railway"
	| "kubernetes"
	| "aws-ecs"
	| "gcp-cloud-run"
	| "hetzner"
	| "custom"
	| "custom-platform";

export interface DeployOption {
	displayName: string;
	name: Provider;
	shortTitle?: string;
	href: string;
	description: string;
	icon?: any;
	badge?: string;
	/** If true, this platform should NOT be shown for generic deploy guides for Node/Bun-specific platforms. */
	specializedPlatform?: boolean;
}

export const deployOptions: DeployOption[] = [
	{
		displayName: "Rivet Compute",
		name: "rivet",
		href: "/docs/deploy/rivet-compute",
		description:
			"Deploy to Rivet's managed compute platform",
		icon: faRivet as any,
	},
	{
		displayName: "Vercel",
		name: "vercel",
		href: "/docs/deploy/vercel",
		description: "Deploy Next.js + RivetKit apps to Vercel's edge network",
		icon: faVercel as any,
	},
	{
		displayName: "Cloudflare Workers",
		shortTitle: "Cloudflare",
		name: "cloudflare-workers",
		href: "/docs/deploy/cloudflare",
		description:
			"Run RivetKit on Cloudflare Workers with the WebAssembly runtime",
		icon: faCloudflare as any,
		specializedPlatform: true,
	},
	{
		displayName: "Supabase Functions",
		shortTitle: "Supabase",
		name: "supabase-functions",
		href: "/docs/deploy/supabase",
		description:
			"Run RivetKit on Supabase Edge Functions with the WebAssembly runtime",
		icon: faSupabase,
		specializedPlatform: true,
	},
	{
		displayName: "Railway",
		name: "railway",
		href: "/docs/deploy/railway",
		description: "Deploy containers to Railway's managed infrastructure",
		icon: faRailway as any,
	},
	{
		displayName: "Kubernetes",
		name: "kubernetes",
		href: "/docs/deploy/kubernetes",
		description: "Deploy to any Kubernetes cluster with container images",
		icon: faKubernetes as any,
	},
	{
		displayName: "AWS ECS",
		shortTitle: "AWS",
		name: "aws-ecs",
		href: "/docs/deploy/aws-ecs",
		description:
			"Run containerized workloads on Amazon Elastic Container Service",
		icon: faAws as any,
	},
	{
		displayName: "Google Cloud Run",
		shortTitle: "GCP",
		name: "gcp-cloud-run",
		href: "/docs/deploy/gcp-cloud-run",
		description: "Deploy containers to Google Cloud Run for auto-scaling",
		icon: faGoogleCloud,
	},
	{
		displayName: "Hetzner",
		name: "hetzner",
		href: "/docs/deploy/hetzner",
		description: "Deploy to Hetzner's cost-effective cloud infrastructure",
		icon: faHetznerH as any,
	},
	{
		displayName: "VM & Bare Metal",
		name: "custom",
		shortTitle: "VM",
		href: "/docs/deploy/vm-and-bare-metal",
		description:
			"Run on virtual machines or bare metal servers with full control",
		icon: faServer as any,
	},
	{
		displayName: "Custom Platform",
		name: "custom-platform",
		href: "/docs/deploy/custom",
		description:
			"Integrate RivetKit with any other hosting platform of your choice",
		icon: faRocket as any,
	},
];
