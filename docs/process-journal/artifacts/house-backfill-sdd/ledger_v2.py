import json, re
OUT="/tmp/claude-1000/-home-senol-agents-peter/f3a0b645-51c4-44b5-a80c-00a967d4d4d7/tasks/wh18cm78h.output"
ART="/home/senol/Git/Muxsmith/.superpowers/sdd/house-backfill"
J="/home/senol/.claude/projects/-home-senol-agents-peter/99378955-7efb-44a9-882c-67c303aeaed0/subagents/workflows/wf_7ff09182-796/journal.jsonl"
d=json.load(open(OUT))["result"]; cls=json.load(open(ART+"/classify-output.json"))
tier1=d["tier1"]; demo=d["promotionsDemoted"]

# journal for demoted full clusters + audits
clusters_by_id={}; audits_by_id={}
for line in open(J):
    line=line.strip()
    if not line: continue
    v=json.loads(line).get("result")
    if not isinstance(v,dict): continue
    if "clusters" in v:
        for c in v["clusters"]: clusters_by_id[c["id"]]=c
    elif "verdict" in v and "verified_count" in v: audits_by_id[v["id"]]=v
def toks(s): return set(t for t in re.findall(r"[A-Za-z0-9]{5,}",str(s or "")))
def score(a,b): return len(toks(a)&toks(b))
demoted_full=[]
for x in demo:
    c=clusters_by_id.get(x["id"]); a=audits_by_id.get(x["id"])
    if not c: continue
    occs=list(c.get("occurrences",[])); vc=(a or {}).get("verified_count",x["verified_count"])
    for dr in ((a or {}).get("dropped") or []):
        best,bs=None,0
        for o in occs:
            s=score(o.get("ref",""),dr.get("ref",""))
            if s>bs: bs,best=s,o
        if best is not None and bs>0: occs.remove(best)
    if len(occs)!=vc: occs=list(c.get("occurrences",[]))[:vc]
    c2=dict(c); c2["occurrences"]=occs; c2["count"]=len(occs); demoted_full.append(c2)

promoted_ids=set(h["id"] for h in cls["handpromotions"])   # migrated to Tier 2
esc_ids=set(e["id"] for e in cls["escalations"])
allt1=[c for c in (tier1+demoted_full) if c["id"] not in promoted_ids]  # keep escalations + stay + patterns

def rn(s):
    if not s: return s
    for a,b in [("D-memos","ADRs"),("D-memo","ADR"),("design memos","ADRs"),("Design memos","ADRs"),("design memo","ADR"),("Design memo","ADR")]:
        s=s.replace(a,b)
    return s
def q(s): return '"'+rn(" ".join(str(s).split())).replace("\\","\\\\").replace('"','\\"')+'"'
def entry(c):
    esc = c["id"] in esc_ids
    L=[f'- id: {c["id"]}',f'  kind: {c["kind"]}','  tier: 1',
       f'  domain: {c.get("domain","")}',f'  statement: {q(c.get("statement",""))}',
       f'  steelman: {q(c["steelman"]) if c.get("steelman") else "null"}',
       f'  blocked_on: {q(c["blocked_on"]) if c.get("blocked_on") else "null"}',
       f'  status: {"escalated" if esc else c.get("status","settled")}','  promoted_at: null',
       f'  count: {c.get("count",len(c.get("occurrences",[])))}','  occurrences:']
    for o in c.get("occurrences",[]):
        L.append(f'    - {{date: "{o.get("date","")}", kind: {o.get("kind","")}, ref: {q(o.get("ref",""))}}}')
    return "\n".join(L)

hdr=f"""# Muxsmith decision ledger (Tier 1) - reconstructed bootstrap
#
# Every considered approach and its outcome; always recorded, on-demand load.
# Entry schema (canonical copy in software-dev-process doctrine section 7):
#   id, kind (pattern|restraint|non-decision), tier (1|2), source, nature,
#   domain, statement, steelman (restraint), blocked_on (non-decision),
#   status (settled|contested|blocked|escalated), promoted_at, count (== len(occurrences)),
#   occurrences: [{{date, kind (decided|reinforced|violated-corrected|deferred), ref}}]
# count is always len(occurrences); an occurrence is a distinct decision EVENT,
# not a distinct document of one event. Promotion (Tier 1 -> a Tier-2 nature
# file) by the source x nature matrix (doctrine section 7). Tier-1 entries carry
# no source/nature yet (assigned at promotion).
#
# Bootstrapped 2026-07-13 by a retroactive reconstruction sweep over the whole
# project trail (git, 8 ADRs, the 10-session journal, plan-1..5.5 SDD verdict
# archives, ROADMAP/IDEAS/spec). 549 records -> 358 clusters; 48 recurrence-promoted
# + {len(promoted_ids)} authority-promoted -> Tier-2 nature files; {len(esc_ids)} escalated to the
# owner (status: escalated); the rest ({len(allt1)-len(esc_ids)}) stay Tier 1 below threshold.
# Tier-1 counts are reconstruction-stage; only promotions passed the occurrence audit.
"""
open("/home/senol/Git/Muxsmith/docs/decision-ledger.yaml","w").write(
    hdr+"\nentries:\n"+"\n".join(entry(c) for c in allt1)+"\n")
print("Tier-1 entries after migration:",len(allt1),"(removed",len(promoted_ids),"handpromoted )")
print("escalations kept in ledger (status=escalated):",len([c for c in allt1 if c['id'] in esc_ids]),"/",len(esc_ids))
