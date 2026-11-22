# Mathematical review of the part files (P01 §9–§12, P02 §13 and §15, P03 §16 and §18, P05 §35)

Reviewer: independent, read-only. Scope as assigned. Every worked number in scope was recomputed, by hand and by
script (exact rationals, IEEE binary32 emulation through NumPy, brute-force enumeration, and a simulation of the
free-body variational step). Nothing in the part files was changed.

Counts: **2 critical, 7 major, 17 minor.** Outside those 26 items the mathematics in scope is correct; the list of
sections checked and found correct is at the end.

Severity: *critical* is a wrong formula or rule that would be built on; *major* is a convention inconsistency, an
overclaim, or a theorem-card route that cannot work as stated; *minor* is everything else that is wrong.

## 1. Findings, ordered by severity

| ID | Sev. | File and place | Quoted text (short) | What is wrong | Corrected text |
|---|---|---|---|---|---|
| F-01 | critical | P03.md:302, §16.7 step 2 | "the right-trivialized derivative of τ at −h·ξₖ" | Wrong trivialization for the wrong sign of the argument. C-36 (P01.md:1564–1576) uses the **left**-trivialized differential at −h·ξₖ, which equals the right-trivialized one at **+**h·ξₖ. The right-trivialized differential at −h·ξₖ equals the left-trivialized one at +h·ξₖ, which is exactly the planted negative control of C-36 (P01.md:1607) and the counterexample "right-trivialized differential in the variational solve" of the P01 ledger. Simulated on inertia diag(1, 2, 3): C-36's form is second order (global error ratios 4.00, 4.01 per halving), energy error 1.1·10⁻⁴ → 6.8·10⁻⁶ from h = 0.02 to 0.005, reversal to 10⁻¹⁴; the §16.7 form is first order (ratios 1.99, 1.97), energy error 0.44 → 0.094, reversal error 0.38. Spatial momentum is exact in both, so the momentum row of GATE-KIN-01 cannot catch it. | "the left-trivialized derivative of τ at −h·ξₖ (equally, the right-trivialized derivative at +h·ξₖ; the matrix of C-36)". Replacement R-01. |
| F-02 | critical | P03.md:255–257 (§16.6) and P03.md:584 (PO-KIN-03); against P05.md:983–996 and :1209 | "any subtree of trailing zeros contributes an exact zero … The canonical zero is +0"; "trailing +0 entries equals the sum over the real prefix, for every capacity … Proved by induction" | False for a prefix whose sum is −0.0: (−0.0) + (+0.0) = +0.0, so the bits change with capacity (one active element −0.0 at capacity 1 against capacity 2; any power-of-two count of −0.0 entries). §35.5 states exactly this as its counterexample, prescribes −0.0 as the padding of sums and makes "padding with +0.0" a negative control of GATE-SPARK-15. The two chapters prescribe opposite padding values and PO-KIN-03 is claimed `Proved` although it is false as stated. | Pad sums with −0.0 in §16.6 and in PO-KIN-03. Replacements R-02a, R-02b, R-02c. |
| F-03 | major | P01.md:475–477 and :494 (C-10); P02.md:140 (§13.4 table) | "The ambient group is the leg-permutation part of Γ_obs, intersected with the isotropy subgroup of the commanded velocity. Trot, pace, bound, walk, pronk and the jump are distinct (H, K) pairs"; "The quadruped atlas recovers the primary gaits reported by Golubitsky et al." | With the stated ambient group the claim and the test cannot hold. The body group of a quadruped is at most C₂ × C₂ and a forward command leaves the left-right mirror C₂. Under C₂, trot, pace and walk all have (H, K) = (C₂, 1) and bound and pronk both have (C₂, C₂). Under C₂ × C₂ trot, pace, bound and pronk separate, but walk has H = C₂ only and neither walk nor the jump receives its quarter-period lag. The six primary gaits of Golubitsky et al. are separated by ℤ₄ × ℤ₂, the symmetry of their eight-cell clock network, which contains permutations no isometry of the body realizes and is not a subgroup of Γ_obs. | State both groups: the body group gates reduction; a declared clock-network group gives the six patterns. Replacements R-03a, R-03b, R-03c. |
| F-04 | major | P01.md:157–159 and :166–168 (C-01); P02.md:402 | "intersections and products in the subgroup lattice, which gives the right answer for over-constrained linkages"; pilot "the Sarrus and Bennett linkages … Falsifier: one wrong mobility" | Overclaim. The subgroup lattice explains exceptional over-constrained linkages (Sarrus: two planar-gliding limbs intersect in a line translation). It does not explain paradoxical ones: the Bennett linkage has four skew revolute axes, no product of which is a subgroup, and its mobility follows from metric conditions on twists and lengths. A lattice method returns the count or no verdict on Bennett, so the pilot fails by construction on its own falsifier. The known answers themselves (Sarrus 1 against 0, Bennett 1 against −2) are right. | Restrict the claim to exceptional linkages and require `undecided` on Bennett. Replacements R-04a, R-04b, R-04c. |
| F-05 | major | P01.md:1495–1497 (C-34, lane 2) | "which proves a unique root within twice the Newton step; distinctness follows from disjoint balls and reality from disjointness with the conjugate ball" | Two errors in the certificate criterion. (1) α(f, z) < (13 − 3√17)/4 proves that Newton's iteration from z converges quadratically to an associated root within 2β; it proves no uniqueness in that ball. (2) Reality is reversed: a ball disjoint from its conjugate proves the root is **not** real. Reality needs the robust test (α(f, z) < 0.03 and ‖z − Re z‖ < 1/(20·γ(f, z))), which shows that z and its conjugate share one associated root. The threshold 0.157671 is right. | Replacements R-05a, R-05b. |
| F-06 | major | P03.md:305–306, §16.7 step 3 | "μ⁺ is carried to the new body frame by the coadjoint action of τ(h·ξₖ)" | Convention inconsistency with §11.2 and §13.3, which define the group coadjoint of c as Ad_c⁻ᵀ. The transport of C-36 step 3 is μ° = Ad_cᵀ·μ⁺, the wrench pull-back, which is the group coadjoint of c⁻¹. Read with §13.3's definition, the sentence gives Ad_c⁻ᵀ and hence μ̇ = −ad_ξᵀμ, the reversed sign that ADR-MARROW-05 exists to prevent. The parenthesis about μ̇ = ad*_ξ μ points to the right map but the named group element is the wrong one. | "by the wrench pull-back μ° = (Ad_c)ᵀ·μ⁺ of §11.3 (the group coadjoint of c⁻¹, not of c), where c = τ(h·ξₖ)". Replacement R-06. |
| F-07 | major | P01.md:304 (C-06); against P02.md:1261 (§15.6) | "where the gradient of the convex potential ℓ_c is the contact impulse" | Sign. Stationarity of ½·(v − v*)ᵀA(v − v*) + Σ ℓ_c(J_c v) is A·(v − v*) = −Σ J_cᵀ∇ℓ_c, so the impulse that enters the momentum balance is −∇ℓ_c. §15.6 writes "Its impulse −∂ℓ/∂v". The two sections disagree by a sign. | "where minus the gradient of the convex potential ℓ_c is the contact impulse". Replacement R-07. |
| F-08 | major | P01.md:2083 (T-10) | "Polynomial identity per generator, or exact-rational randomized identity testing with a Schwartz–Zippel bound" | The route cannot work as stated for the kernels the card names. By C-14 a step kernel is a straight-line program over +, −, ×, ÷, √ and selects. A select makes the map piecewise, and a square root makes it non-polynomial; the Schwartz–Zippel bound applies to a polynomial of known degree sampled on a product set, not to either. Exact-rational evaluation of √ at random rational points is also not exact. The route works for branch-free rational kernels, with each √ replaced by a witness and a side relation, one identity per select branch. The card also omits that equivariance under Γ_mech needs gravity and terrain transformed (see F-16). | Replacement R-08. |
| F-09 | major | P02.md:185–186 (§13.4, route (ii)) | "in the real subfield of the n-th cyclotomic field for n-fold rotations, n ≤ 12" | Wrong field. A rotation by 2π/n has entries cos(2π/n) and sin(2π/n). The sine lies in the n-th cyclotomic field only when 4 divides n (n = 4, 8, 12 in range). For n = 3 and 6 the real subfield is ℚ and sin = √3/2 is outside it; for n = 5 it is ℚ(√5) and sin(72°) = √(10 + 2√5)/4 is outside it. The check PᵀP = I cannot even be posed in the stated field. The correct field is the real subfield of the cyclotomic field of order lcm(n, 4). (The character tables of Cₙ do live in the n-th cyclotomic field; only the isometries need the larger one. KILL-MARROW-BLOCKS's bit budget should be computed for it.) | Replacement R-09. |
| F-10 | minor | P03.md:1559 (§18.12) | "κ(Ĥ) is within a factor of two of the unit-diagonal value" | The scaled diagonal entries lie in [½, 2], a ratio of 4, and κ(T·B·T) ≤ κ(T)²·κ(B) with κ(T)² ≤ 4. H = diag(2, 1.99) under the rule of §35.10 gives Ĥ = diag(0.5, 1.99), κ = 3.98, where the unit-diagonal value is 1. | "κ(Ĥ) is within a factor of four of the unit-diagonal value". Replacement R-10. |
| F-11 | minor | P03.md:1547 (§18.12) against P05.md:1120 (§35.10) | "each sᵢ the power of two nearest to Hᵢᵢ^(−½) (`K-scale-diag`)" against "`K-scale` sets D_ii = 2^(−⌈e_i/2⌉)" | Two definitions and two names for one kernel. They agree only if "nearest" is taken in the logarithm; taken linearly they differ whenever Hᵢᵢ^(−½) lies between √2 and 1.5 times a power of two. The factor is bit-invariant either way, but every tolerance and the regularization are stated in scaled variables, so the two rules give different decisions. | One rule in both chapters: sᵢ = 2^(−⌈eᵢ/2⌉) with eᵢ the binary exponent of Hᵢᵢ. Replacement R-11. |
| F-12 | minor | P03.md:262–274 (§16.6 capacity table) | "Tracked skid-steer … 264 … 37 kB"; "four rows per contact … plus joint rows" | (a) The stated formula gives 36,464 bytes for the tracked row, which is 36 kB, not 37. (b) The stated row rule gives R = 120, 164 and 218 for the quadruped, hexapod and humanoid rows; the table has 128, 168 and 224, which are those values rounded up to a multiple of 8. The byte figures use the table's R and are otherwise right (26,668; 41,404; 71,752; 11,860; 37,164). | 36 kB; add "rounded up to a multiple of 8" to the rule. Replacements R-12a, R-12b. |
| F-13 | minor | P03.md:496–499 (§16.11) | "within a factor of four of that on 24 cores would produce 0.4 to 1.7 times the humanoid GPU target" | Does not follow from 28,000 to 46,000 steps/s per core. At a factor of four below, 24 cores give 168,000 to 276,000 steps/s, 0.34 to 0.55 of the 0.5 M target; at parity 1.3 to 2.2. The printed pair is reproduced only by a midpoint of 36,000 taken at both ends. "Most humanoid worlds" holds only near parity. | "0.34 to 0.55 … at a factor of four and 1.3 to 2.2 at parity". Replacement R-13. |
| F-14 | minor | P01.md:499–500 (C-10 ceiling); also P00.md:309 and P06.md:275, against P06.md:789 | "between 2% and more than 100% of a training run on the published figures above" | 4 s and 260 s against the plan's 410 s training run are 1.0% and 63%. P06.md:789 computes the same comparison as "1.6 to about 100 times" cheaper, which is 1% to 63%. The figure "2% to over 100%" needs a training run of 200 to 260 s, which is not the figure the plan cites. | "between about 1% and 63% of a 410 s training run". Replacement R-14. |
| F-15 | minor | P01.md:269–271 (C-05); P01.md:1592 (C-36 payoff) | "for a rigid body in a potential, its discrete Euler–Poincaré equations conserve the discrete momentum map exactly"; "exact momentum" | Beyond its domain. With a potential only the momentum of the symmetries the potential keeps is conserved (under gravity: vertical angular momentum and horizontal linear momentum, as C-36's own test and §16.7 say). | Replacements R-15a, R-15b. |
| F-16 | minor | P01.md:418–419 (C-09) | "The dynamics are equivariant under Γ_mech: f(ρ_X(g)·x, ρ_U(g)·u) = ρ_X(g)·f(x, u)" | Γ_mech preserves the body, not gravity or terrain (§13.4 puts gravity in 𝔾_obs). The identity holds when the world is transformed with the robot (the metamorphic use) or for the elements that fix gravity and terrain; a top-bottom symmetric body is the counterexample. | Replacement R-16. |
| F-17 | minor | P01.md:435–437 (C-09 item 3) against P02.md:137–141 (§13.4 table) | "Weight tying, invariant critics, orbit deduplication (C-22), block-diagonal linearizations and gait orbits (C-10) use only Γ_obs, intersected with the stabilizers of the current payload, boundary conditions and contact mode" | The two sections disagree on preconditions. §13.4 licenses weight tying under 𝔾_obs with world and command distributions invariant in law (no per-world stabilizer), and block-diagonal mass and stiffness matrices under 𝔾_mech ∩ Stab(configuration). C-09 puts both under Γ_obs ∩ world stabilizers, which would switch weight tying off in every asymmetric world. §13.4 is the correct statement. | C-09 item 3 should read: "*Exact reduction.* Invariant critics inside one world, orbit deduplication (C-22), closed-loop block-diagonal linearizations and gait orbits (C-10) use only Γ_obs, intersected with the stabilizers of the current payload, boundary conditions and contact mode. Weight tying needs Γ_obs and invariance in law of the world and command distributions; block-diagonal mass and stiffness matrices need only Γ_mech ∩ Stab(configuration) (§13.4)." |
| F-18 | minor | P01.md:382 (C-08 payoff) | "curvature magnitude over reachable shape space bounds displacement per cycle" | Not a bound. For a non-abelian group the curvature integral is the leading-order term of the holonomy (§11.6 says "≈" and "Large loops keep the ordered product"); it is exact only for abelian groups. | "curvature integrated over reachable shape space estimates, to leading order in loop size, the displacement per cycle". Replacement R-18. |
| F-19 | minor | P01.md:2119 (§12.3, obligations table) | "The articulated variational step is symplectic. … False as stated for a configuration-dependent mass matrix." | Wrong reason. A step derived from a discrete Lagrangian is symplectic whatever the mass matrix; what fails for a tree is that the kick–drift splitting of C-36 is no longer that variational step (C-36 and §16.7 say this correctly). | Replacement R-19. |
| F-20 | minor | P01.md:2099 (T-26) | "Σᵢ α/[i·(i + 1)] = α … Lean (telescoping sum)" | The infinite sum is an analytic statement (a limit), which ADR-LEMMA-01 defers and whose Mathlib closure is not known to avoid blocked modules. The property needed is algebraic: every partial sum equals α·n/(n + 1) < α. | Replacement R-20. |
| F-21 | minor | P01.md:2222–2231 (§12.4 reflectors) against T-21 at P01.md:2094 | "¬ ∃ x : Fin n → ℚ, …"; "∀ x : Fin n → ℚ, x ≠ 0 → 0 < x ⬝ᵥ A.mulVec x" | The soundness statements exclude or cover rational vectors only, while forces and velocities are real and T-21 promises "over an ordered field". The implications to ℝ are true but are extra arguments (for LDLᵀ: a rational kernel vector would exist), so the reflected instance proves less than the card. | State both conclusions for x over every linearly ordered field containing ℚ, with M, r and A cast into it; the one-line proofs are unchanged. |
| F-22 | minor | P03.md:350–352 (§16.8) | "the error rates hold at any stopping time by Ville's inequality, and independence of episodes is not needed" | True only for the conditional null. The likelihood-ratio process is a supermartingale when, given the earlier pairs, each new pair exceeds with probability at least p₁ (for E⁺) or at most p₀ (for E⁻). For a null about a marginal rate with dependent episodes it is not. | Replacement R-22. |
| F-23 | minor | P01.md:1344 (C-31 tier 1) | "m_cap is an upper bound on the moving mass" | As defined the kinetic term omits rotational and reflected inertia, which is a source of the kind the chapter's own omitted-term control rejects. A rotor of 10⁻⁴ kg·m² behind a ratio of 9 on a 0.05 m wheel at 0.4 m/s holds 0.26 J, against 0.64 J for the 8 kg body of the fixture. | "m_cap is an upper bound on the effective moving mass, including the reflected inertia of rotors, gears and wheels (N²·J_r/r² at a wheel of radius r)". Replacement R-23. |
| F-24 | minor | P01.md:991–997 (C-21) with P05.md:1056–1069 (§35.8) | "The scaled run must reproduce the original in every significand bit and every branch" | As stated the hypothesis fails on any tier that uses anchored positions: the cell side of 8 m is a dimensional constant of the representation, not an input. With lengths × 4, x = 3 m has offset 3.0 in cell 0 and its image 12 m has offset −4.0 in cell 2; the offsets are not power-of-two images and round at different granularity. The test needs the cell side (and the 12 m per-step bound) scaled with lengths. | Replacement R-24. |
| F-25 | minor | P05.md:868 (§35.1, D0 row) with P05.md:1083–1085 | "random draws" listed as D0 "on every backend" | Exact for the Philox integers and the uniform conversion. A normal variate built from `det_*` floating-point functions is a floating-point computation: D2 per backend, and equal across backends only where D1 holds, which the chapter labels a HYPOTHESIS. | Replacement R-25. |
| F-26 | minor | P01.md:253 (C-04 test) | "kinetic energy equals ½·q̇ᵀM q̇" | The same bullet defines q̇ = 𝒯(q)·v; for a floating base q̇ ≠ v and M pairs with v. | "kinetic energy equals ½·vᵀM v". Replacement R-26. |

## 2. Replacement strings

Each ORIGINAL below is a verbatim span that occurs exactly once in the named file (checked by script). Every
span lies on one physical line, so a plain text substitution applies. Where a sentence continues on the next line
of the file, the replacement is written to read correctly with that unchanged continuation.

### Critical

**R-01** (F-01), `parts/P03.md`

```text
ORIGINAL:    the momentum that the right-trivialized derivative of τ at −h·ξₖ
REPLACEMENT: the momentum that the left-trivialized derivative of τ at −h·ξₖ (equally, the right-trivialized derivative at +h·ξₖ; the matrix of C-36)
```

**R-02a** (F-02), `parts/P03.md`

```text
ORIGINAL:    in a prefix: any subtree of trailing zeros contributes an exact zero, so the tree over capacity P reduces to the
REPLACEMENT: in a prefix: any subtree of trailing −0.0 padding sums to −0.0, and x + (−0.0) = x for every x, so the tree over capacity P reduces to the
```

**R-02b** (F-02), `parts/P03.md` (the preceding line of the file ends "The canonical zero is")

```text
ORIGINAL:    +0. Counterexample that motivates the rule, in `f32`: with a = 10⁸, b = −10⁸, c = 1, (a + b) + c = 1 and
REPLACEMENT: −0.0, as §35.5 requires for sums: +0 is not neutral, because (−0.0) + (+0.0) = +0.0 changes the bits of a sum that is −0.0. Counterexample that motivates the rule, in `f32`: with a = 10⁸, b = −10⁸, c = 1, (a + b) + c = 1 and
```

**R-02c** (F-02), `parts/P03.md`

```text
ORIGINAL:    A power-of-two-aligned pairwise sum with trailing +0 entries equals the sum over the real prefix, for every capacity.
REPLACEMENT: A power-of-two-aligned pairwise sum with trailing −0.0 entries equals the sum over the real prefix, for every capacity (with +0 entries it fails when the prefix sums to −0.0).
```

### Major

**R-03a** (F-03), `parts/P01.md` (the next line of the file begins "pairs with prescribed phase lags.")

```text
ORIGINAL:    isotropy subgroup of the commanded velocity. Trot, pace, bound, walk, pronk and the jump are distinct (H, K)
REPLACEMENT: isotropy subgroup of the commanded velocity. That group is small: a quadruped commanded forward keeps only the left-right mirror C₂, under which trot, pace and walk share the pair (C₂, 1) and bound and pronk share (C₂, C₂). The six primary gaits are separated only by the symmetry group of a declared leg-clock network, ℤ₄ × ℤ₂ for the eight-cell quadruped network of Golubitsky et al.; that group contains permutations that no isometry of the body realizes, so it is a declared symmetry of the controller's coupling and not a subgroup of Γ_obs. Under ℤ₄ × ℤ₂ trot, pace, bound, walk, pronk and the jump are distinct (H, K)
```

**R-03b** (F-03), `parts/P01.md`

```text
ORIGINAL:    - **Test.** The quadruped atlas recovers the primary gaits reported by Golubitsky et al. (1999); the max-plus
REPLACEMENT: - **Test.** With the ℤ₄ × ℤ₂ clock-network group declared, the quadruped atlas recovers the primary gaits reported by Golubitsky et al. (1999); with the body group C₂ alone it returns the pairs (C₂, C₂), (C₂, 1) and (1, 1) and reports trot, pace and walk as one class; the max-plus
```

**R-03c** (F-03), `parts/P02.md`

```text
ORIGINAL:    | Gait enumeration by (H, K) pairs (C-10) | 𝔾_obs | A gait is a closed-loop object |
REPLACEMENT: | Gait enumeration by (H, K) pairs (C-10) | 𝔾_obs ∩ Stab(command) for the body; the declared clock-network group (ℤ₄ × ℤ₂ for a quadruped) for the phase patterns | A gait is a closed-loop object; the body group alone does not separate the six primary quadruped gaits |
```

**R-04a** (F-04), `parts/P01.md` (the next line of the file is "linkages where the Grübler–Kutzbach count is wrong.")

```text
ORIGINAL:    use intersections and products in the subgroup lattice, which gives the right answer for over-constrained
REPLACEMENT: use intersections and products in the subgroup lattice, which gives the right answer for the exceptional over-constrained linkages whose mobility follows from subgroup structure (Sarrus) and returns no verdict for paradoxical ones (Bennett, Bricard), whose mobility rests on metric conditions and is decided by the rank test alone; both kinds are
```

**R-04b** (F-04), `parts/P01.md` (the next line of the file begins "mobility.")

```text
ORIGINAL:    linkage, against the rank of the constraint Jacobian at exact rational sample points. Falsifier: one wrong
REPLACEMENT: linkage, against the rank of the constraint Jacobian at exact rational sample points. The lattice must return 1 for Sarrus and `undecided` for Bennett, where the rank test returns 1. Falsifier: one wrong
```

**R-04c** (F-04), `parts/P02.md`

```text
ORIGINAL:    | Subgroup-lattice mobility [F] (C-01) | Correct mobility for over-constrained linkages where the count is wrong; no speed benefit | Sarrus and Bennett by hand. Falsifier: a lattice answer that disagrees with the exact rank of A-01. |
REPLACEMENT: | Subgroup-lattice mobility [F] (C-01) | Correct mobility for exceptional over-constrained linkages (Sarrus) and an explicit `undecided` for paradoxical ones (Bennett); no speed benefit | Sarrus (the lattice returns 1) and Bennett (the lattice returns `undecided`, the rank returns 1) by hand. Falsifier: a lattice answer that disagrees with the exact rank of A-01. |
```

**R-05a** (F-05), `parts/P01.md`

```text
ORIGINAL:    which proves a unique root within twice the Newton step; distinctness follows from disjoint balls and reality
REPLACEMENT: which proves that Newton's iteration from z converges quadratically to a root within twice the Newton step; distinctness follows from disjoint balls, non-reality
```

**R-05b** (F-05), `parts/P01.md`

```text
ORIGINAL:    from disjointness with the conjugate ball. A result is `Complete` only when the certified count equals an
REPLACEMENT: from disjointness with the conjugate ball, and reality from the robust test α(f, z) < 0.03 with ‖z − Re z‖ < 1/(20·γ(f, z)), which shows that z and its conjugate converge to one root. A result is `Complete` only when the certified count equals an
```

**R-06** (F-06), `parts/P03.md` (the next line of the file begins "τ(h·ξₖ), with the sign convention of §11")

```text
ORIGINAL:    3. Drift and transport: gₖ₊₁ = gₖ·τ(h·ξₖ), and μ⁺ is carried to the new body frame by the coadjoint action of
REPLACEMENT: 3. Drift and transport: gₖ₊₁ = gₖ·τ(h·ξₖ), and μ⁺ is carried to the new body frame by the wrench pull-back μ° = (Ad_c)ᵀ·μ⁺ of §11.3 (the group coadjoint of c⁻¹, not of c), where c =
```

**R-07** (F-07), `parts/P01.md` (the next line of the file begins "impulse.")

```text
ORIGINAL:    v = argmin ½·(v − v*)ᵀA(v − v*) + Σ_c ℓ_c(J_c v), where the gradient of the convex potential ℓ_c is the contact
REPLACEMENT: v = argmin ½·(v − v*)ᵀA(v − v*) + Σ_c ℓ_c(J_c v), where minus the gradient of the convex potential ℓ_c is the contact
```

**R-08** (F-08), `parts/P01.md`

```text
ORIGINAL:    | T-10 | A model's derived step kernels are equivariant under each generator of its mechanical symmetry group. | Polynomial identity per generator, or exact-rational randomized identity testing with a Schwartz–Zippel bound | [F] |
REPLACEMENT: | T-10 | A model's derived step kernels are equivariant under each generator of its mechanical symmetry group, with gravity and terrain transformed by the same isometry. | Polynomial identity per generator for the branch-free rational kernels, with each square root replaced by a witness t and the side relation t² = radicand, and one identity per select branch under its guard; exact-rational randomized identity testing with a Schwartz–Zippel bound only on those polynomial pieces, with a recorded degree bound and sample set. A kernel with data-dependent selects (contact, limits) is `Tested` by the round-off-level residual test | [F] |
```

**R-09** (F-09), `parts/P02.md` (the preceding line of the file ends "in the real subfield of the")

```text
ORIGINAL:    n-th cyclotomic field for n-fold rotations, n ≤ 12; or (iii) by tolerance, for imported designs, in which case
REPLACEMENT: cyclotomic field of order lcm(n, 4) for n-fold rotations, n ≤ 12 (sin(2π/n) is in the n-th cyclotomic field only when 4 divides n; for n = 3 the real subfield is ℚ and sin(2π/3) = √3/2 is outside it); or (iii) by tolerance, for imported designs, in which case
```

### Minor (offered for convenience; F-17 and F-21 are multi-line and are given as corrected text in the table)

**R-10** (F-10), `parts/P03.md`

```text
ORIGINAL:    costs no determinism class; κ(Ĥ) is within a factor of two of the unit-diagonal value.
REPLACEMENT: costs no determinism class; κ(Ĥ) is within a factor of four of the unit-diagonal value, because the scaled diagonal entries span a ratio of up to four.
```

**R-11** (F-11), `parts/P03.md`

```text
ORIGINAL:    each sᵢ the power of two nearest to Hᵢᵢ^(−½) (`K-scale-diag`). Reasons:
REPLACEMENT: each sᵢ = 2^(−⌈eᵢ/2⌉), with eᵢ the binary exponent of Hᵢᵢ, which is the rule of §35.10 (`K-scale-diag` here and `K-scale` there are one kernel). Reasons:
```

**R-12a** (F-12), `parts/P03.md`

```text
ORIGINAL:    | Tracked skid-steer with two flippers | 12 | 8 | 80 | 64 | 264 | 37 kB | No, in normal driving. |
REPLACEMENT: | Tracked skid-steer with two flippers | 12 | 8 | 80 | 64 | 264 | 36 kB | No, in normal driving. |
```

**R-12b** (F-12), `parts/P03.md`

```text
ORIGINAL:    per actuated joint). Bytes are per world in `f32` with a dense Jacobian, from
REPLACEMENT: per actuated joint), rounded up to a multiple of 8. Bytes are per world in `f32` with a dense Jacobian, from
```

**R-13** (F-13), `parts/P03.md` (the next line of the file begins "humanoid worlds;")

```text
ORIGINAL:    produce 0.4 to 1.7 times the humanoid GPU target, so under the `lockstep` profile the CPU could shadow most
REPLACEMENT: produce 0.34 to 0.55 times the humanoid GPU target at a factor of four and 1.3 to 2.2 times at parity, so under the `lockstep` profile the CPU could shadow between a third and all of the
```

**R-14** (F-14), `parts/P01.md`

```text
ORIGINAL:    is between 2% and more than 100% of a training run on the published figures above, so the atlas replaces
REPLACEMENT: is between about 1% and 63% of a 410 s quadruped training run (4 s and 260 s; FACT, source register), so the atlas replaces
```

**R-15a** (F-15), `parts/P01.md`

```text
ORIGINAL:    in a potential, its discrete Euler–Poincaré equations conserve the discrete momentum map exactly (discrete
REPLACEMENT: in a potential, its discrete Euler–Poincaré equations conserve exactly the discrete momentum map of every symmetry the potential keeps, which is all of it for a free body (discrete
```

**R-15b** (F-15), `parts/P01.md`

```text
ORIGINAL:    Payoff.** For a body in a potential: no secular energy drift over long horizons, exact momentum, and a
REPLACEMENT: Payoff.** For a body in a potential: no secular energy drift over long horizons, exact momentum along the symmetries the potential keeps, and a
```

**R-16** (F-16), `parts/P01.md`

```text
ORIGINAL:    equivariant under Γ_mech: f(ρ_X(g)·x, ρ_U(g)·u) = ρ_X(g)·f(x, u). Each representation splits into isotypic
REPLACEMENT: equivariant under Γ_mech when gravity and terrain are transformed with the robot, and under the elements that fix them otherwise: f(ρ_X(g)·x, ρ_U(g)·u) = ρ_X(g)·f(x, u). Each representation splits into isotypic
```

**R-18** (F-18), `parts/P01.md` (the next line of the file begins "per cycle,")

```text
ORIGINAL:    - **Payoff.** A design oracle with no rollouts: curvature magnitude over reachable shape space bounds displacement
REPLACEMENT: - **Payoff.** A design oracle with no rollouts: curvature integrated over reachable shape space estimates, to leading order in loop size, the displacement
```

**R-19** (F-19), `parts/P01.md`

```text
ORIGINAL:    | The articulated variational step is symplectic. | False as stated for a configuration-dependent mass matrix. | Not claimed; energy behaviour is measured. |
REPLACEMENT: | The articulated variational step is symplectic. | True of a step derived from a discrete Lagrangian and false of the kick–drift splitting of C-36 once the mass matrix depends on configuration; either way an analytic property of a map, which no checker here decides. | Not claimed; energy behaviour is measured. |
```

**R-20** (F-20), `parts/P01.md`

```text
ORIGINAL:    is as stated; and Σᵢ α/[i·(i + 1)] = α. | Exhaustive enumeration in exact rationals; Lean (telescoping sum) |
REPLACEMENT: is as stated; and for every n, Σ_{i ≤ n} α/[i·(i + 1)] = α·n/(n + 1) < α. | Exhaustive enumeration in exact rationals; Lean (induction on n; the infinite sum is an analytic statement and is not needed) |
```

**R-22** (F-22), `parts/P03.md`

```text
ORIGINAL:    time by Ville's inequality, and independence of episodes is not needed. Budgeted mode (DESIGN): 32 pairs, after
REPLACEMENT: time by Ville's inequality. Independence of episodes is not needed if the null is read conditionally: given the earlier audited pairs, each new pair exceeds with probability at least p₁ (for E⁺) or at most p₀ (for E⁻). Budgeted mode (DESIGN): 32 pairs, after
```

**R-23** (F-23), `parts/P01.md`

```text
ORIGINAL:    m_cap is an upper bound on the moving mass and v a bound on its speed from the encoders; the integral
REPLACEMENT: m_cap is an upper bound on the effective moving mass, including the reflected inertia of rotors, gears and wheels (N²·J_r/r² at a wheel of radius r), and v a bound on its speed from the encoders; the integral
```

**R-24** (F-24), `parts/P01.md`

```text
ORIGINAL:    significand bit and every branch (HYPOTHESIS; the test is the gate). A kernel that hides a dimensional constant
REPLACEMENT: significand bit and every branch (HYPOTHESIS; the test is the gate), provided that representation constants with a dimension are scaled as inputs: the anchor cell side of §35.8 becomes 32 m when lengths are multiplied by 4, otherwise the offsets are not power-of-two images of each other. A kernel that hides a dimensional constant
```

**R-25** (F-25), `parts/P05.md`

```text
ORIGINAL:    exact-integer kernels and the decisions of guarded kernels on every backend (§21.8); random draws; rational clocks
REPLACEMENT: exact-integer kernels and the decisions of guarded kernels on every backend (§21.8); integer and uniform random draws (a normal variate built from `det_*` floating-point functions is D2 per backend and D1 across backends only where GATE-SPARK-01 passes); rational clocks
```

**R-26** (F-26), `parts/P01.md`

```text
ORIGINAL:    - **Test.** Parity with `fs-mbd` (GATE-KIN-03); ballistic invariants; kinetic energy equals ½·q̇ᵀM q̇;
REPLACEMENT: - **Test.** Parity with `fs-mbd` (GATE-KIN-03); ballistic invariants; kinetic energy equals ½·vᵀM v;
```

## 3. Answers to the six checks

1. **Conventions.** One convention is stated in §11.2 and it is used consistently in §10 (C-02, C-04, C-36), §13.3
   and §13.8: Ad_g on twists, Ad_g⁻ᵀ pushing wrenches forward, Ad_gᵀ pulling them back, ad*_ξ = ad_ξᵀ, and
   μ̇ = ad*_ξ μ + w, which is the correct sign for a left-invariant system in body velocity (checked in components
   and on the free-body fixtures). Angular-first ordering and `a_from_b` are consistent. The two departures are in
   the prose of §16.7 (F-01, F-06). The impulse sign of C-06 disagrees with §15.6 (F-07).
2. **Formulas.** All formulas listed in the brief were checked; the wrong ones are F-01 to F-09 and the minor
   items. Everything else is in §4.
3. **Worked numbers.** Wrong: the tracked-row bytes (F-12), "0.4 to 1.7" (F-13), "2% to more than 100%" (F-14),
   "factor of two" (F-10). Every other worked number in scope reproduces.
4. **Decidability of theorem cards.** T-10 cannot work as stated (F-08). T-26's infinite sum is not on the
   algebraic route (F-20). The reflector specifications prove less than T-21 promises (F-21). The remaining cards
   name routes that can work; T-06, T-13, T-33 and reflection carry honest HYPOTHESIS or [M] labels.
5. **Honest limits.** Symplecticity is not claimed for articulated trees or with dissipative contact, and
   cross-backend bit identity is labeled a HYPOTHESIS everywhere it appears. Over-reach found: F-03, F-04, F-15,
   F-16, F-18, F-24, F-25.
6. **Contradictions between sections.** F-01 and F-06 (§16.7 against C-36 and §11), F-02 (§16.6 against §35.5),
   F-07 (C-06 against §15.6), F-11 (§18.12 against §35.10), F-14 (C-10 against the gait table of §22), F-17 (C-09
   against §13.4).

## 4. Sections checked and found correct

**P01 §9.** D-10's bound 1/(1 − f + f/r + h); the arithmetic behind D-18 (452 + 23,438 of 65,536 is 36%).

**P01 §10.**
- C-01: the twelve Hervé classes and six lower pairs; Sarrus 0 → 1 and Bennett −2 → 1 as counts (the claim about
  the lattice is F-04).
- C-02: Ad as a functor, the contragredient, the three typed maps.
- C-03: pseudo-inertia and positivity; the mass-support bound and its fixture (0.012 m² against 0.010 m²; shell
  equality at 1/150 kg·m²); shell masses 64.8 kg, 3.45 kg, ratio 18.8.
- C-04: momentum, mechanical connection, Euler–Poincaré sign; the 2R fixture (J, torques −3 N·m, velocity
  (−0.03, 0.08) m/s, −0.3 W); the semidefinite articulated inertia diag(0, 1, 1, 1, 1, 1).
- C-05: Cayley on SO(3) and SE(3), translation (I − ω̂/2)⁻¹v, angle 2·atan(|ω|h/2), second order.
- C-06: cone, dual, strong convexity bound, the √2 pyramid exploit (41%).
- C-07: growth vectors, the ball-box exponent, the one-sided-input counterexample, the skid-steer argument, the
  chord formula and (0, 2/π), the `planar_drive` limit (0.8342, 0.5514).
- C-08: reconstruction equation, holonomy sign, Elroy's beanie.
- C-09: parity rule, Reynolds operator, real and complex types, absence of quaternionic type for the groups in
  scope, the sagittal-mirror check.
- C-10: H/K cyclic, twisted boundary condition, max-plus eigenvalue (the ambient group is F-03).
- C-11: up to four solutions of 3R positioning.
- C-12 to C-14: port algebra, T-15, the `f32` reassociation example.
- C-15: betting e-process, Ville, the table (factor 0.95/0.70, division by 6, 16 clean pairs, 22 with one
  exceedance), Σ α/[i(i + 1)] = α, and 0.0662 = 1 − 0.0125^(1/64) (the tail level is stated only in §28).
- C-16: saltation matrix and event-time shift; bouncing ball Ξ with determinant r²; rimless wheel map and
  multiplier cos²(2α) = ½ for eight spokes; 27 strata and 162 transitions.
- C-17 to C-19: the family definition and polarity table; the docking algebra (u₁ = 1/g − 2p, u₂ = p − 1/g,
  1 − U ≤ p ≤ (U + ½)/2, ε < (U + ½)/4, the witnesses, 15/16 at equality, 1/65 m under the gain 33/32); the
  non-convexity fixture (841/800 N); majority votes 27/32, 459/512, 3807/4096; label count; 720 orderings in 8
  classes; the bound (1 + TV)/2.
- C-20 to C-22: Gramian and gauge; the two-anchor fixture; similarity scales and groups; Burnside counts 86,185,
  500,500 and 92; wreath count (k² + k)/2; ballast 735,471 placements in 24,546 classes (enumerated).
- C-25 to C-27: control-variate variance, multilevel cost, doubly robust unbiasedness and variance (2.25);
  1/(1 − ρ²) thresholds; Kendall τ; split-conformal index (38 of 39; 19 as the minimum); zero-failure bound;
  defensive-mixture bound; splitting coefficient of variation 7%; ln 20/p*.
- C-28 to C-30: order ideals, the Shapley pair (½, ½), robustness semantics and the unit example.
- C-31: budget arithmetic (1.49 J of 2.00 J, 50.5 ms), sill 0.94176 J, the clipped projection (1, 0.5) and its
  counterexample (the definition of m_cap is F-23).
- C-32: Farkas and conic witnesses; the template identity Σqᵢfᵢ = B·P + e·W; the zonotope count; 13 bisections
  and side fractions 0.64 and 0.41; the stance family (b = 3/14 m, forces (160, 130, 110, 0), 165 N at b = 1/5 m,
  both allocations at b = ¼ m, torque (30, −60, 0)).
- C-33: ladder bound 2√2·a, both disc counterexamples, half-bin 3.75°.
- C-34: circumcentre system, Grashof, the α threshold, root counts 4, 16, 40 and 4,326 (the certificate wording
  is F-05).
- C-35: SIMP sensitivity; Miner damage 0.5 + 0.0625 and 11%; tolerance exponents and 0.0447, 0.0894 mm; the
  two-bar truss.
- C-36: the inverse Cayley differential on SE(3) (verified against finite differences to 2·10⁻¹⁰), its rotation
  block inverse, the four-step map (derived from the discrete Lagrangian and simulated), the transport identity
  Ad_{c⁻¹}ᵀ·(Dcay_x)⁻ᵀ = (Dcay_{−x})⁻ᵀ, the nonholonomic projection, the Chaplygin sleigh.
- C-37: the four-dimensional cone and its dual, r = 2R/3 and 3πR/16, coupling 1/√2, stiction and backlash rows.
- C-38: port-Hamiltonian form, entropy production, runaway condition and 35.8 A, the steady-state formula.
- C-39, C-40: the value-of-information closed form, the probe fixture, the dominance fixture.

**P01 §11.** §11.1 to §11.8 in full: adjoint and coadjoint matrices, the free-body checks, the push-forward check
(0, −1, 0) N·m, the parity table, pseudo-inertia identities, the curvature formula with its sign control and the
BCH bookkeeping, the clock example (L = 1,200; periods 3, 12, 40; repeat 120), the firing count, the 20 N impulse.

**P01 §12.** The class order and the claim product; cards T-01 to T-09, T-11 to T-25, T-27 to T-35 as routes
(T-10 and T-26 are F-08 and F-20); the list of obligations no checker decides, except the row of F-19; the bundle
example; the proof-kind registry.

**P02 §13.** The unit-complex update; the component table of §13.3 and the generator sign; the energy-blind
fixture (π̇ = (0, 0, −1), spatial rate (0, 0, 2) under the wrong sign); the reflection rules; the candidate count
2·m·(m − 1)·(m − 2); the Frobenius–Schur table and the dimension formula; the C₃ counterexample; 16,384 weights;
the guarded-predicate construction and the single-precision 0.3, 0.4, 0.5 case (equal in `f32`, exact value
+1.19·10⁻⁸); the laws of §13.8. The one error is F-09.

**P02 §15.** All of it: the convention factors √(3/2), √2, √3 (22% to 73%); the motor in port-Hamiltonian form;
the implicit current update; the stiction potential, creep ε·τ/τ_b and Hessian; rotor elimination and 0.09 N·m;
the backlash impact (0.577 rad/s, 0.0115 J); the thermal card (0.314 W/K, 164.7 K, 132 W, 247 s, 385 s, 22.8 A,
18.6 A, 31.9 A, 306 K, 83 s, 0.00136 s⁻¹, 61 s, 15.9 A, 16.5 A, 25.5 A, 12.8 K); the two-node condition; V_lim =
V_bus/√2 and 13.4%; the voltage ellipse and the card (33.9 V, 3,151, 3,394, −32.2 A, 23.7 A, 59%, 5,657); the
four-quadrant gear law and its counterexample; screw efficiencies; the differential's reflected inertia; the
battery equations; the bus ceiling (7,200 W, 44.8 V, 40.2 A).

**P03 §16.** Cholesky flop counts; 31 ns and 0.1 µm; the dyadic refinement law (variance ℓ/2, zero covariance)
and the ¾ redraw counterexample; the constrained impulse operator and the tied-sliders counterexample; leading-
block invariance of a padded Cholesky factor; branch-induced sparsity; the byte formula and the Jacobian share
(24% to 44%); the e-process table and the rejection example; the doubly robust construction and the factors 1.3,
2.0, 5.3, 10.3; the energy exploit (0.5 m/s, 0.125 J, 0.05 J); coverage arithmetic (1,000 and 4,000
world-seconds per second, 0.5% to 2.4% and 0.1% to 0.6%); 576 and 1,600 episodes, 8 minutes to 1.9 hours. Errors:
F-01, F-02, F-06, F-12, F-13, F-22. Also checked although outside the assigned lines: the padding bounds
1/r ≤ Ω_p ≤ r⁻ᵖ, the Ω table, the worked bucket card and the bound 1/(1 − f + f/r + h) of §17.5 and §17.6, all
correct.

**P03 §18.** Enclosure logic and the 6 mm case; the thin-barrier fixture (7 mm, 3 mm beyond); CL-H in normal,
friction and cone form, and the non-associated term; the flop ratio 14; the four-dimensional and anisotropic
cones; the backlash known answer (17.5 ms, 0.083 rad/s); Stefan's law, its integral and the three times (0.038 s,
2.3 s, 0.76 s); the brush tyre law; Bekker and Janosi–Hanamoto with 19.6 mm, 389 N and 293 N; the half-precision
calculation (4,801, 2.34, 9k/n and its three values); the known-answer battery (0.153 s, 0.288 s, 0.525, 0.690,
0.619). Errors: F-10, F-11.

**P05 §35.** The class definitions and what each licenses; 16.6/λ; the factor 10 for pairing; thirty discordant
worlds at 4,096 and the rule of three; the padding-neutrality rule (−0.0 for sums is the correct one); anchored
positions (7.6 µm, 0.24 µm, Sterbenz for 4 ≤ |x| ≤ 16, 2²⁵ cases, the 8 − 2⁻²⁵ counterexample); the uniform
conversion; scaling invariance of Cholesky, the exponent rule and its range [½, 2), the lower bound κ̂, the
refinement estimate, the diag(1, 10⁻⁸) example. The chapter is correct; its only item is the D0 wording of F-25,
and it is the right side of the contradictions F-02 and F-11.
