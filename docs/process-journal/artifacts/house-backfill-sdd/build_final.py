import json, re
OUT="/tmp/claude-1000/-home-senol-agents-peter/f3a0b645-51c4-44b5-a80c-00a967d4d4d7/tasks/wh18cm78h.output"
ART="/home/senol/Git/Muxsmith/.superpowers/sdd/house-backfill"
DOCS="/home/senol/Git/Muxsmith/docs"
J="/home/senol/.claude/projects/-home-senol-agents-peter/99378955-7efb-44a9-882c-67c303aeaed0/subagents/workflows/wf_7ff09182-796/journal.jsonl"
d=json.load(open(OUT))["result"]; cls=json.load(open(ART+"/classify-output.json"))
prom_by_id={c["id"]:c for c in d["promotionsConfirmed"]}

# journal -> demoted full
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
def audit_occs(c):
    occs=list(c.get("occurrences",[])); a=c.get("audit",{}) or {}
    vc=a.get("verified_count",len(occs))
    for dr in (a.get("dropped") or []):
        best,bs=None,0
        for o in occs:
            s=score(o.get("ref",""),dr.get("ref",""))
            if s>bs: bs,best=s,o
        if best is not None and bs>0: occs.remove(best)
    if len(occs)!=vc: occs=list(c.get("occurrences",[]))[:vc]
    return occs,len(occs)
demoted_full=[]
for x in d["promotionsDemoted"]:
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
t1_all=d["tier1"]+demoted_full
t1_by_id={c["id"]:c for c in t1_all}

def q(s): return '"'+" ".join(str(s).split()).replace("\\","\\\\").replace('"','\\"')+'"'
def entry(c,source,nature,occs,count,promoted_at,tier=2,status=None):
    L=[f'- id: {c["id"]}',f'  kind: {c["kind"]}',f'  tier: {tier}']
    if tier==2: L+= [f'  source: {source}',f'  nature: {nature}']
    L+=[f'  domain: {c.get("domain","")}',f'  statement: {q(c.get("statement",""))}',
        f'  steelman: {q(c["steelman"]) if c.get("steelman") else "null"}',
        f'  blocked_on: {q(c["blocked_on"]) if c.get("blocked_on") else "null"}',
        f'  status: {status or c.get("status","settled")}',
        f'  promoted_at: {promoted_at if promoted_at is not None else "null"}',f'  count: {count}','  occurrences:']
    for o in occs:
        L.append(f'    - {{date: "{o.get("date","")}", kind: {o.get("kind","")}, ref: {q(o.get("ref",""))}}}')
    return "\n".join(L)

buckets={"product-boundaries.yaml":[],"conventions.yaml":[],"process-conventions.yaml":[]}
# 48 promotions (merge i18n-02 -> core-37)
staged={}
for p in cls["promotions"]:
    c=prom_by_id[p["id"]]; occs,cnt=audit_occs(c)
    staged[p["id"]]=dict(c=c,source=p["source"],nature=p["nature"],file=p["file"],occs=occs,count=cnt)
if "i18n-02-prose-free-core" in staged and "core-37-prose-free-core" in staged:
    src=staged.pop("i18n-02-prose-free-core"); tgt=staged["core-37-prose-free-core"]
    seen={(o.get("date"),o.get("ref")) for o in tgt["occs"]}
    for o in src["occs"]:
        if (o.get("date"),o.get("ref")) not in seen: tgt["occs"].append(o)
    tgt["count"]=len(tgt["occs"])
for s in staged.values():
    buckets[s["file"]].append(entry(s["c"],s["source"],s["nature"],s["occs"],s["count"],3))
# 54 handpromotions
for h in cls["handpromotions"]:
    c=t1_by_id[h["id"]]; occs=c.get("occurrences",[]); cnt=c.get("count",len(occs))
    buckets[h["file"]].append(entry(c,h["source"],h["nature"],occs,cnt,cnt))
# 4 RATIFIED escalations -> product-boundaries (user-decree)
RATIFIED=["core-05-global-toggles-rejected","cross-02-wails-rejected","cross-03-avalonia-rejected","core-83-zero-rule-keep-passthrough"]
for rid in RATIFIED:
    c=dict(t1_by_id[rid]); occs=list(c.get("occurrences",[]))
    if rid=="core-83-zero-rule-keep-passthrough":
        c["kind"]="pattern"
        c["statement"]=("A zero-rule profile with tracks.unmatched=keep is a LEGAL pure-passthrough remux "
            "(all primary tracks copied; use it to change only title / attachments / chapters, or to "
            "normalize the container, without any track rule). With tracks.unmatched=drop, zero rules "
            "stays a NoTrackRules error. MUST be documented and hinted - the passthrough path via "
            "unmatched=keep is non-obvious. (Şenol ruling, escalation resolution, session 10.)")
        c["blocked_on"]=None; c["status"]="settled"
        occs=occs+[{"date":"2026-07-13","kind":"decided","ref":"Şenol ruling (escalation resolution, session 10): zero-rule keep = legal passthrough"}]
    else:
        c["status"]="settled"
        occs=occs+[{"date":"2026-07-13","kind":"reinforced","ref":"Şenol ratification (escalation resolution, session 10)"}]
    buckets["product-boundaries.yaml"].append(entry(c,"user-decree","product-scope",occs,len(occs),len(occs)))

