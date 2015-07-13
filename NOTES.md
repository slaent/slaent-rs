Suppose we need linkage information for any thread of at least `POSTS_PER_THREAD` posts (call this `P` for short), where `P >= 2`.

```
Posts(T) ⇔ T ⊆ N × N ∧
    ∀ (u, v) ∈ T, u < v ^ ( ∀ u' ∈ N . (u', v) ∈ T ↔ u = u' )

Parent(T, u, v) ⇔ Posts(T) ∧ (u, v) ∈ T

Ancestor(T, u, w) ⇔ ∃ v ∈ N s.t. Parent(T, u, v) and either
    (i) v = w, or
    (ii) Ancestor(T, v, w)

Links(T, L) ⇔ Posts(T), ∃ k ∈ N s.t. |L| = k * P, and ∃ u ∈ L s.t. ∀ v ∈ L,
    (i) u ≠ v ↔ Ancestor(T, u, v), and
    (ii) ∀ x ∈ N, u < x < v → (x ∈ L ↔ Ancestor(T, u, x))
```

```
Lemma 1: Ancestor(T, u, w) → u < w

Proof:

Assume the opposite, and apply WOP to get the minimal u - w s.t. Ancestor(T, u, w) and w ≤ u.
By defn Ancestor, ∃ v ∈ N s.t. Parent(T, u, v) ⇒ (u, v) ∈ T ⇒ u < v, and either:
    (BC) v = w ⇒ u < w ⇒ ⊥, or
    (IS) Ancestor(T, v, w).  But u < v, so u - w < v - w, which means u - w was not minimal if
         w ≤ v.  So v < w ⇒ u < w ⇒ ⊥
So our assumption was false, and u < w.
```

```
Lemma 2: Suppose Links(T, L).  Then ∀ v ∈ L, either
    (i) ∃ x ∈ L s.t. (x, v) ∈ T, or
    (ii) v = min(Links(T, L))

Proof:

By definition of Links(T, L), ∃ u ∈ L s.t. ∀ v ∈ L,
    (i) u ≠ v ↔ Ancestor(T, u, v), and
    (ii) ∀ x ∈ N, u < x < v → (x ∈ L ↔ Ancestor(T, u, x))

By Links(i) and Lemma (1), we know ∀ v ∈ L, u ≠ v → u < v ⇒ u = min(Links(T, L))

(i) Assume that ¬(∃ x ∈ L s.t. (x, v) ∈ T) ⇒ ∀ x ∈ L, ¬((x, v) ∈ T).

    Let |L| be the smallest member of the set of sizes of Links fulfilling this criterion.

    By definition of Links, |L| is at least N.

    (BC) [Insert proof that N isn't minimal here].

    (IS) Since |L| > N ≥ 2, take u, v ∈ L s.t. u = min(Links(T, L)), ∀ x ∈ L, ¬((x, v) ∈ T), and x ≠ v ∧ x ≠ u.
         Then either:
         (i) u < v < x ⇒> can shrink L by removing x (removing max element is always sound where |L| > N, see Lemma).
         (ii) u < x < v ⇒

    Apply WOP to get the minimal u s.t. ∀ x ∈ N, u ∈ L ∧ ¬((x, v) ∈ T).

(i) Suppose v ≠ u.

    Assume that ¬(∃ x ∈ L s.t. (x, v) ∈ T) ⇒ ∀ x ∈ L, ¬((x, v) ∈ T).

    Apply WOP to get the minimal v - x s.t. x ∈ L ∧ ¬((x, v) ∈ T).

    Since u ≠ x, by Links(i) Ancestor(T, u, x).

    Since x ∈ L and v ≠ u, u < x.  So u < x < v →

    Then Ancestor(T, u, v) ⇒ ∃ x ∈ N s.t. Parent(T, u, x) ⇒ (u, x) ∈ T ⇒ u < x, and either:
      (BC) x = v ⇒ Parent(T, u, v) ⇒ (u, v) ∈ T, or
      (IS) Ancestor(T, x, v).  By Lemma (1), x < v ⇒ u < x < v.  So by Links(ii),
           x ∈ L ↔ Ancestor(T, u, x).  By Ancestor(i), Parent(T, u, x) => Ancestor(T, u, x), so
           x ∈ L.
           Now consider
    Then u < v.  So u ∈ L ↔ Ancestor(T, u, x).
    Assume that ¬(∃ x ∈ L s.t. (x, v) ∈ T) ⇒ ∀ x ∈ L, ¬((x, v) ∈ T).

    Apply WOP to get the minimal v - u s.t. ∀ x ∈ L, ¬((x, v) ∈ T).

    So ∀ x ∈ N,
      u < x < v → x ∈ L ∨ ¬Ancestor(T, u, x)
    ⇒ x - u < v - u → x ∈ L ∨ ¬Ancestor(T, u, x)
    ⇒ u - u < v - u → u ∈ L ∨ ¬Ancestor(T, u, x)
    ⇒ u ∈ L ∨

    Ancestor(T, u, v) ⇒ ∃ x ∈ N s.t. Parent(T, u, x) ⇒ (u, x) ∈ T ⇒ u < x, and either:
      (BC) x = v ⇒ Parent(T, u, v) ⇒ (u, v) ∈ T ⇒ ⊥, or
      (IS) Ancestor(T, x, v). Then Parent(T, u, x), so by Ancestor(i), Ancestor(T, u, x).
           By Lemma(1), x < v ⇒ u < x < v, so by Links(ii), x ∈ L.
           So ∀ w ∈ N, x < w < v → w ∈ L ∨ ¬Ancestor(T, u, w).
           Thus, ¬((x, v) ∈ T).  Then
           So x - u < v - u.
      (IS) Ancestor(T, x, v).  Then by Lemma(1), x < v, which means v was not minimal if ¬((x, v) ∈ T)
           and x ∈ L.
           So
      (IS) Ancestor(T, x, v).  Then by Lemma(1), x < v, so x - v < v - u, which means v - u was not
           minimal if ¬((x,v) ∈ T).  So (x, v) ∈ T ⇒
      (ii) Ancestor(T, x, v).  Then by Lemma(1), x < v, so u < x < v.  By Links(ii), either:
           (i) x ∈ L.  x - u < v - u, so
           which means v was not minimal if x ∈ L.
           So
           If u < x, u < x < v, so by Links(ii), either:
               (i) x ∈ L ⇒ (x, or
               (ii) ¬Ancestor(T, u, x), If x ∈ L,
           By Links(ii), if w ∈ L,
(i) Suppose v ≠ u.  Then by Links(i), Ancestor(T, u, v), so ∃ w ∈ N s.t. Parent(T, u, w) and either
    (i) w = v ⇒ Parent(T, u, v) ⇒ (u, v) ∈ T, or
    (ii) Ancestor(T, w, v).  Then by Links(ii),
         ∀ w ∈ L, ∀ x ∈ N, v < x < w → x ∈ L v ¬Ancestor(T, u, x)
         u < v <

    ¬(v ∈ L) ∧ Ancestor(T, u, v) → (v ≤ ) ∨ (w ≤ v)

         Thus by defn. Ancestor,

(ii) Suppose v = u.
     Then v = min(Links(T, L)).

Now suppose

    Exp
    (ii) ∀ u, v ∈ N s.t. u < v, ∀ w ∈ N s.t. u < w,

    ∀ x ∈ L, ∃ y ∈ L s.t. (x, y) ∈ T v (y, x) ∈ T.

```

