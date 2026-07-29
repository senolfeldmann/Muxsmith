/**
 * The registry types the editor's field registries (`registries.ts`) are
 * built against (D45). The compiler's grip on completeness is the
 * `Record<keyof T, FieldSpec>` shape a registry is declared with, not this
 * file: `FieldSpec`/`FieldWidget`/`RegistryName` only give that shape
 * somewhere to point.
 */

/** A field the editor renders. */
export interface EditableField {
  labelKey: string;
  /** D54's annotated set only; value always === labelKey when present
   *  (naming convention, written out literally so D62's literal scan
   *  sees it - never derived). */
  helpId?: string;
  widget: FieldWidget;
}

/** A field that exists in the model and is deliberately not exposed.
 *  `why` is a source comment, not user prose: nothing renders it. */
export interface FixedField {
  readonly fixed: true;
  why: string;
}

export type FieldSpec = EditableField | FixedField;

/**
 * `Input.pattern` is a regex compiled directly; `TemplateBlock.template` is
 * a template in literal mode (spec 4.8); `Locator.match_pattern` is a
 * template in regex mode (spec 4.7) -- a genuinely third thing from either.
 * `Meta.*` is plain prose.
 */
export type TextSyntax = "plain" | "regex" | "templateLiteral" | "templateRegex";

/**
 * The name of one of the editor's 13 registries (one per edited struct),
 * used by `keywordOrBlock`/`list`/`section` to point at another registry
 * without depending on its module. Symmetric with the 13-struct table in
 * `registries.ts`: one name per struct, regardless of whether every struct
 * is actually pointed at by another widget.
 */
export type RegistryName =
  | "profile"
  | "meta"
  | "input"
  | "output"
  | "templateBlock"
  | "externalBlock"
  | "trackRule"
  | "locator"
  | "attachments"
  | "tracks"
  | "attachmentRule"
  | "tags"
  | "matchExpr";

/**
 * 10 variants, closed. `fixed` is not one of them: it is the other half of
 * the `FieldSpec` union above, which is the whole point of splitting them.
 */
export type FieldWidget =
  | { kind: "text"; syntax: TextSyntax; multiline: boolean }
  | { kind: "bool" }
  // checked -> Some(true), unchecked -> absent (`validate_locator` in
  // profile/validate.rs rejects Some(false) for `match_to_source`; not a
  // tri-state).
  | { kind: "optionalFlag" }
  | { kind: "select"; options: readonly string[] }
  | { kind: "keywordOrBlock"; keywords: readonly string[]; block: RegistryName }
  | { kind: "directoryPath"; optional: boolean }
  | { kind: "stringList" }
  | { kind: "propertyMap"; properties: "matchable" | "settable"; values: "scalar" | "string" }
  | { kind: "list"; item: RegistryName; reorderable: boolean }
  | { kind: "section"; of: RegistryName; optional: boolean };
