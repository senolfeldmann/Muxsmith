import { createApp } from "vue";
import { createFluentVue } from "fluent-vue";
import { FluentBundle, FluentResource } from "@fluent/bundle";
import App from "./App.vue";

// Fluent catalog loaded as a raw string (Vite `?raw`) rather than through
// vue-i18n's JSON/YAML message-catalog loader: Muxsmith standardizes on
// Fluent (.ftl) everywhere, CLI side included (see
// `crates/muxsmith-cli/src/i18n.rs`), so the GUI reads the same format
// instead of introducing a second one.
import guiCommonFtl from "../locales/en/gui-common.ftl?raw";

const bundle = new FluentBundle("en");
bundle.addResource(new FluentResource(guiCommonFtl));

const fluent = createFluentVue({
  bundles: [bundle],
});

createApp(App).use(fluent).mount("#app");
