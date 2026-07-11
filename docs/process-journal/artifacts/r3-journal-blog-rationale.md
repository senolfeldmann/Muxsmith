# Recovered rationale: journal-vs-blog split (residue R3)

The strategic rationale behind the process-journal/blog architecture was
uttered once in the Plan-1 kickoff session and never persisted (audit
residue R3). Recovered 2026-07-11 by transcript mining; quotes verified
against the source line by line.

**Provenance.** Plan-1 session transcript, discussion 2026-07-08
15:49-15:55 CEST (the audit note's "late morning" was wrong). Trigger: the
user asked whether the blog posts and repo guide should be written
mid-project or at the end, and which session mechanics (same session,
handoff, transcript distillation) should produce them. The rationale is a
single assistant message, 15:51 CEST.

## Key passages (verbatim)

> The core insight driving both answers: your three deliverables decay at
> very different rates, and the prompting strategy should follow the
> material, not the other way around.

On the process story (fast decay, irreproducible):

> What makes such a post exceptional rather than generic AI-content is the
> lived specifics [...] partly only in this session's context (the token
> counts, the dispatch decisions, the reasoning). That last category is
> already degrading and will be gone after compaction.

> [The journal is] written by the session that lived them while the
> context is hot. [...] That journal is the diary the process blog gets
> written from; without it you'll be mining hundred-of-k-token transcripts
> for anecdotes. The per-plan close is also the moment I still *know*
> which review finding was interesting and which was noise.

On product docs and guide (opposite decay direction):

> Written now, they'd describe a torso - no dry-run, no executor, no GUI -
> and Plans 2-4 would invalidate half of every page. Writing them before
> 1.0 is churn, not progress.

On two posts:

> The process post and the product post have disjoint audiences (the
> AI-engineering crowd vs. the selfhosted/media-library crowd) and
> different half-lives - the process post is timely, the product post is
> evergreen. Separate, cross-linked. The repo guide is a third thing
> entirely: it lives *in* the repo as contributor documentation, and the
> product post links to it.

On who writes the final documents (fresh sessions at 1.0):

> Documents written by the context that did the work inherit its
> Betriebsblindheit. For the repo guide especially, the author's curse is
> fatal - its target audience is a fresh reader, so it should be *written*
> by a fresh reader forced to derive understanding from the actual repo.
> Where that reader stumbles is exactly where a contributor would.

> *Handoffs* are the wrong tool here - they're optimized for resuming
> work, deliberately dropping narrative texture, which is precisely what
> the blog needs. [...] *Transcript distillation* works but is the
> expensive, lossy fallback: the distiller can't distinguish load-bearing
> from incidental without guidance, and burns most of its budget reading.
> The journal is that guidance, pre-written by the author with intent.

## Condensation

1. **Differential decay rate** is the organizing principle: strategy
   follows the material.
2. **Process story**: decays fast, irreproducible after compaction ->
   rolling journal written immediately by the involved session at each
   plan boundary.
3. **Product docs / guide**: pre-1.0 they describe a torso and get
   invalidated by later plans -> written at 1.0.
4. **Fresh reader vs Betriebsblindheit**: final documents are written by
   fresh sessions at 1.0, fed journal + repo + git history + artifact
   archive; not by the involved session, not from handoffs, not from raw
   transcripts as primary feed.
5. **Disjoint audiences and half-lives**: two separate cross-linked posts
   (process: timely, AI-engineering crowd; product: evergreen,
   selfhosted/media-library crowd); the guide is a third artifact living
   in the repo.
6. **Only the raw-material capture is time-critical**; the finished
   documents keep until 1.0.

Division-of-labor nuance: the decay argument justifies the *journal*; the
Betriebsblindheit and audience arguments attach to the *authoring
strategy* for the final documents. The three strands appear in one message
but do different work.
