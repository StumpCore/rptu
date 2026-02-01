# Sets
- Two sets are equal if contain the same elements {a,b}={b,a}

# Subsets
A sub B : {2,3,7} sub {1,2,3,4,5,6,7}
A not sub B : {2,3,7} sub {1,2,4,5,6,7}
{} sub B or A
|B| = n then B has 2^n subsets

# Powersets
P(A) = {X:X sub A}
A= {1,2,3} => {}, {1}, {2}, {3}, {1,2}, {1,3}, {2,3}, {1,2,3}
P(A) = {{}, {1}, {2}, {3}, {1,2}, {1,3}, {2,3}, {1,2,3}}
2^|A| = possible sets
{} is part of the power set count! 


# Intervals
empty set = {}
natural numbers = {1,2,3..}
integers= {...,-1,-2,0,1,2,...}
ration nubers = {x:x=m/n, where m,n elements of integers and n!=0}
real numbers = {all other}

[a,b] = {x ele R: a <= x <=b}
(a,b) = {x ele R: a < x < b}
(a,b] = {x ele R: a < x <= b}
[a,b) = {x ele R: a <= x <= b}
(a,infty) = {x ele R: a < x}
[a,infty) = {x ele R: a <= x}
(-infty,b) = {x ele R: x < b}
(-infty,b] = {x ele R: x <= b}

# Order pair
(x,y) != (y,x)

# Cartesian product
AxB = {(a,b): a in A, b in B}

| k | l| m| n|
| --------------- | --------------- | --------------- | --------------- |
r| (k,r)| (l,r) | (m,r) | (n,r) |
q| (k,q)| (l,q) | (m,q) | (n,q) |

if A and B a finite sets : |AxB|=|A|x|B|

A^n = AxAxAx...xA_n

# Union, Intersection, Difference
AuB = {x: x in A **or** x in B} = BuA
AnB = {x: x in A **and** x in B} = BnA
A\B = {x: x in A **and** x not in B} != B\A

AnBnC = (AnB)nC = An(BnC)
AuBuC = (AuB)uC = Au(BuC)
(AuB)nC != Au(BnC)

A1uA2u...An = *at least one set* = everything that is in at *least one* of the sets.
This means the resulting union contains all values, whereby each value is in at least one of the sets.
This must not be the same set but can be.

A1nA2n...An = *for every set*
This means the resulting set-values are in all sets!

# Coplements
U = Universal Set
Â= U-A




# Chapter 1 
### Foundations of Probability Spaces
1. A probability space is defined by the triplet $(\Omega, \mathcal{A}, P)$
2. Sample Space ($\Omega$): The set of all possible outcomes
3. $\sigma$-Algebra ($\mathcal{A}$): A collection of subsets of $\Omega$ that satisfies:
    1. $\Omega \in \mathcal{A}$
    2. If $A \in \mathcal{A}$, then $A^c \in \mathcal{A}$ (closure under complement)
    3. If $A_1, A_2, \dots \in \mathcal{A}$, then $\bigcup_{i=1}^{\infty} A_i \in \mathcal{A}$ (closure under countable unions)
4. Borel $\sigma$-Algebra ($\mathcal{B}$): The smallest $\sigma$-algebra containing all open sets in $\mathbb{R}$
5. Kolmogorov’s Axioms for Measure ($P$):
    1. $P(A) \ge 0$ for all $A \in \mathcal{A}$
    2. $P(\Omega) = 1$ 
6. $\sigma$-additivity: For pairwise disjoint sets $A_1, A_2, \dots$, $P(\bigcup_{i=1}^{\infty} A_i) = \sum_{i=1}^{\infty} P(A_i)$

### Essential Probability Formulas
1. Complement Rule: $P(A^c) = 1 - P(A)$ 
2. Addition Rule: $P(A \cup B) = P(A) + P(B) - P(A \cap B)$ 
3. Monotonicity: If $A \subseteq B$, then $P(A) \le P(B)$ 
4. Inclusion-Exclusion Principle: $P(\bigcup_{i=1}^n A_i) = \sum P(A_i) - \sum P(A_i \cap A_j) + \dots + (-1)^{n-1} P(\bigcap A_i)$ 
5. Continuity of Probability:
    1. If $A_n \uparrow A$ (increasing), then $\lim P(A_n) = P(A)$ 
    2. If $A_n \downarrow A$ (decreasing), then $\lim P(A_n) = P(A)$ 

### Combinatorics & Ordered/Unordered Sampling
1. For a set of $N$ objects and a sample of size $n$:
    1. Ordered with replacement: $N^n$ 
    2. Ordered without replacement (Permutations): $P_N(n) = \frac{N!}{(N-n)!}$
    3. Unordered without replacement (Combinations): $\binom{N}{n} = \frac{N!}{n!(N-n)!}$
2. Binomial Coefficient Identity: $\sum_{k=0}^n \binom{n}{k} = 2^n$ 
3. Vandermonde's Identity: $\sum_{i} \binom{n}{i} \binom{k}{m-i} = \binom{n+k}{m}$ (used to calculate sums of independent discrete trials)

### Random Variables & Distributions
1. Random Variable ($X$): A measurable mapping $X: \Omega \to \mathbb{R}$ such that $\{X \in B\} \in \mathcal{A}$ for any Borel set $B$ 
2. Cumulative Distribution Function (CDF): $F_X(x) = P(X \le x)$. Properties: $F_X$ is non-decreasing, right-continuous, $\lim_{x \to -\infty} F_X(x) = 0$, and $\lim_{x \to \infty} F_X(x) = 1$
3. Discrete Uniform Distribution: $P(A) = \frac{|A|}{|\Omega|}$ for finite $\Omega$
4. Continuous Uniform Distribution: $f_X(x) = \frac{1}{b-a}$ for $x \in a, b$ 
5. Hypergeometric Distribution: $P(X=k) = \frac{\binom{M}{k}\binom{N-M}{n-k}}{\binom{N}{n}}$ (sampling without replacement) 
6. Quantile Function: $q_F(\alpha) = \inf\{t \in \mathbb{R} : F(t) \ge \alpha\}$ 

### Algebraic and Geometric Reminders
1. Stirling’s Approximation: $n! \approx \sqrt{2\pi n} \left(\frac{n}{e}\right)^n$ 
2. Geometric Series: $\sum_{j=k}^\infty r^j = \frac{r^k}{1-r}$ for $|r| < 1$ 
3. Standard Integrals: $\int_{-\infty}^{\infty} e^{-x^2/2} dx = \sqrt{2\pi}$
4. Algebraic Error Trap: $(x+a)^n \ne x^n + a^n$


### Proof
Let $A_0 := \emptyset$                                                                     
For disjoint sets: $P\left(\bigcup_{i=1}^{\infty} B_i\right) = \sum_{i=1}^{\infty} P(B_i)$ 
Define: $B_k := A_k \backslash A_{k-1}$ for each $k$ 
Verify: The sets $\{B_k\}_{k \in \mathbb{N}}$ are pairwise disjoint                        
Show: $\bigcup_{k=1}^{\infty} B_k = A$                                                     
Apply σ-additivity: $P(A) = \sum_{k=1}^{\infty} P(A_k \backslash A_{k-1})$                 
Recognize: $\sum_{k=1}^{n} P(A_k \backslash A_{k-1}) = P(A_n)$                             
Take limit: $\lim_{n \to \infty} P(A_n) = P(A)$                                            

