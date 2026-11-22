import assert from "node:assert/strict";
import { chromium } from "playwright-core";

const base = new URL(process.argv[2] ?? "http://localhost:8787");
const browser = await chromium.launch({ channel: "chrome", headless: true });
try {
  const page = await browser.newPage({ baseURL: base.origin });
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));

  await page.goto("/");
  await page.getByRole("link", { name: "Explore the plan" }).click();
  await page.waitForURL("**/plan/");
  assert.equal(await page.locator(".shelf a").count(), 71);
  await page.goto("/");
  for (const [index, force] of ["165 N", "160 N", "150 N"].entries()) {
    const button = page.locator('[data-stance="' + index + '"]');
    await button.click();
    assert.equal(await button.getAttribute("aria-pressed"), "true");
    assert.equal(await page.locator("#force").innerText(), force);
  }
  for (const step of await page.locator(".studio-steps summary").all()) {
    if (!(await step.evaluate((el) => (el.parentElement as HTMLDetailsElement).open)))
      await step.click();
    assert(await step.locator("..").locator("pre").isVisible());
  }

  await page.keyboard.press("/");
  await page.waitForURL("**/search/");
  await page.getByRole("searchbox").fill("Cayley");
  const first = page.locator("#results a").first();
  await first.waitFor();
  assert((await page.locator("#results mark").count()) > 0);
  const before = await page.locator("#results a").count();
  await page.getByRole("button", { name: "Show more chapters" }).click();
  await page.waitForFunction(
    (count) => document.querySelectorAll("#results a").length > count,
    before,
  );
  const resultPath = await first.getAttribute("href");
  assert(resultPath?.startsWith("/plan/"));
  await first.click();
  await page.waitForURL((url) => url.pathname.startsWith("/plan/"));
  if (new URL(page.url()).hash) {
    assert(
      await page.evaluate(
        () => !!document.getElementById(decodeURIComponent(location.hash.slice(1))),
      ),
    );
  }
  await page.goBack();
  assert.equal(await page.getByRole("searchbox").inputValue(), "Cayley");
  await page.getByRole("searchbox").fill('"jzqxvfjzqxvf"');
  await page.getByRole("status").filter({ hasText: "No matching chapters" }).waitFor();

  await page.setViewportSize({ width: 390, height: 844 });
  for (const path of ["/", "/plan/", "/plan/25/", "/plan/appendix-m/", "/search/"]) {
    await page.goto(path);
    await page.evaluate(() => document.fonts.ready);
    assert(
      await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
      "Horizontal overflow at " + path,
    );
  }
  assert.deepEqual(errors, []);
  console.log(
    "Verified the library, stance controls, studio walkthrough, search, result navigation and mobile layout at " +
      base.origin,
  );
} finally {
  await browser.close();
}
