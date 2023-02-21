
<!-- README.md is generated from README.Rmd. Please edit that file -->

# Rapier 2D for R

<!-- badges: start -->
<!-- badges: end -->

## What is Rapier?

[Rapier](https://github.com/dimforge/rapier) is the 2D and 3D physics
engines for the Rust programming language.

## Installation

You can install the development version of rpr2dr like so:

``` r
# FILL THIS IN! HOW CAN PEOPLE INSTALL YOUR DEV PACKAGE?
```

## Example

``` r
library(rpr2dr)

d <- bouncing_ball()

for (i in seq_len(nrow(d))) {
  plot(d[i, ], ylim = c(0, 1), cex = 4)
}
```

<img src="man/figures/README-bouncing_ball-.gif" width="100%" />
