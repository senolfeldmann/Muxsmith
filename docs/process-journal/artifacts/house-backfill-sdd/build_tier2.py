import json, re
OUT="/tmp/claude-1000/-home-senol-agents-peter/f3a0b645-51c4-44b5-a80c-00a967d4d4d7/tasks/wh18cm78h.output"
ART="/home/senol/Git/Muxsmith/.superpowers/sdd/house-backfill"
DOCS="/home/senol/Git/Muxsmith/docs"
d=json.load(open(OUT))["result"]
cls=json.load(open(ART+"/classify-output.json"))
prom_by_id={c["id"]:c for c in d["promotionsConfirmed"]}
t1_by_id={c["id"]:c for c in d["tier1"]}

def toks(s): return set(t for t in re.findall(r"[A-Za-z0-9]{5,}", str(s or "")))
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

def q(s): return '"'+" ".join(str(s).split()).replace("\\","\\\\").replace('"','\\"')+'"'
def entry(c,source,nature,occs,count,promoted_at):
    L=[f'- id: {c["id"]}',f'  kind: {c["kind"]}','  tier: 2',
       f'  source: {source}',f'  nature: {nature}',f'  domain: {c.get("domain","")}',
       f'  statement: {q(c.get("statement",""))}',
       f'  steelman: {q(c["steelman"]) if c.get("steelman") else "null"}',
       f'  blocked_on: {q(c["blocked_on"]) if c.get("blocked_on") else "null"}',
       f'  status: {c.get("status","settled")}',f'  promoted_at: {promoted_at}',f'  count: {count}',
       '  occurrences:']
    for o in occs:
        L.append(f'    - {{date: "{o.get("date","")}", kind: {o.get("kind","")}, ref: {q(o.get("ref",""))}}}')
    return "\n".join(L)

buckets={"product-boundaries.yaml":[],"conventions.yaml":[],"process-conventions.yaml":[]}
srcmix={f:{} for f in buckets}

# --- 48 recurrence promotions ---
merge_into={"i18n-02-prose-free-core":"core-37-prose-free-core"}  # merge target
staged={}  # id -> (entry_obj) for merge handling
for p in cls["promotions"]:
    c=prom_by_id[p["id"]]; occs,cnt=audit_occs(c)
    staged[p["id"]]=dict(c=c,source=p["source"],nature=p["nature"],file=p["file"],occs=occs,count=cnt)
# merge i18n-02 -> core-37 (combine occurrences, dedup by date+ref)
if "i18n-02-prose-free-core" in staged and "core-37-prose-free-core" in staged:
    src=staged.pop("i18n-02-prose-free-core"); tgt=staged["core-37-prose-free-core"]
    seen={(o.get("date"),o.get("ref")) for o in tgt["occs"]}
    for o in src["occs"]:
        if (o.get("date"),o.get("ref")) not in seen: tgt["occs"].append(o); seen.add((o.get("date"),o.get("ref")))
    tgt["count"]=len(tgt["occs"])
for pid,s in staged.items():
    e=entry(s["c"],s["source"],s["nature"],s["occs"],s["count"],3)
    buckets[s["file"]].append(e); srcmix[s["file"]][s["source"]]=srcmix[s["file"]].get(s["source"],0)+1

# --- 54 authority handpromotions (Tier1 -> Tier2 at current count) ---
for h in cls["handpromotions"]:
    c=t1_by_id[h["id"]]; occs=c.get("occurrences",[]); cnt=c.get("count",len(occs))
    e=entry(c,h["source"],h["nature"],occs,cnt,cnt)
    buckets[h["file"]].append(e); srcmix[h["file"]][h["source"]]=srcmix[h["file"]].get(h["source"],0)+1

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
# Promotion (Tier 1 -> Tier 2) by the source x nature matrix: user-decree /
# controller-adr promote at count 1 (authority); agent-emergent promotes at
# count 3 (recurrence), except agent-emergent x product-scope which escalates
# to the owner. count is always len(occurrences); an occurrence is a distinct
# decision EVENT, not a distinct document of one event.
"""

for fname,entries in buckets.items():
    open(f"{DOCS}/{fname}","w").write(header(fname)+"\nentries:\n"+"\n".join(entries)+"\n")
    print(f"{fname}: {len(entries)} entries  source-mix={srcmix[fname]}")

# escalations for the owner
print("\nESCALATIONS (agent-emergent product-scope; need your ratification):")
for e in cls["escalations"]:
    c=t1_by_id.get(e["id"],{})
    print(f"- {e['id']}: {' '.join(str(c.get('statement','')).split())[:120]}  [{e['reason']}]")
print("\nstay Tier 1:", len(cls["stay_tier1"]))
