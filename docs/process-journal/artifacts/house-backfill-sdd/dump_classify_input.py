import json
OUT="/tmp/claude-1000/-home-senol-agents-peter/f3a0b645-51c4-44b5-a80c-00a967d4d4d7/tasks/wh18cm78h.output"
d=json.load(open(OUT))["result"]
conf=d["promotionsConfirmed"]; t1=d["tier1"]
def refs(c): return [o.get("ref","") for o in c.get("occurrences",[])]
def st(s): return " ".join(str(s).split())
prom=[{"id":c["id"],"kind":c["kind"],"domain":c["domain"],"statement":st(c["statement"]),
       "occ_refs":refs(c)} for c in conf]
res=[{"id":c["id"],"kind":c["kind"],"domain":c["domain"],"statement":st(c["statement"]),
      "count":c["count"],"occ_refs":refs(c)} for c in t1 if c.get("kind")=="restraint"]
non=[{"id":c["id"],"kind":c["kind"],"domain":c["domain"],"statement":st(c["statement"]),
      "count":c["count"],"blocked_on":st(c.get("blocked_on") or ""),"occ_refs":refs(c)}
     for c in t1 if c.get("kind")=="non-decision"]
json.dump({"promotions":prom,"restraints":res,"non_decisions":non},
          open("/home/senol/Git/Muxsmith/.superpowers/sdd/house-backfill/classify-input.json","w"),indent=0)
print("promotions:",len(prom),"restraints:",len(res),"non_decisions:",len(non))
