
<!-- README.md is generated from README.Rmd. Please edit that file -->

# rapierr

Bring Rapier 2D functionality to R

<!-- badges: start -->
<!-- badges: end -->

## What is Rapier?

[Rapier](https://github.com/dimforge/rapier) is the 2D and 3D physics
engines for the Rust programming language.

## Installation

You can install the development version of rapierr like so:

``` r
remotes::install_github("yutannihilation/rapierr")
```

## Example

``` r
library(rapierr)
library(ggplot2)

d <- bouncing_ball()

p <- ggplot() +
  ggforce::geom_circle(aes(x0 = x, y0 = y, r = 0.07, fill = index)) +
  scale_size_identity() +
  theme_minimal() +
  theme(legend.position = "none") +
  coord_equal(xlim = c(-0.5, 0.5), ylim = c(0, 1))

for (frame in seq_len(max(d$frame))) {
  plot(p %+% dplyr::filter(d, frame == {{ frame }}))
}
```

<img src="man/figures/README-bouncing_ball-.gif" width="100%" />
