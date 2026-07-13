import json, re

OUT = "/tmp/claude-1000/-home-senol-agents-peter/f3a0b645-51c4-44b5-a80c-00a967d4d4d7/tasks/wh18cm78h.output"
J = "/home/senol/.claude/projects/-home-senol-agents-peter/99378955-7efb-44a9-882c-67c303aeaed0/subagents/workflows/wf_7ff09182-796/journal.jsonl"

d = json.load(open(OUT))["result"]
tier1 = d["tier1"]
demo = d["promotionsDemoted"]

# ---- parse journal for full clusters + audits ----
clusters_by_id, audits_by_id = {}, {}
for line in open(J):
    line = line.strip()
    if not line: continue
    v = json.loads(line).get("result")
    if not isinstance(v, dict): continue
    if "clusters" in v:
        for c in v["clusters"]: clusters_by_id[c["id"]] = c
    elif "verdict" in v and "verified_count" in v:
        audits_by_id[v["id"]] = v

def toks(s): return set(t for t in re.findall(r"[A-Za-z0-9]{5,}", str(s or "")))
def score(a, b): return len(toks(a) & toks(b))

# ---- reconstruct the 15 demoted as full Tier-1 entries ----
demoted_full = []
for x in demo:
    cid = x["id"]; c = clusters_by_id.get(cid); a = audits_by_id.get(cid)
    if not c: continue
    occs = list(c.get("occurrences", []))
    vc = (a or {}).get("verified_count", x["verified_count"])
    for dr in ((a or {}).get("dropped") or []):
        dref = dr.get("ref", "")
        best, bs = None, 0
        for o in occs:
            s = score(o.get("ref",""), dref)
            if s > bs: bs, best = s, o
        if best is not None and bs > 0: occs.remove(best)
    if len(occs) != vc: occs = list(c.get("occurrences", []))[:vc]
    c2 = dict(c); c2["occurrences"] = occs; c2["count"] = len(occs)
    c2["promoted"] = False; c2["promoted_at"] = None
    demoted_full.append(c2)

# ---- ADR rename (bake into statements/refs) ----
def rn(s):
    if not s: return s
    for a, b in [("D-memos","ADRs"),("D-memo","ADR"),("design memos","ADRs"),
                 ("Design memos","ADRs"),("design memo","ADR"),("Design memo","ADR")]:
        s = s.replace(a, b)
    return s

def q(s): return '"' + rn(" ".join(str(s).split())).replace("\\","\\\\").replace('"','\\"') + '"'

def entry(c):
    L = [f'- id: {c.get("id","")}', f'  kind: {c.get("kind","")}', '  tier: 1',
         f'  domain: {c.get("domain","")}', f'  statement: {q(c.get("statement",""))}',
         f'  steelman: {q(c["steelman"]) if c.get("steelman") else "null"}',
         f'  blocked_on: {q(c["blocked_on"]) if c.get("blocked_on") else "null"}',
         f'  status: {c.get("status","settled")}', '  promoted_at: null',
         f'  count: {c.get("count", len(c.get("occurrences",[])))}']
    occs = c.get("occurrences", [])
    if occs:
        L.append('  occurrences:')
        for o in occs:
            L.append(f'    - {{date: "{o.get("date","")}", kind: {o.get("kind","")}, ref: {q(o.get("ref",""))}}}')
    else:
        L.append('  occurrences: []')
    return "\n".join(L)

allt1 = tier1 + demoted_full
hdr = f"""# Muxsmith decision ledger (Tier 1) - reconstructed bootstrap
#
# Every considered approach and its outcome; always recorded, on-demand load.
# Entry schema (canonical copy in software-dev-process doctrine section 7):
#   id, kind (pattern|restraint|non-decision), tier (1|2), domain,
#   statement, steelman (restraint only), blocked_on (non-decision only),
#   status (settled|contested|blocked), promoted_at, count (== len(occurrences)),
#   occurrences: [{{date, kind (decided|reinforced|violated-corrected|deferred), ref}}]
# count is always len(occurrences); no occurrence, no increment. An occurrence is a
# distinct decision EVENT, not a distinct artifact documenting one event. Promotion
# to the Tier-2 nature files happens by the source x nature matrix (doctrine section 7).
#
# Bootstrapped 2026-07-13 by a retroactive reconstruction sweep over the whole
# project trail (git, 8 ADRs, the 10-session journal, plan-1..5.5 SDD verdict
# archives, ROADMAP/IDEAS/spec), walked chronologically. 549 occurrence records
# -> 358 clusters: {len(tier1)} below-threshold + {len(demoted_full)} audit-demoted
# (promotion candidates whose count fell below 3 once the occurrence audit dropped
# duplicate-event citations) = {len(allt1)} Tier-1 entries. Tier-1 counts are
# reconstruction-stage; only the 48 promoted (-> Tier-2 nature files) passed the
# adversarial per-occurrence audit.
"""
open("/home/senol/Git/Muxsmith/docs/decision-ledger.yaml","w").write(
    hdr + "\nentries:\n" + "\n".join(entry(c) for c in allt1) + "\n")

print("demoted reconstructed:", len(demoted_full), "of", len(demo))
missing = [x["id"] for x in demo if x["id"] not in clusters_by_id]
print("missing from journal (if any):", missing)
print("total Tier-1 entries:", len(allt1))
print("ledger lines:", len(open('/home/senol/Git/Muxsmith/docs/decision-ledger.yaml').readlines()))
# sanity: any remaining 'memo' term
import subprocess
print("residual 'design memo'/'D-memo' in ledger:",
      subprocess.run(["grep","-ic","d-memo\\|design memo","/home/senol/Git/Muxsmith/docs/decision-ledger.yaml"],
                     capture_output=True,text=True).stdout.strip())
