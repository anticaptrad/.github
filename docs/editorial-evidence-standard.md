# AntiCapTrad editorial evidence standard

This standard applies to every research note, AI prompt, script, storyboard, caption, thumbnail claim, description, and live-stream talking point produced by AntiCapTrad systems.

The channel's recurring question is legitimate but easy to overstate:

> When do profit, growth, scale, or market-share incentives conflict with conservative or traditional goods such as family stability, local rootedness, privacy, stewardship, public decency, equal justice, Sabbath rest, professional duty, and care for vulnerable people?

The system may investigate that question. It must not turn it into a predetermined answer or fabricate a single economic cause for a complicated social change.

## Required claim record

Every material factual claim must have a machine-readable record containing:

- the exact claim as it will appear;
- the proposed causal mechanism;
- the relevant institution, place, and time period;
- at least one primary source, or a reason no primary source exists;
- corroborating sources where the claim is disputed or unusually strong;
- contrary evidence or the strongest reasonable counterargument;
- an evidence grade;
- the date sources were checked;
- language that is permitted in the script;
- language that is prohibited because it outruns the evidence.

A source merely mentioning two facts is not evidence that one caused the other. Revenue, lobbying, ownership, chronology, and outcomes should be sourced independently when an episode links them.

## Evidence grades

### A — documented mechanism

A primary source, reproducible dataset, court record, contract, audit, contemporaneous business record, or high-quality research design directly supports both the mechanism and the reported outcome.

Permitted language includes: “the contract required,” “the audit found,” “the experiment measured,” and “the company reported.”

### B — supported inference

Multiple reliable facts support the interpretation, but intent or causation is not directly established. Plausible alternatives remain.

Permitted language includes: “this creates an incentive,” “the evidence is consistent with,” “one defensible interpretation is,” and “the pattern may help explain.”

### C — editorial hypothesis

The idea is worth investigating but currently rests on analogy, incomplete evidence, disputed causal attribution, or a normative judgment.

It must be labeled as a question or hypothesis. It may not be voiced as settled history.

### D — not ready for publication

The claim relies on an invented motive, an anonymous assertion that cannot be checked, a misleading statistic, a source that does not support the stated conclusion, or a causal leap with no adequate evidence.

It may remain in the research backlog, but it must not reach a generated script or public asset.

## Causal-language rules

The pipeline must distinguish:

1. **Incentive:** what conduct a payment, ownership, advertising, quota, metric, or market structure rewards.
2. **Behavior:** what an institution or person actually did.
3. **Outcome:** what followed and how it was measured.
4. **Attribution:** how confidently the behavior caused the outcome.
5. **Motive:** what decision-makers intended, which requires direct evidence rather than speculation.

Do not substitute an incentive for proof of behavior, correlation for proof of causation, or a social outcome for proof of a coordinated plan.

## Corrections to recurring draft claims

### Health care

A defensible episode may show how fee-for-service payment, consolidation, pharmaceutical exclusivity, insurance design, or private-equity ownership can reward revenue-generating care and create tension with access, continuity, or professional duty. It must identify the specific payment mechanism and outcome. “For-profit medicine never helps people” is false and prohibited.

### Contract prisons

Government audits can support claims about safety incidents, contract monitoring, staffing, or quality. They do not by themselves prove that a prison operator caused over-criminalization or lobbied for a particular sentencing rule. Lobbying and sentencing claims require their own records.

### Legal representation

The strongest documented frame is unequal access to representation and the procedural advantages that resources can purchase. “Rich criminals are above the law” is an overbroad conclusion unless an episode examines a specific case and its evidence.

### Women's education

The proposition that universities enrolled daughters principally to obtain “daddy's money” is not ready for publication. Expansion of women's education has many documented causes and benefits. An episode may investigate university revenue incentives only with institution-specific enrollment, finance, marketing, and decision records.

### Women's paid employment

The proposition that a coordinated capitalist class put women to work principally to double labor supply and suppress wages is not established. Women's labor-force participation reflects changes in education, fertility, technology, law, occupational structure, household expectations, war, individual agency, and employer demand. Episodes may examine employer preference for lower-paid labor in a specific industry and period, but must not convert that evidence into a universal explanation.

### Media, pornography, and attention markets

It is defensible to examine how advertising, subscriptions, recommendation systems, and engagement metrics monetize attention and can reward sensational or sexual content. Claims about individual or population-level harms require separate evidence and should distinguish exposure, association, and causation.

## Counterargument requirement

Every episode must include the strongest serious counterargument, not a caricature. Typical counterarguments include:

- profit can fund innovation, scale, access, and professional specialization;
- competition can punish low quality when consumers have information and meaningful choice;
- public and nonprofit institutions can fail under different incentives;
- women and other groups gained autonomy and opportunity through market participation;
- mass media can broaden access to art, education, and dissent;
- regulation can create capture, scarcity, or incumbent protection;
- the relevant problem may be monopoly, subsidy, liability rules, information asymmetry, or weak governance rather than markets in general.

The conclusion may remain critical, but it must answer the best version of the opposing case.

## Source hierarchy

Prefer, in order:

1. statutes, regulations, contracts, court filings, audited financial statements, public datasets, inspector-general reports, and original research papers;
2. systematic reviews and high-quality academic syntheses;
3. reputable investigative reporting with named evidence;
4. trade publications and clearly disclosed advocacy research;
5. commentary, social posts, and anonymous anecdotes only as leads—not proof.

AI-generated summaries and citations are never sources. The retrieval system must resolve every citation to the underlying document and verify that the cited passage supports the exact sentence.

## Script structure

A standard episode should follow this sequence:

1. **Traditional good:** identify the value at stake without assuming all traditionalists define it identically.
2. **Market mechanism:** identify the concrete revenue, ownership, labor, advertising, or financial incentive.
3. **Evidence:** present measured conduct and outcomes with dates and scope.
4. **Limits:** state what the evidence does not prove.
5. **Counterargument:** present benefits or alternative explanations fairly.
6. **Institutional alternatives:** discuss possible reforms without pretending every reform is costless.
7. **Conclusion:** distinguish fact from the channel's normative judgment.

## Automated publishing gates

No generated asset may move directly from model output to public YouTube visibility.

The minimum lifecycle is:

`research draft → citation verification → editorial review → rendered candidate → private YouTube upload → channel-identity verification → playback/QC → explicit publish approval → visibility change`

Required safeguards:

- default YouTube visibility is `private`;
- channel ID and handle must match the configured AntiCapTrad channel before upload;
- script, render, source manifest, approval, and upload records share one correlation ID;
- public or unlisted visibility requires an authenticated, audited action separate from upload;
- prompts and telemetry must not contain API keys, OAuth material, local file paths, private account data, or unpublished personal information;
- browser automation must not bypass Google authentication, bot protections, authorization prompts, or YouTube policy controls;
- official platform APIs are preferred; browser automation is reserved for authorized functions with no supported API;
- a failed citation check, unresolved rights question, mismatched channel, or missing approval fails closed.

## Corrections policy

Descriptions should preserve a source manifest and correction history. When a material claim is wrong or materially incomplete:

1. stop scheduled republication and derivative generation;
2. correct the source record and script;
3. add a visible correction to the existing video's description when feasible;
4. publish a correction or revised edition when the error changes the argument;
5. retain the audit record rather than silently rewriting history.
