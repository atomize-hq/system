# HCM-1.4 implementation review 2 contract-repair proof wall

- review agent: `/root/hcm_1_4_impl_review_2`
- review result: `BLOCKED`
- finding class: Required
- scope: HCM-1.4 checkout-stable package-definition bytes only

## Reproduced contradiction

The fresh reviewer admitted the exact 27-path staged subject, then reproduced
both mandatory named tests on the native Windows MSVC host. With repository
`core.autocrlf=true`, Git checked fingerprint-bound package definitions out as
CRLF. `include_bytes!` therefore bound bytes different from the existing LF
fingerprints: the engine test failed with `Registry(FingerprintMismatch)` and
the compiler setup test returned `ShippedProfileUnavailable`.

The implementation packet simultaneously required native-Windows runtime
success, exact-byte fingerprint enforcement, no definition-byte edits, and no
repository-root file change. Canonical-LF temporary copies had hidden the normal
Windows checkout failure. Reader-side newline normalization would weaken the
exact-byte contract and is rejected.

## Bounded contract repair

The SPEC, plan, and todo now admit exactly one repository-root change:
`crates/engine/definitions/** text eol=lf` in `.gitattributes`. This makes Git
preserve the already-authoritative LF bytes at the source checkout boundary;
it neither changes any of the 29 definition index blobs nor teaches an engine
reader to accept alternate bytes.

Before that runtime rule is implemented, this documentation-only repair must
receive fresh isolated read-only review. After approval, the parent must add
only the frozen attribute, prove all 29 attributes and byte identities in a
fresh native-Windows `core.autocrlf=true` clone, rerun both named tests there,
and replay the full HCM-1.4 wall. The other valid review findings remain open
and receive separate in-scope remediation before final review.
