import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

/**
 * A single-segment URL that is not a routed locale — a stray dotted path such
 * as /foo.txt, which middleware deliberately leaves alone so real static files
 * keep resolving — binds `[locale]` to that segment and would otherwise render
 * the shared home page as a 200 under a fake locale (`lang="foo.txt"`): a
 * soft 404 for crawlers and generative-engine probes. The layout must turn any
 * locale outside the routed registry into a real 404 before it renders.
 */
describe("locale layout rejects unregistered locales", () => {
  const layout = readFileSync(new URL("../../app/[locale]/layout.tsx", import.meta.url), "utf8");

  it("guards on isValidLocale and calls notFound()", () => {
    expect(layout).toContain("import { isValidLocale, localeDirection, locales, type Locale }");
    expect(layout).toContain("if (!isValidLocale(locale)) notFound();");
  });

  it("checks the locale before reading dictionaries or rendering chrome", () => {
    const guard = layout.indexOf("isValidLocale(locale)");
    const chrome = layout.indexOf("getChrome(locale)");
    expect(guard).toBeGreaterThan(-1);
    expect(chrome).toBeGreaterThan(-1);
    expect(guard).toBeLessThan(chrome);
  });
});
