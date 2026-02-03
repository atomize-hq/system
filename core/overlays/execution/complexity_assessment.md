# Overlay: Complexity Assessment

Use this overlay when `enable_complexity` is true.

## Goal
Keep slices and functions small enough that:
- evidence remains easy to collect
- tests are straightforward
- quality gates remain deterministic

## Constraints
- Prefer adding one small module at a time.
- If a function has many branches/early returns, split it.
- If a slice requires touching many files/services, split it.

## Simple rubric (language-agnostic)
A change is **too complex** if it has any of:
- more than ~2 distinct responsibilities in one function/module
- more than ~7 decision points in one function (if you’re tracking that metric)
- more than ~3 integration boundaries touched (network, DB, filesystem, external services)
- more than ~400 LOC net-new in one slice (hard cap in many teams)

## Output expectation
When producing a slice plan, include:
- a short complexity note (“why this is tiny/small”)
- any proposed split points (“if this grows, split at …”)
