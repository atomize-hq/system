# Overlay: Research & Discovery Tasks

Use this overlay when you want explicit research/discovery tasks to exist before committing to architectural or
implementation decisions.

## Intent
Planning should be bound in reality: unknowns get validated via small experiments, reading existing code, or targeted
proofs of concept.

## Task type: research_discovery
When active, sprint planning should create `research_discovery` tasks for:
- unknown external dependencies or integrations
- performance assumptions or scaling targets
- unclear APIs/contracts
- unclear testing strategy at boundaries

Each research task must specify:
- a hypothesis or question
- method (read code / prototype / benchmark / doc review)
- expected output artifact (note, summary, decision input)
- stop condition (how you know it's "done")

## Output expectations
Research tasks should produce one of:
- a short research note (2–10 bullets)
- an update to a decision registry
- a concrete recommendation + evidence summary
