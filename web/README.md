# FrankenRobotics.com

The site presents the project’s final hybrid research plan. The homepage adapts FrankenLife’s dark instrument panels, green typography and worked examples to robot design. The quadruped illustration is concept art. The stance example uses the static loading assumptions and exact force allocations from §25.13. The CLI walkthrough quotes the proposed interface in §32.16.

## Build and preview

Run these commands in `web/` with Bun installed.

```sh
bun install --frozen-lockfile
bun run dev
```

`bun run build` reads `../COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md`, assembles the chapter reader and full-text index in `out/`, and checks links, search coverage and the Markdown download. All chapters are static HTML and can be read with JavaScript disabled. Search runs in the browser against a locally served Pagefind index.

With the preview running and Google Chrome installed, `bun run smoke` exercises the worked example, studio walkthrough, search and mobile layout. Supply a URL to run the same checks against a deployment.

## Publish

`bun run deploy` rebuilds the site and publishes the static assets and hostname redirect to the Cloudflare Worker configured in `wrangler.jsonc`. Its custom domains are frankenrobotics.com and www.frankenrobotics.com. The latter redirects to the same path and query on the apex.
