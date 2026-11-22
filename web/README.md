# FrankenRobotics.com

The site presents the research plan in `../COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md`. The home page explains it in six chapters with interactive figures. The reader at `/plan/` holds every section and appendix, cross-linked and searchable.

Nothing described on the site has been built. Each figure carries a badge that says what it is (computed here, a toy, a drawing, illustrative numbers, or quoted from the plan) and a title block that names its source section.

## Build and preview

Run these commands in `web/` with Bun installed.

```sh
bun install --frozen-lockfile
bun run dev
```

`bun run build` reads the plan, writes static pages to `out/`, indexes them with Pagefind, and then checks every internal link, the share cards, search coverage and the Markdown download. Pages can be read with JavaScript disabled. The story and construction catalogues are extracted from §7 and §10 while building, and the build stops if either section changes shape.

`bun run test` type-checks and runs the unit tests of the figures. With Google Chrome installed and a preview running, `bun run smoke` mounts every figure and exercises the controls, the palette, search, the share cards and the phone layout. Supply a URL to check a deployment.

## Layout

| Path | Holds |
| --- | --- |
| `src/plan.ts`, `src/reader.ts` | The plan parsed into pages, and the reader around them |
| `src/home.ts`, `src/chapters/` | The home page, rendered to HTML |
| `src/figures/` | One module per figure, each with a `mount(root)` and its arithmetic exported for tests |
| `src/client/home.ts` | Loads a figure when it nears the screen |
| `src/catalogue.ts` | Stories and constructions, read from the plan |
| `src/stills.ts` | The hero posters and the share cards |
| `static/` | Stylesheets, the script every page shares, and the icon |

## Share cards

`media/share/` holds a 1200 × 630 card for every page and a 1200 × 600 version of the home card. They are photographed in Chrome from the site's own figures:

```sh
bun run build
bun run stills          # what is missing or whose text has changed
bun run stills --all    # everything, after a figure has changed
```

Their URLs carry a content hash. They point at frankenrobotics.com unless `SITE_ORIGIN` names another host when the site is built, which a deployment on another host needs for its previews to load.

## Publish

`bun run deploy` rebuilds the site and publishes it to the Cloudflare Worker configured in `wrangler.jsonc`, which serves frankenrobotics.com and redirects www to it.

The Vercel deployment at frankenrobotics.vercel.app is a prebuilt upload. Build with `SITE_ORIGIN=https://frankenrobotics.vercel.app`, then run `bun run vercel-output <directory>` to write the Build Output API layout and `vercel deploy --prebuilt --prod` in that directory once it is linked to the project.
