---

---

# A taste of Rust

## Introduction
As fast as `C++`, as ergonomic as `Python`: often better the best of bost worlds. Rust
is a "systems" programming language that values performance, reliability, and productivity.

We assume you're here because you're heard of rust, maybe used it a little bit, and want to find out
if you should dive into learning it properly. Today we will introduce and motivate the use of Rust,
tour the ecosystem surrounding the language, quickly introduce Rust syntax, and briefly describe
how to use Rust within `C/C++`, `R`, and `Python` worlds.

## Why use Rust as a Python or R programmer
Many of the day-to-day tasks involve numerical computing, sometimes on a large scale. Where you need
performance the most common tools you have available to you are:
- Write performance sentitive code in `C/C++`
- Cython
- NumPy

While Cython and NumPy are great, there may be occasions where they do not apply or you cannot achieve
the performance you need. And writing `C/C++` is a major barrier due to their lack or ergonomics and
unsafety.

This is where Rust comes in. You can now write your performance sensitive code in an ergonomic and
safe language. Furthermore, you may find that writing Rust is in some ways _more_ ergonomic than
Python or R!

## Why use Rust as a C programmer
`C` is a beautifully simple language. As a `C` programmer you have no doubt built your list of
idioms over the years to help you be effective. Nevertheless, `C` lacks many features that can make
you much more effective. Memory safety and an expansive collections library jump to mind.

## Why use Rust as a C++ programmer
`C++` is an extremely complex language that has maintained backwards compatibility (including with `C`)
for decades. Unfortunately, in order to provide programmers with the excellent features to make us
effective `C++` must continually become more complex.

## Can Rust be used for Scientific computing?
Day-to-day you will need to write software that interacts with _legacy_ systems written in Python, R,
C, C++, maybe even Fortran. With Rust, you can still do all of these things.
