/**
 * Shared, non-component plumbing for the ten `FieldWidget` components
 * (D45): the per-kind prop-narrowing helper and the `RegistryName ->
 * registry` lookup `SectionWidget`/`ListWidget`/`KeywordOrBlockWidget` need
 * to recurse into a nested struct's own registry without depending on
 * which registry module owns it.
 */
import type { FieldSpec, FieldWidget, RegistryName } from "../fieldSpec";
import {
  attachmentRuleFields,
  attachmentsFields,
  externalBlockFields,
  inputFields,
  locatorFields,
  matchExprFields,
  metaFields,
  outputFields,
  profileFields,
  tagsFields,
  templateBlockFields,
  trackRuleFields,
  tracksFields,
} from "../registries";

/**
 * An `EditableField` narrowed to one `FieldWidget` variant, e.g.
 * `EditableFieldOf<"text">` has `widget: { kind: "text"; syntax:
 * TextSyntax; multiline: boolean }`. Each widget component's `spec` prop
 * uses this so the component only ever sees the shape it renders, while
 * the dispatcher (which must hold every kind) stays on the union.
 */
export type EditableFieldOf<K extends FieldWidget["kind"]> = {
  labelKey: string;
  widget: Extract<FieldWidget, { kind: K }>;
};

/**
 * The 13 registries (Task 9, `registries.ts`), keyed by the `RegistryName`
 * a `section`/`list`/`keywordOrBlock` widget's `of`/`item`/`block` field
 * names. One entry per `RegistryName` member -- if a 14th registry is ever
 * added, `RegistryName` grows and this object literal fails to satisfy
 * `Record<RegistryName, ...>` until the entry is added here too.
 */
export const registryByName: Record<RegistryName, Record<string, FieldSpec>> = {
  profile: profileFields,
  meta: metaFields,
  input: inputFields,
  output: outputFields,
  templateBlock: templateBlockFields,
  externalBlock: externalBlockFields,
  trackRule: trackRuleFields,
  locator: locatorFields,
  attachments: attachmentsFields,
  tracks: tracksFields,
  attachmentRule: attachmentRuleFields,
  tags: tagsFields,
  matchExpr: matchExprFields,
};
