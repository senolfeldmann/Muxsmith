import { marked } from "marked";
import { primarySubtag } from "../i18n";

/** Help topics, eagerly embedded at build time - byte-for-byte the
 *  pattern src/i18n/index.ts established for .ftl (D51): offline by
 *  construction, atomic with the code that references the ids. */
const topicSources = import.meta.glob("../../help/*/*.md", {
  query: "?raw",
  import: "default",
  eager: true,
}) as Record<string, string>;

const TOPIC_PATH = /\/help\/([^/]+)\/([^/]+)\.md$/;

function sourceFor(helpId: string, localeDir: string): string | null {
  for (const [path, source] of Object.entries(topicSources)) {
    const m = TOPIC_PATH.exec(path);
    if (m && m[1] === localeDir && m[2] === helpId) {
      return source;
    }
  }
  return null;
}

/** Renders one topic (D50: marked, defaults, first-party input only).
 *  Fallback per topic: locale -> en -> the raw help-id as visible text
 *  (the Renderer::msg raw-id posture; only reachable in a build that
 *  dodged CI's D62 gate), never a silent blank. */
export function topicHtml(helpId: string, locale: string): string {
  const source =
    sourceFor(helpId, primarySubtag(locale)) ?? sourceFor(helpId, "en");
  if (source === null) {
    return helpId;
  }
  return marked.parse(source, { async: false });
}
