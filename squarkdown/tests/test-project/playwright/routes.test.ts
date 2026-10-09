// reused with modifications from:
// <https://github.com/Sup2point0/integrity/blob/main/site/tests/routes.test.ts>

import { test, chromium, expect } from "@playwright/test";


type url = string;

const WHITE = `\x1b[0m`;
const BLACK = `\x1b[30m`;
const GREY  = `\x1b[90m`;

const PINK   = `\x1b[95m`;
const RED    = `\x1b[31m`;
const YELLOW = `\x1b[93m`;
const GREEN  = `\x1b[92m`;
const CYAN   = `\x1b[96m`;
const BLUE   = `\x1b[94m`;


let browser = await chromium.launch();

test("crawl-routes", async () =>
{
	test.setTimeout(60 * 1000);

	let visited = new Set<url>();
	let failed_routes: url[] = [];

	let to_visit = [
		{ url: "/", source: "." },
	];

	let pending_routes = 0;
	let routes_visited = 0;
	let questions_visited = 0;

	await Promise.allSettled(Array.from({ length: 4 }, (_, i) =>
		(async () => {
			await sleep(i * 1000);

			const WORKER = `${PINK}[worker ${i+1}]${WHITE}`;
			console.info(`${GREY}>> ${WORKER} ${YELLOW}Scanning for links...`);

			let ctx = await browser.newContext();
			let page = await ctx.newPage();

			while (true) {
				let next = to_visit.pop();

				if (next == undefined) {
					if (pending_routes === 0) break;
					
					/* NOTE: Other workers might push more targets to the queue, so we'll wait a hot sec before deciding to finish */
					await sleep(1000);

					next = to_visit.pop();
					if (next == undefined) break;
				}

				let { url: target, source } = next;

				if (visited.has(target)) continue;
				visited.add(target);

				if (questions_visited > 10 && target.includes("?shard=")) continue;
				if (target.includes("?shard=")) questions_visited++;

				try {
					console.info(`${GREY}-- ${WORKER} Visiting: ${CYAN}${u(target)}`);

					pending_routes++;
					routes_visited++;

					let response = await page.goto(target, { waitUntil: "networkidle" });
					if (!response?.ok()) throw Error(response?.status().toString());

					let links = await page.locator("a").evaluateAll(
						anchors => anchors
							.map(a => !a.disabled && a.href)
							.filter(url => url != undefined && url.includes("://localhost:") && !url.includes("#"))
					);
					to_visit.push(...links.map(url => ({ url, source: target })));
				}
				catch (e) {
					console.error(`${RED}!! ${WORKER} Failed: ${CYAN}${u(target)}${WHITE}; linked from: ${BLUE}${u(source)}${RED}`);
					console.error(e.message);

					failed_routes.push(target);
				}

				pending_routes--;
			}

			await ctx.close();

			console.info(`${GREY}>> ${WORKER} ${YELLOW}Finished scanning.`);
		})()
	));

	if (failed_routes.length > 0) {
		console.error(
			RED,
			`\n—————————————————————————————————————————————————————————————————————`,
			`\n!! Failed to access ${failed_routes.length}/${routes_visited} routes:`,
			`\n\n   `,
			failed_routes.join("\n    ")
		);
	}

	expect(failed_routes.length).toBe(0);
});


async function sleep(ms: number): Promise<void>
{
	await new Promise(r => setTimeout(r, ms));
}

function u(url: string): string
{
	return url.replace("http://localhost:4173", "");
}
