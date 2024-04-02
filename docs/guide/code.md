# Syntax highlighting

Nelsie supports syntax highlighting for many common languages ([The list of supported syntaxes](#the-list-of-supported-syntaxes)).

The `.code()` method creates a box with syntax highlighted text. It works similar to the `.text()` method.
except that it takes a name for the syntax highlighter. You can use the name of the language or a filename extension
for the language

```nelsie
@deck.slide()
def code_demo(slide):
    slide.code("""
fn main() {
    println!("Hello world!")
}""", "Rust")
```


## Default syntax language for syntax highlighting

If you use mostly a single programming through the slides, you can set the default language for syntax highlighting.


```nelsie
deck = SlideDeck(default_code_language="Rust")

@deck.slide()
def code_demo(slide):
    slide.code("""
fn main() {
    println!("Hello world!")
}""")
```


## Styling code

You can change the style by passing the parameter `style`:

```nelsie
@deck.slide()
def code_demo(slide):
    slide.code("print('Hello world!')", "Python", style=TextStyle(size=60))
```


## Named style `"code"`.

The `.code()` method uses the named style `"code"` as the default style.
This allows you to change the code style globally or locally per slide.

```python
deck.update_style("code", TextStyle(size=60))
```


## Inline text styles

In contrast to `.text()`, the `.code()` method does not parse inline styles by default.
But parsing the styles can be enabled by argument `parse_styles`.

```nelsie
@deck.slide()
def code_demo(slide):
    slide.set_style("big", TextStyle(size=80, color="orange"))
    slide.code("print('~big{Hello} world!')", "Python",
               style=TextStyle(size=60), parse_styles=True)
```

If characters "~", "{", "}" clashes with your programming language,
you can change it by the `style_delimiters` parameter.

```nelsie
@deck.slide()
def code_demo(slide):
    slide.set_style("big", TextStyle(size=80, color="orange"))
    slide.code("print('$big<Hello> world!')", "Python",
               style=TextStyle(size=60), parse_styles=True, style_delimiters="$<>")
```


## Syntax highlighting color themes

You can change color theme by passing argument `theme`:

```nelsie
@deck.slide()
def code_demo(slide):
    slide.code("print('Hello world!')", "Python",
               theme="Solarized (light)", style=TextStyle(size=60))
```

You can also change the color theme globally by setting `default_code_theme` in `SlideDeck`:

```python
deck = SlideDeck(default_code_theme="Solarized (light)")
```

The list of supported color themes; the default theme is "InspiredGitHub":

* "base16-ocean.dark"
* "base16-eighties.dark"
* "base16-mocha.dark"
* "base16-ocean.light"
* "InspiredGitHub"
* "Solarized (dark)"
* "Solarized (light)"

Custom color themes can be added through [`Resources`](resources.md).
This list is also programmatically available through [`Resources`](resources.md).

## The list of supported syntaxes

Custom syntax can be added through [`Resources`](resources.md).
This list is also programmatically available through [`Resources`](resources.md)

* ASP (asa)
* ActionScript (as)
* AppleScript (applescript, script editor)
* Batch File (bat, cmd)
* BibTeX (bib)
* Bourne Again Shell (bash) (sh, bash, zsh, fish, .bash_aliases, .bash_completions, .bash_functions, .bash_login, .bash_logout, .bash_profile, .bash_variables, .bashrc, .profile, .textmate_init)
* C (c, h)
* C# (cs, csx)
* C++ (cpp, cc, cp, cxx, c++, C, h, hh, hpp, hxx, h++, inl, ipp)
* CSS (css, css.erb, css.liquid)
* Cargo Build Results ()
* Clojure (clj)
* D (d, di)
* Diff (diff, patch)
* Erlang (erl, hrl, Emakefile, emakefile)
* Go (go)
* Graphviz (DOT) (dot, DOT, gv)
* Groovy (groovy, gvy, gradle)
* HTML (html, htm, shtml, xhtml, inc, tmpl, tpl)
* HTML (ASP) (asp)
* HTML (Erlang) (yaws)
* HTML (Rails) (rails, rhtml, erb, html.erb)
* HTML (Tcl) (adp)
* Haskell (hs)
* JSON (json, sublime-settings, sublime-menu, sublime-keymap, sublime-mousemap, sublime-theme, sublime-build, sublime-project, sublime-completions, sublime-commands, sublime-macro, sublime-color-scheme)
* Java (java, bsh)
* Java Properties (properties)
* Java Server Page (JSP) (jsp)
* JavaDoc ()
* JavaScript (js, htc)
* JavaScript (Rails) (js.erb)
* LaTeX (tex, ltx)
* LaTeX Log ()
* Lisp (lisp, cl, clisp, l, mud, el, scm, ss, lsp, fasl)
* Literate Haskell (lhs)
* Lua (lua)
* MATLAB (matlab)
* Make Output ()
* Makefile (make, GNUmakefile, makefile, Makefile, OCamlMakefile, mak, mk)
* Markdown (md, mdown, markdown, markdn)
* MultiMarkdown ()
* NAnt Build File (build)
* OCaml (ml, mli)
* OCamllex (mll)
* OCamlyacc (mly)
* Objective-C (m, h)
* Objective-C++ (mm, M, h)
* PHP (php, php3, php4, php5, php7, phps, phpt, phtml)
* PHP Source ()
* Pascal (pas, p, dpr)
* Perl (pl, pm, pod, t, PL)
* Plain Text (txt)
* Python (py, py3, pyw, pyi, pyx, pyx.in, pxd, pxd.in, pxi, pxi.in, rpy, cpy, SConstruct, Sconstruct, sconstruct, SConscript, gyp, gypi, Snakefile, wscript)
* R (R, r, s, S, Rprofile)
* R Console ()
* Rd (R Documentation) (rd)
* Regular Expression (re)
* Regular Expressions (Javascript) ()
* Regular Expressions (Python) ()
* Ruby (rb, Appfile, Appraisals, Berksfile, Brewfile, capfile, cgi, Cheffile, config.ru, Deliverfile, Fastfile, fcgi, Gemfile, gemspec, Guardfile, irbrc, jbuilder, podspec, prawn, rabl, rake, Rakefile, Rantfile, rbx, rjs, ruby.rail, Scanfile, simplecov, Snapfile, thor, Thorfile, Vagrantfile)
* Ruby Haml (haml, sass)
* Ruby on Rails (rxml, builder)
* Rust (rs)
* SQL (sql, ddl, dml)
* SQL (Rails) (erbsql, sql.erb)
* Scala (scala, sbt)
* Shell-Unix-Generic ()
* Tcl (tcl)
* TeX (sty, cls)
* Textile (textile)
* XML (xml, xsd, xslt, tld, dtml, rss, opml, svg)
* YAML (yaml, yml, sublime-syntax)
* camlp4 ()
* commands-builtin-shell-bash ()
* reStructuredText (rst, rest)


## Empty language

Language for syntax highlighting can be se to `None`. In such case, no syntax highlighting is used while all other `.code()` properties are used.
