# The problem
We want internal relative difficulty: 0-10 (???) indicates how hard a problem
is relative to the other problems in that topic. 
This ensures nice distribution in stencil.

However, when looking at the overall course the difficulties might not line up.
Introductory topics are generally easier - and a 5 at the beginning does not equal a 5 in the end.

## Each problem gets two difficulties: an internal one and an external one [WE'LL DO THIS]
**Pros**: Stencils are often generated with two purposes: drilling one 
specific topic or more comprehensive revising. This accounts for both of these scenarios.
**Cons**: A bit clumsy when including two or three topics (if a lot of problems are condensed into the same difficulty).

Do we want 0 - 10 still?

Internal difficulty doesn't need a max. We could simply assign sequential numbers to harder and harder problems.
A 4 should only come after a 3 has already appeared. If two problems are equivalent in difficulty, they share a number.
Problems should appear sequentially in the code as well.

Still keep 0 - 10 for external difficulty! Good future-proofing

This would allow the user to still filter according to external difficulty (E, C, A), but gives us more granularity in how to structure problems
in the stencil.

### How would the selection algorithm work?
We do not have to decide on this before implementing new difficulties!

- If **one** topic:
    - First apply the filter (intro - easy, easy - hard, etc.)
    - Of all the problems that are left, start with the easiest one
    - Number of problems per difficulty number should be relative to the number of problems.
        Say there are problems between 5 and 10 (we know they are sequential now), and we need 35 problems. 
        We can fit 5 problems per difficulty + 5 extra ones. 
