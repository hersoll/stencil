# The problem
We want internal relative difficulty: 0-10 indicates how hard a problem
is relative to the other problems in that topic. 
This ensures nice distribution in stencil.

However, when looking at the overall course the difficulties might not line up.
Introductory topics are generally easier - and a 5 at the beginning does not equal a 5 in the end.

# Possible solutions
## Each topic gets a "relative" modifier
Easier topics get, say, a -2 in difficulty. A problem that would be a 5 becomes a 3 instead.
**pros**: Easy to implement
**cons**: Very rigid

## Each problem gets two difficulties: an internal one and an external one [WE'LL DO THIS]
**Pros**: Stencils are often generated with two purposes: drilling one 
specific topic or more comprehensive revising. This accounts for both of these scenarios.
**Cons**: A bit clumsy when including two or three topics (if a lot of problems are condensed into the same difficulty).


# Other things we need to fix:
- New selection algorithm. Not strict intro/easy/medium/hard divide, allow mixing some 4s with the 5s.
- Difficulty association. Currently the difficulty resides with the problem in the data. But the same problem 
    could be used in different courses, and might have different difficulties depending on the course.
    Should be stored with topic_problems table?
