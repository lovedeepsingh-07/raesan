import katex from "katex";
import { unified } from "unified";
import rehypeParse from "rehype-parse";
import rehypeStringify from "rehype-stringify";

export const render_math = (input: string) => {
	const with_math = input
		.replace(/\$\$([\s\S]+?)\$\$/g, (_, math) => {
			// const cleaned = math.replace(/<br\s*\/?>/gi, " \\\\ ");
			return katex.renderToString(math.trim(), { displayMode: false, throwOnError: false });
		})
		.replace(/\$([^\$]+?)\$/g, (_, math) => {
			// const cleaned = math.replace(/<br\s*\/?>/gi, " \\\\ ");
			return katex.renderToString(math.trim(), { displayMode: false, throwOnError: false });
		});

	const result = unified()
		.use(rehypeParse, { fragment: true })
		.use(rehypeStringify)
		.processSync(with_math);

	const output = String(result);
	return output;
};
