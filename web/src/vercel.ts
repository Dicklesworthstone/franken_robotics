/** Writes out/ as Vercel's Build Output API v3 under <directory>/.vercel/output: bun run vercel-output <directory> */
import { cpSync, existsSync, mkdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const target = process.argv[2];
if (!target) throw Error("Name the staging directory.");
const out = resolve(import.meta.dir, "../out");
const root = join(resolve(target), ".vercel/output");
if (existsSync(root)) throw Error(`${root} exists. Use an empty staging directory.`);
mkdirSync(join(root, "static"), { recursive: true });
cpSync(out, join(root, "static"), { recursive: true, filter: (from) => !from.includes("/.art") });

// Pages live in directories, and their addresses end in a slash.
const routes: object[] = [
  {
    src: "/(.*)",
    headers: {
      "X-Content-Type-Options": "nosniff",
      "Referrer-Policy": "strict-origin-when-cross-origin",
      "Permissions-Policy": "camera=(), microphone=(), geolocation=()",
    },
    continue: true,
  },
  { src: "/(assets|fonts)/(.*)", headers: { "Cache-Control": "public, max-age=31536000, immutable" }, continue: true },
  { src: "/media/(.*)", headers: { "Cache-Control": "public, max-age=86400" }, continue: true },
];
const pages = [...new Bun.Glob("**/index.html").scanSync({ cwd: out })]
  .map((p) => "/" + p.replace(/index\.html$/, ""))
  .sort();
for (const url of pages) {
  const dest = `${url}index.html`;
  if (url === "/") routes.push({ src: "/", dest });
  else {
    const path = url.slice(0, -1).replace(/[.]/g, "\\.");
    routes.push({ src: path, status: 308, headers: { Location: url } }, { src: `${path}/`, dest });
  }
}
routes.push({ handle: "filesystem" }, { src: "/(.*)", status: 404, dest: "/404.html" });
writeFileSync(join(root, "config.json"), JSON.stringify({ version: 3, routes }, null, 1));
console.log(`${pages.length} pages and ${routes.length} routes in ${root}`);
