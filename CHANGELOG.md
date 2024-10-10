# v0.14.0

* Dependencies updated; slightly faster rendering and smaller PDFs

# v0.13.0

* `set_steps` method
* TextStyle(bold=True) added
* Smaller binaries (debug info stripped)
* `.move_by` and `.line_by` in Path
* Watching on more Python files

# v0.12.0

* Grid layout
* Fixed printing "total time" twice when verbose mode is enabled

# v0.11.0

## New features

* `path` in `.image()` accepts `InSteps`

## Fixes

* `.code()` type annotation fixed (`text` argument is allowed to be `InSteps`)

## Internal

* Migrated to new version of resvg and svg2pdf
* Parsing of next/last keywords moved to Rust

# v0.10.0

## New features

* Hierarchical steps (e.g. step 1.2.3 is allowed)
* `initial_counter_value` added into `ListBox`

## Changes

* Semantics of steps are now different, but it should be backward compatible with reasonable use cases.
  The only real breaking change is removal of `.set_n_steps` and `.get_n_steps`. They have to be replaced by
  `.insert_step`, `.remove_step`, and `.get_steps`.

# 0.9.1

* No actual changes, created to fix incomplete deployment to PyPI

# 0.9.0

## New features

* Clickable links in PDF output
* Font decorations: underline, overline, line-through
* Watching a slide file and automatic rebuild via `python3 -m nelsie watch SLIDES.PY`

## Removed features

* GIF support was officially removed, as it was not working in the new renderer released in 0.8

## Fixes

* Fixed problem when ListBuilder is used together with steps
* Fixed loading SVG files with DTD

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
