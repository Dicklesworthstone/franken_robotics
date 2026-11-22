import { readFileSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import assert from "node:assert/strict";
import { origin } from "./layout";

const root = resolve(import.meta.dir, "..");
const out = join(root, "out");
const source = readFileSync(join(root, "../COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md"));
assert(
  source.equals(readFileSync(join(out, "plan.md"))),
  "The Markdown download differs from the source.",
);

const pages = new Map<string, { links: Set<string>; ids: Set<string> }>();
let chapters = 0;
for (const path of new Bun.Glob("**/*.html").scanSync({ cwd: out })) {
  const url = "/" + path.replace(/index\.html$/, "");
  const links = new Set<string>();
  const ids = new Set<string>();
  let titles = 0;
  const page = new HTMLRewriter()
    .on("[id]", {
      element(el) {
        const id = el.getAttribute("id")!;
        assert(!ids.has(id), url + " repeats #" + id);
        ids.add(id);
      },
    })
    .on("[href], [src]", {
      element(el) {
        links.add((el.getAttribute("href") ?? el.getAttribute("src"))!);
      },
    })
    .on("h1", {
      element() {
        titles++;
      },
    })
    .on("[data-pagefind-body]", {
      element() {
        chapters++;
      },
    })
    .transform(new Response(Bun.file(join(out, path))));
  await page.text();
  assert.equal(titles, 1, url + " needs one document title.");
  pages.set(url, { links, ids });
}

let checked = 0;
for (const [sourcePath, { links }] of pages) {
  for (const link of links) {
    const dest = new URL(link, origin + sourcePath);
    if (dest.origin !== origin) continue;
    const found = pages.get(dest.pathname);
    // The canonical on 404.html names the error route, which returns this same page.
    if (sourcePath === "/404.html" && dest.pathname === "/404/") continue;
    if (found && dest.hash) {
      assert(
        found.ids.has(decodeURIComponent(dest.hash.slice(1))),
        sourcePath + " links to missing " + dest.href,
      );
    } else if (!found) {
      assert(
        statSync(join(out, decodeURIComponent(dest.pathname)), { throwIfNoEntry: false })?.isFile(),
        sourcePath + " links to missing " + dest.href,
      );
    }
    checked++;
  }
}
const search = JSON.parse(readFileSync(join(out, "pagefind/pagefind-entry.json"), "utf8"));
assert.equal(search.languages.en.page_count, chapters, "Search missed reader entries.");
assert.equal(chapters, 71, "Review chapter coverage for this plan edition.");
console.log(
  "Checked " +
    chapters +
    " indexed reader entries, " +
    checked +
    " internal references and the exact Markdown download.",
);
