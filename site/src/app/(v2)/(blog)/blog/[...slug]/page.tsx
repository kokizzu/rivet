import { ArticleSocials } from "@/components/ArticleSocials";
import { Comments } from "@/components/Comments";
import { DocsTableOfContents } from "@/components/DocsTableOfContents";
import { Prose } from "@/components/Prose";
import {
	generateArticlesPageParams,
	loadArticle,
	loadArticles,
} from "@/lib/article";
import { formatTimestamp } from "@/lib/formatDate";
import { Avatar, AvatarFallback, AvatarImage } from "@rivet-gg/components";
import {
	Icon,
	faBluesky,
	faCalendarDay,
	faChevronRight,
	faGithub,
	faXTwitter,
} from "@rivet-gg/icons";
import clsx from "clsx";
import type { Metadata } from "next";
import Image from "next/image";
import Link from "next/link";
import type { CSSProperties } from "react";

export async function generateMetadata({
	params: { slug },
}): Promise<Metadata> {
	const { description, title, author, published, tags, category, image } =
		await loadArticle(slug.join("/"));

	return {
		title,
		description,
		authors: [
			{
				name: author.name,
				url: author.socials?.twitter || "",
			} satisfies Metadata["authors"],
		],
		keywords: tags,
		openGraph: {
			title,
			description,
			type: "article",
			publishedTime: new Date(published).toISOString(),
			authors: [author.name],
			section: category.name,
			tags,
			images: [
				{
					url: image.src,
					width: image.width,
					height: image.height,
					alt: image.alt,
				},
			],
		},
	};
}

export default async function BlogPage({ params: { slug } }) {
	const {
		Content,
		title,
		tableOfContents,
		author,
		published,
		category,
		image,
	} = await loadArticle(slug.join("/"));

	const isTechnical =
		category.name === "Technical" || category.name === "Guide";

	return (
		<>
			<div
				className={clsx(
					"mx-auto mt-20 w-full max-w-6xl px-4 md:mt-32",
					{
						"max-w-[1440px]": isTechnical,
					},
				)}
				style={{ "--header-height": "5rem" } as CSSProperties}
			>
				<ul className="text-muted-foreground my-4 flex flex-wrap items-center gap-2 text-xs">
					<li>
						<Link href="/blog">Blog</Link>
					</li>
					<li className="h-2.5">
						<Icon
							className="block h-full w-auto"
							icon={faChevronRight}
						/>
					</li>
					<li>{category.name}</li>
					<li className="h-2.5">
						<Icon
							className="block h-full w-auto"
							icon={faChevronRight}
						/>
					</li>
					<li className="text-foreground font-semibold">{title}</li>
				</ul>
				<div className="flex flex-col gap-8 pb-6 lg:flex-row">
					<aside className="order-1 mt-2 flex min-w-0 max-w-s flex-1 flex-col gap-2">
						<div className="top-header sticky pt-2">
							<p className="mb-2 text-sm font-semibold">
								Posted by
							</p>
							<div className="mb-4 flex items-center gap-2">
								<Avatar className="mt-1 size-14">
									<AvatarFallback>{author[0]}</AvatarFallback>
									<AvatarImage
										{...author.avatar}
										alt={author}
									/>
								</Avatar>
								<div className="flex flex-col">
									<h3 className="font-bold">{author.name}</h3>
									<p className="text-muted-foreground text-sm">
										{author.role}
									</p>
									<p className="flex gap-2 text-xs text-muted-foreground mt-1">
										{author.socials?.twitter ||
										author.url ? (
											<a
												href={
													author.socials?.twitter ||
													author.url
												}
												target="_blank"
												rel="noopener noreferrer"
											>
												<Icon icon={faXTwitter} />
											</a>
										) : null}
										{author.socials?.bluesky ? (
											<a
												href={author.socials.bluesky}
												target="_blank"
												rel="noopener noreferrer"
											>
												<Icon icon={faBluesky} />
											</a>
										) : null}
										{author.socials?.github ? (
											<a
												href={author.socials.github}
												target="_blank"
												rel="noopener noreferrer"
											>
												<Icon icon={faGithub} />
											</a>
										) : null}
									</p>
								</div>
							</div>
							<div className="text-muted-foreground mb-6 flex items-center gap-2 text-sm">
								<Icon icon={faCalendarDay} />
								<time className="text-sm">
									{formatTimestamp(published)}
								</time>
							</div>
							<p className="mb-2 hidden text-sm font-semibold lg:block">
								Other articles
							</p>
							<OtherArticles slug={slug[0]} />
						</div>
					</aside>
					<Prose
						as="article"
						className={clsx(
							"order-3 mt-4 w-full flex-shrink-0 lg:order-2",
							{
								"xl:max-w-[1000px]": isTechnical,
								"max-w-prose": !isTechnical,
							},
						)}
					>
						<Image
							{...image}
							alt="Promo Image"
							className="rounded-xl border border-white/10"
						/>
						<Content />
						<ArticleSocials title={title} />
					</Prose>
					<aside className="order-2 min-w-0 max-w-xs flex-1 lg:order-3">
						<DocsTableOfContents
							tableOfContents={tableOfContents}
						/>
					</aside>
				</div>
			</div>

			<Comments
				className={clsx("mx-auto w-full mb-8", {
					"xl:max-w-[1000px]": isTechnical,
					"max-w-prose": !isTechnical,
				})}
			/>
		</>
	);
}

export function generateStaticParams() {
	return generateArticlesPageParams();
}

async function OtherArticles({ slug }) {
	const articles = await loadArticles();

	return (
		<ul className="text-muted-foreground hidden list-disc pl-5 text-sm lg:block">
			{articles
				.filter((article) => article.slug !== slug)
				.sort((a, b) => b.published - a.published)
				.slice(0, 3)
				.map((article) => (
					<li key={article.slug} className="py-1">
						<Link href={article.category.id === "changelog" ? `/changelog/${article.slug}` : `/blog/${article.slug}`}>
							{article.title}
						</Link>
					</li>
				))}
		</ul>
	);
}
