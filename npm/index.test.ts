import type { PageData } from "./index.d.ts";


function either(page_data: PageData)
{
	// @ts-expect-error
	page_data.heading;
}

function long(page_data: PageData<"long">)
{
	page_data.heading;

	// @ts-expect-error
	page_data.head;
}

function short(page_data: PageData<"short">)
{
	page_data.head;

	// @ts-expect-error
	page_data.heading;
}
