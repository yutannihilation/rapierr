
<!-- README.md is generated from README.Rmd. Please edit that file -->

# rapierr

Bring Rapier 2D functionality to R

<!-- badges: start -->
<!-- badges: end -->

:warning: :warning: :warning: :warning: :warning: :warning: :warning:
:warning: :warning: :warning:

Don’t expect this package to work at the moment.

:warning: :warning: :warning: :warning: :warning: :warning: :warning:
:warning: :warning: :warning:

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

world <- Rapier2DWorld$new()

# ground
world$add_fixed_polyline(-50.0, 0.0, 50.0, 0.0, 0.70);

# left wall
world$add_fixed_polyline(-0.3, 0.0, -0.2, 100.0, 0.70);

world$add_ball(0.0, 1.0, 0.08, 0.97)
world$add_ball(0.01, 1.2, 0.08, 0.97)
world$add_ball(0.02, 0.6, 0.08, 0.97)
world$add_ball(-0.01, 0.5, 0.08, 0.97)
world$add_ball(-0.02, 0.8, 0.08, 0.97)

d <- world$step(200)

p <- ggplot() +
  ggforce::geom_circle(aes(x0 = x, y0 = y, r = 0.08, fill = index)) +
  scale_size_identity() +
  theme_minimal() +
  theme(legend.position = "none") +
  coord_equal(xlim = c(-0.5, 0.5), ylim = c(0, 1))

for (frame in seq_len(max(d$frame))) {
  plot(p %+% dplyr::filter(d, frame == {{ frame }}))
}
```

<img src="man/figures/README-bouncing_ball-.gif" width="100%" />
