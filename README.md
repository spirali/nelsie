<p align="center">
<img src='docs/imgs/nelsie-logo.jpg' width='400'>
</p>

# Nelsie

* New Elsie: New generation of [Elsie](https://github.com/spirali/elsie)
* New fast rendering engine written in Rust (independent on Inkscape)
* New fragment reveling philosophy (removes many needs of overlays)
* Flexbox layout

* State: **Early prototype**

## TODO

* General
    * ~~Rendering PDF, SVG, PNG~~
    * ~~@slide decorator~~
    * ~~Step values~~
    * Equivalent for "next"/"next+"/"last"/"last+" (not as keywords but as an explicit counter)
    * ~~Box debugging visualization~~
    * Parallel rendering [if needed]
    * Box rotations
    * Jupyter support
    * Slide viewbox
    * Slide post processing callback
* Layout
    * ~~Box size~~
    * ~~Direction~~
    * ~~z-level~~
    * Min size & Max size
    * Aspect ratio
    * ~~Margin~~
    * ~~Padding~~
    * Align & Justify items
    * Gap
    * ~~Flex grow & shrink~~
    * ~~Absolute positioning~~
    * Flex basis
    * Positions derived from other boxes
* Shapes
    * ~~Box background color~~
    * Box border color
    * Lines
    * Ellipse & Polygon
    * ~~Paths~~
    * Arrows
    * Rounded box corners
    * ~~Opacity~~
* Text
    * ~~Style parsing & text rendering~~
    * ~~Steps in styles~~
    * ~~All basic TextStyle properties~~
    * TextStyle priorities
    * ~~Text align: left, right, center~~
    * ~~Detecting invalid fonts~~
    * Line box
    * Inline box
    * ~~Syntax highlight~~
    * Merging own styles & syntax highlight
    * Fit-in-box rendering
    * ~~Opacity~~
* Images
    * ~~loading SVG image + fragments~~
    * ~~loading raster images~~
    * ~~loading ORA + fragments~~
    * ~~ignore fragments~~
    * ~~shift_first_step~~
    * select steps
    * detect invalid font in SVG image
