
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
library(ggplot2)

d <- bouncing_ball()
d <- d[1:200,]

for (i in seq_len(nrow(d))) {
  p <- ggplot(d[i, ]) +
    geom_point(aes(x, y), size = 10) +
    scale_size_identity() +
    coord_cartesian(ylim = c(0, 1))
  plot(p)
}
```

<img src="man/figures/README-bouncing_ball-.gif" width="100%" />