HDRS={
"product-boundaries.yaml":("product boundaries","product-scope","What the product IS and ISN'T. The owner/controller reads this when scoping a feature; an agent must not cross a boundary here or add one (agent-emergent product-scope escalates to the owner - doctrine section 7)."),
"conventions.yaml":("technical conventions","technical-code","How we write code in this repo. The reviewer checks every diff against this (the house dimension); a lone deviation is the outlier, not a new style."),
"process-conventions.yaml":("process & operational conventions","process + operational","How we work: method, gates, CI, release. Project-specific; complements the general software-dev-process doctrine, it does not restate it."),
}
def header(fname):
    title,nat,blurb=HDRS[fname]
    return f"""# Muxsmith {title} (Tier 2, nature = {nat})
#
# {blurb}
#
# Entry schema (canonical copy in software-dev-process doctrine section 7):
#   id, kind (pattern|restraint|non-decision), tier (=2 here),
#   source (user-decree|controller-adr|agent-emergent),
#   nature (product-scope|technical-code|process|operational),
#   domain, statement, steelman (restraint), blocked_on (non-decision),
#   status (settled|contested|blocked), promoted_at, count (== len(occurrences)),
#   occurrences: [{{date, kind (decided|reinforced|violated-corrected|deferred), ref}}]
# Promotion by the source x nature matrix (doctrine section 7).
"""
for fname,entries in buckets.items():
    open(f"{DOCS}/{fname}","w").write(header(fname)+"\nentries:\n"+"\n".join(entries)+"\n")
    print(f"{fname}: {len(entries)} entries")

# ---- ledger: drop 54 handpromotions + 4 ratified; gui-26 escalated->blocked ----
def rn(s):
    if not s: return s
    for a,b in [("D-memos","ADRs"),("D-memo","ADR"),("design memos","ADRs"),("Design memos","ADRs"),("design memo","ADR"),("Design memo","ADR")]:
        s=s.replace(a,b)
    return s
def ql(s): return '"'+rn(" ".join(str(s).split())).replace("\\","\\\\").replace('"','\\"')+'"'
def lentry(c):
    status = "blocked" if c["id"]=="gui-26" else c.get("status","settled")
    L=[f'- id: {c["id"]}',f'  kind: {c["kind"]}','  tier: 1',f'  domain: {c.get("domain","")}',
       f'  statement: {ql(c.get("statement",""))}',
       f'  steelman: {ql(c["steelman"]) if c.get("steelman") else "null"}',
       f'  blocked_on: {ql(c["blocked_on"]) if c.get("blocked_on") else "null"}',
       f'  status: {status}','  promoted_at: null',
       f'  count: {c.get("count",len(c.get("occurrences",[])))}','  occurrences:']
    for o in c.get("occurrences",[]):
        L.append(f'    - {{date: "{o.get("date","")}", kind: {o.get("kind","")}, ref: {ql(o.get("ref",""))}}}')
    return "\n".join(L)
drop=set(h["id"] for h in cls["handpromotions"])|set(RATIFIED)
led=[c for c in t1_all if c["id"] not in drop]
lhdr=f"""# Muxsmith decision ledger (Tier 1) - reconstructed bootstrap
#
# Every considered approach and its outcome; always recorded, on-demand load.
# Entry schema (canonical copy in software-dev-process doctrine section 7):
#   id, kind, tier(1|2), source, nature, domain, statement, steelman, blocked_on,
#   status (settled|contested|blocked|escalated), promoted_at, count (==len(occurrences)),
#   occurrences: [{{date, kind, ref}}]. An occurrence is a distinct decision EVENT.
# Promotion (Tier 1 -> a Tier-2 nature file) by the source x nature matrix (doctrine §7).
#
# Bootstrapped 2026-07-13 from a retroactive reconstruction sweep (549 records ->
# 358 clusters). 48 recurrence + 54 authority + 4 escalation-ratified -> Tier-2
# nature files; 1 escalation (gui-26) resolved as blocked-on-Plan-6; {len(led)} stay Tier 1.
"""
open(f"{DOCS}/decision-ledger.yaml","w").write(lhdr+"\nentries:\n"+"\n".join(lentry(c) for c in led)+"\n")
print("ledger Tier-1 entries:",len(led))
