/**
 * Standalone home for the all-locales Fluent parse guard (moved out of
 * `i18n-en.ts`'s module-import side effect, T21 idiomacy fix): a real
 * catalog regression now fails one named red test instead of an opaque
 * module-load error surfacing in every spec that happens to import
 * `i18n-en.ts`. See that file's `assertAllCatalogsParseCleanly` doc for
 * what it checks and why `addResource`'s own error list is not enough.
 */
import { test } from "@playwright/test";
import { assertAllCatalogsParseCleanly } from "./i18n-en";

test("all Fluent catalogs parse cleanly", () => assertAllCatalogsParseCleanly());
