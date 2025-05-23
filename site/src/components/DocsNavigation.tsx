import { ActiveLink } from "@/components/ActiveLink";
import { CollapsibleSidebarItem } from "@/components/CollapsibleSidebarItem";
import routes from "@/generated/routes.json";
import type { SidebarItem } from "@/lib/sitemap";
import { cn } from "@rivet-gg/components";
import { Icon, faArrowUpRight } from "@rivet-gg/icons";
import type { PropsWithChildren, ReactNode } from "react";

interface TreeItemProps {
	item: SidebarItem;
}

function TreeItem({ item }: TreeItemProps) {
	if (
		"collapsible" in item &&
		"title" in item &&
		"pages" in item &&
		item.collapsible
	) {
		return (
			<CollapsibleSidebarItem item={item}>
				<Tree pages={item.pages} />
			</CollapsibleSidebarItem>
		);
	}

	if ("title" in item && "pages" in item) {
		return (
			<div>
				<p className="mt-2 px-2 py-1 text-sm font-semibold">
					{item.icon ? (
						<Icon icon={item.icon} className="mr-2 size-3.5" />
					) : null}
					<span className="truncate"> {item.title}</span>
				</p>
				<Tree pages={item.pages} />
			</div>
		);
	}

	return (
		<NavLink href={item.href} external={item.external}>
			{item.icon ? (
				<Icon icon={item.icon} className="mr-2 size-3.5" />
			) : null}
			<span className="truncate">
				{item.title ?? routes.pages[getAliasedHref(item.href)]?.title}
			</span>
			{item.external ? (
				<Icon icon={faArrowUpRight} className="ml-2 size-3" />
			) : null}
		</NavLink>
	);
}

interface TreeProps {
	pages: SidebarItem[];
	className?: string;
}

export function Tree({ pages, className }: TreeProps) {
	return (
		<ul className={cn(className)}>
			{pages.map((item, index) => (
				<li
					// biome-ignore lint/suspicious/noArrayIndexKey: FIXME: used only for static content
					key={index}
					className="relative"
				>
					<TreeItem item={item} />
				</li>
			))}
		</ul>
	);
}

export function NavLink({
	href,
	external,
	children,
	className,
}: PropsWithChildren<{
	href: string;
	external?: boolean;
	children: ReactNode;
	className?: string;
}>) {
	return (
		<ActiveLink
			strict
			href={href}
			target={external && "_blank"}
			className={cn(
				"group flex w-full items-center rounded-md border border-transparent px-2 py-1 text-sm text-muted-foreground hover:underline aria-current-page:text-foreground",
				className,
			)}
		>
			{children}
		</ActiveLink>
	);
}

export function DocsNavigation({ sidebar }: { sidebar: SidebarItem[] }) {
	return (
		<div className="top-header sticky pr-4 text-white md:max-h-content md:overflow-y-auto md:pb-4 md:pt-8">
			<Tree pages={sidebar} />
		</div>
	);
}

export function getAliasedHref(href: string) {
	const [_, __, ...slug] = href.split("/");
	return `/docs/${slug.join("/")}`;
}
