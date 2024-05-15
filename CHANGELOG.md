# Unreleased

## New features

* Watching a slide file and automatic rebuild via `python3 -m nelsie watch SLIDES.PY`

## Removed features

* GIF support was officially removed, as it was not working in the new renderer released in 0.8


# 0.8

## New features

* New PDF renderer. Nelsie now produce smaller slides faster when raster images are used
* Rendering slides & image preprocessing in parallel


# 0.7

## Fixes

* Fixed #19


# 0.6

## New features

* Subslides insertion (`.slide_at()`)
* Custom code syntaxes and code color themes


# 0.5

## New features

* text itself can be `InSteps`
* `ListBox` helper added
* `default_code_language` added (by @fgelm01)
* `default_theme` renamed to `default_code_theme`


# 0.4

## New features

* verbose level & progress bar
