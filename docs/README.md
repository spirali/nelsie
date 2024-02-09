<p align="center">
<img src='imgs/nelsie-logo.jpg' width='400'>
</p>

# Nelsie

Nelsie allows you to create slides programmatically using Python. It is a library
with a Python API and a renderer written in Rust.
The output is a PDF file or a set of SVG/PNG files.

There is no DSL or GUI; presentations created with Nelsie are fully programmed in Python.
We believe that creating presentations in a programmable way
makes the process of creating slides smoother and more reliable.

Nelsie focuses on controlling what the audience sees, so you can continuously reveal fragments of the slide,
or simply manage which parts are highlighted.


## History

Nelsie is a complete rewrite of the previous project [Elsie](https://github.com/spirali/elsie). Nelsie solves the biggest problems of Elsie: Dependency on Inkscape as a rendering engine (This makes Elsie difficult to install on some systems; performance issues and problems when Inkscape changes its programming API). This is solved by a own rendering engine (based on [resvg](https://github.com/RazrFalcon/resvg)) included in the Nelsie package. Nelsie also offers many API improvements, namely the introduction of InSteps and the Flexbox layout engine.