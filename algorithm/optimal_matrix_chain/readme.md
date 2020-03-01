# optimal matrix chain product

Given a list of matrix dimensions [a0, a1, ..., aN], there are (4^N)/(N^1.5) ways to parenthesize the product (this is a catalan number), which is exponential. Dynamic programing gives a polynomial time algorithm to find the optimal parenthesization.