```
Suppose ∃ T ∈ Posts(T), u, v ∈ N, u < v.

Let T' = T ∪ {(u, v)}.

Lemma: adding a legal edge (i.e. Posts(T')) never reduces linkage.

Proof:

Suppose L is a valid link in T.

Now, there are four possibilities:

1. ∀ x ∈ L, x ≠ u ∧ x ≠ v.
   Then the link must still be valid in T' (since it's the transitive closure).

2. v ∈ L, but ∀ u' ∈ L, u ≠ u'.  Two sub-possibilities:
   (a) ¬(∃ u' ∈ L s.t. (u', v) ∈ T).
       Then by Lemma (2), v = min(Links(T, L)).  Then Links(i) still trivially holds, and Links(ii) does too since u < v
       and (ii) only cares about v < x < w where w ∈ L.  So Links(T, L).
   (b) ∃ u' ∈ L s.t. (u', v) ∈ T.  By assumption, u ≠ u'.  Then (u, v) and (u', v) are both in T', which violates Posts(T').
       So the edge was not legal to add.

3. u ∈ L, but ∀ v' ∈ L, v ≠ v'.  Two sub-possibilities:
   (a) ¬(∃ v' ∈ L s.t. (u, v') ∈ T).
       Then by Lemma (3), v = max(Links(T, L)).  Then Links(i) still trivially holds, and Links(ii) does too since u < v
       and (ii) only cares about w < x < u where w ∈ L.  So Links(T, L).
   (b) ∃ v' ∈ L s.t. (u, v') ∈ T.  By assumption, v ≠ v'.  Then (u, v) and (u, v') are both in T'.  Two cases:
       (i) u < v < v'.  Then just leave Links as is (Links(ii) doesn't care).
       (ii) u < v' < v.  Then take Link with min = v; tree invariants mean size of that link just needs to be
       removed from the end to preserve the Links count.  This part may be a bit tricky to prove.

4. Both u and v ∈ L.  Then T = T' so the link must still be valid in T'.

Let f(T) = T \ { (x, y) : x = u ∨ y = v }.

Then f(T) = f(T ∪ {(u, v)}) so Linkage(f(T)) = Linkage(f(T u {(u, v)})).

```

```
Lemma: removing a legal edge never increases linkage.
```

```

```

```
Links(T, P, R) ⇔ Posts(T) ∧ S ⊆ T ∧

We want to find the maximum number of "link" pages for any thread topology of n posts, assuming we cache posts and their descendents.

Lemma 1: Maximal number of links for `P` posts is 1 and all maximal forums of `P` posts (`P>= 2`) contain the exact linkage sequence:
  `(x_0, x_1), (x_0, x_2), ..., (x_0, x_{P-1}), (x_0, x_P)`

Proof: Clearly the above sequence satisfies linkage:

Lemma 1: Maximal number of links for `n >= P` posts => forum contains (where `m = n - P`, `P >= 2`):
  `(x_m, x_{m+1}), (x_m, x_{m+2}), ..., (x_m, x_{n-1}), (x_m, x_n)`
up to isomorphism.

Proof:  S

BC: For n = P, the maximum for the graph of P elements is at least 1, since the above does force a link

Suppose not.  Then

Assume not.  Then

Lemma 2: Maximal number of links is found in a forum of the form:
  (x\_0, x\_1), (x\_1, x\_2), ..., (x\_{k-1}, x\_k),
  (x\_k, x\_{k+1}), (x\_k, x\_{k+2}), ..., (x\_k, x\_{k+page::MAX})

(1<<48)-(1<<16)

```
