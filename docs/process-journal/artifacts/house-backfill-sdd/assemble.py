import json

OUT = "/tmp/claude-1000/-home-senol-agents-peter/f3a0b645-51c4-44b5-a80c-00a967d4d4d7/tasks/wh18cm78h.output"
d = json.load(open(OUT))["result"]

conf = d["promotionsConfirmed"]      # full clusters + audit
demo = d["promotionsDemoted"]        # {id, statement, verified_count, note}
tier1 = d["tier1"]                   # full clusters, not promoted
counts = d["counts"]

def q(s):
    s = " ".join(str(s).split())
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'

def occ_line(o):
    ev = o.get("evidence")
    ref = q(o.get("ref", ""))
    return f'    - {{date: "{o.get("date","")}", kind: {o.get("kind","")}, ref: {ref}}}'

def entry_yaml(c, tier, count, promoted_at):
    L = []
    L.append(f'- id: {c.get("id","")}')
    L.append(f'  kind: {c.get("kind","")}')
    L.append(f'  tier: {tier}')
    L.append(f'  domain: {c.get("domain","")}')
    L.append(f'  statement: {q(c.get("statement",""))}')
    L.append(f'  steelman: {q(c["steelman"]) if c.get("steelman") else "null"}')
    L.append(f'  blocked_on: {q(c["blocked_on"]) if c.get("blocked_on") else "null"}')
    L.append(f'  status: {c.get("status","settled")}')
    L.append(f'  promoted_at: {promoted_at if promoted_at is not None else "null"}')
    L.append(f'  count: {count}')
    occs = c.get("occurrences", [])
    if occs:
        L.append('  occurrences:')
        for o in occs:
            L.append(occ_line(o))
    else:
        L.append('  occurrences: []')
    return "\n".join(L)

# ---------- decision-ledger.yaml (Tier 1) ----------
hdr = """# Muxsmith decision ledger (Tier 1) - reconstructed bootstrap
#
# Every considered approach and its outcome; always recorded, on-demand load.
# Entry schema (canonical copy in software-dev-process doctrine section 7):
#   id, kind (pattern|restraint|non-decision), tier (1|2), domain,
#   statement, steelman (restraint only), blocked_on (non-decision only),
#   status (settled|contested|blocked), promoted_at, count (== len(occurrences)),
#   occurrences: [{date, kind (decided|reinforced|violated-corrected|deferred), ref}]
# count is always len(occurrences); no occurrence, no increment. Promotion to
# CONVENTIONS.yaml at count 3; the occurrence list migrates and keeps growing.
#
# Bootstrapped 2026-07-13 by a retroactive reconstruction sweep over the whole
# project trail (git, 8 D-memos, the 10-session journal, plan-1..5.5 SDD verdict
# archives, ROADMAP/IDEAS/spec), walked chronologically. 549 occurrence records
# -> 358 clusters. Tier-1 counts here are RECONSTRUCTION-STAGE (not individually
# audited); only the promoted entries (-> CONVENTIONS.yaml) passed the
# adversarial per-occurrence audit that drops fabricated/duplicate citations.
"""

led = [hdr, "\nentries:"]
for c in tier1:
    led.append(entry_yaml(c, 1, c.get("count", len(c.get("occurrences", []))), None))
# demoted: abbreviated (kind/domain/occurrences not in the return payload)
led.append("\n# --- Audit-demoted promotion candidates (count fell below 3 after")
led.append("# occurrence audit). Full occurrences in .superpowers/sdd/house-backfill/")
led.append("# audit-<id>.md. Fold into the schema above on next touch. ---")
for x in demo:
    led.append(f'#   {x["id"]}  verified_count={x["verified_count"]}  "{ " ".join(x["statement"].split())[:90] }"')

open("/home/senol/Git/Muxsmith/docs/decision-ledger.yaml", "w").write("\n".join(led) + "\n")

# ---------- triage digest (48 promotions) ----------
byd = {}
for c in conf:
    byd.setdefault(c.get("domain","?"), []).append(c)

dig = []
dig.append(f"# House-knowledge promotion candidates - TRIAGE ({len(conf)})")
dig.append("Confirmed by the occurrence audit (>=3 verified, cited occurrences).")
dig.append("These become standing CONVENTIONS.yaml entries on your approval.\n")
for dom in sorted(byd, key=lambda k: -len(byd[k])):
    items = sorted(byd[dom], key=lambda c: -c["audit"]["verified_count"])
    dig.append(f"## {dom} ({len(items)})")
    for c in items:
        a = c["audit"]
        vc = a["verified_count"]
        red = f" [REDUCED from {c['count']}]" if a["verdict"]=="REDUCED" else ""
        st = " ".join(c["statement"].split())
        if len(st) > 160: st = st[:157] + "..."
        dig.append(f"- **{c['id']}** ({c['kind']}, count {vc}{red}) - {st}")
    dig.append("")

open("/home/senol/Git/Muxsmith/.superpowers/sdd/house-backfill/TRIAGE.md","w").write("\n".join(dig))

# ---------- stats ----------
print("records:", counts["records"], "clusters:", counts["clusters"])
print("tier1 written:", len(tier1), "| promotions confirmed:", len(conf), "| demoted:", len(demo))
kinds = {}
for c in conf: kinds[c["kind"]] = kinds.get(c["kind"],0)+1
print("promotion kinds:", kinds)
reduced = sum(1 for c in conf if c["audit"]["verdict"]=="REDUCED")
print("promotions CONFIRMED-as-is:", len(conf)-reduced, "| REDUCED by audit:", reduced)
print("ledger lines:", len(open('/home/senol/Git/Muxsmith/docs/decision-ledger.yaml').readlines()))
print("\n===== TRIAGE DIGEST =====\n")
print("\n".join(dig))
