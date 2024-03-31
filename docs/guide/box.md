# Box reference

Box is a basic layout in Nelsie. It represents a rectangular area on the slide.

A new box is created by calling the `.box()` method on an existing box or a slide. This will return a new box that will be a child of the element on which you call the box method.


## Parameters of `.box()`


### Step-related parameters

* `show` - Defines the steps in which the box (its contents and children) is displayed. It only affects the drawing itself. The layout is always calculated, i.e. the space is reserved for the box, even in the steps where it is not painted.
    Takes the following types:
    * `bool` - the box is shown (`True`) or hidden (`False`) in all steps.
    * `int` - the box is shown only in the given step
    * `str` - a string may have the following format:
        * `"<number>"` - the box is shown only in the given step
        * `"<number>+"` - the box is shown in the given step and all following steps
        * `"<number>-<number>"` - the box is shown in the steps in the given range.
        * Comma separated list of the expression above. Then the box is shown in the union of steps defined by expressions. Example: `"1, 5, 20-30, 35+"`.
* `active` - Takes the same parameters as `show` but in steps when the box is not active, it is also removed from the layout, i.e. no space is reserved for the box.
* `replace_steps` - Takes `None` or a dictionary that maps


### Ordering parameters

* `z_level` - An integer value used in painting order. Higher numbers are drawn later. If not set, the value is inherited from the parent box. The default value of the slide root box is `0`.

### Background parameters

* `bg_color`- Sets a background color of the box. If `None`, no background is drawn. Default: `None`.
* `border_radius` - A radius of the box's rounded corner. If `0` then no border is not rounded. Default: `0`.

### Layout parameters

* `x` -- X position of the box
* `y` -- Y position of the box
* `width` -- Width of the box
    * `None` - (default) Automatic size. Minimum size around the content if `flex-grow` / `flex-shrink` is not set.
    * `int` or `float` or `str` containing digits -- A fixed size given in pixels (example: `20.5`, or `"1.5"`)
    * `str` in format `"XX%"` where `XX` is an integer -- A relative size to the parent box, in percent (example: `"50%"`)
    * `LayoutExpr` -- A fixed size defined by a layout expression.
* `height` - Height of the box
    * The paramter takes the same values as `width`.
* `row` - If `True` then the box arranges its children horizontally; otherwise vertically. Default: `False`
* `reverse` - If `True` then child boxes are ordered in the reverse order; i.e. in bottom-up (or right-left if `row` is `True`) Default: `False`.
* `flex_wrap` -
* `flex_grow` - The `flex_grow` parameter takes a `float` value. The default is `0`. This attribute specifies how much of the remaining space of its parent box should be allocated to this box.
    The remaining space is the size of the box minus the size of all its children. If multiple sibling boxes have positive `flex_grow` values, it is distributed according to the ratio defined by their values.
    This property is equivalent to the CSS property `flex-grow`.
* `flex_shrink` - The `flex_grow` parameter takes a `float` value. The default is `0`. If the size of all flex items is larger than the flex container, items shrink to fit according to `flex_shrink`.     This property is equivalent to the CSS property `flex-grow`.
* `align_items` -
* `align_self` -
* `justify_self` -
* `align_content` -
* `justify_content` -
* `gap` -

### Padding parameters

* `p_left` - Left padding
* `p_right` - Right padding
* `p_top` - Top padding
* `p_bottom` - Bottom padding
* `p_x` - Shortcut for setting `p_left` and `p_right` to the same value
* `p_y` - Shortcut for setting `p_top` and `p_bottom` to the same value

### Margin parameters

* `m_left` - Left margin
* `m_right` - Right margin
* `m_top` - Top margin
* `m_bottom` - Bottom margin
* `m_x` - Shortcut for setting `m_left` and `m_right` to the same value
* `m_y` - Shortcut for setting `m_top` and `m_bottom` to the same value


### Debugging parameters

* `name` - The name of the slide. It is displayed when the layout debugging view is enabled.
* `debug_layout` - Enables/disables the layout debugging view for the box. If `True` then the view will be enabled with the
  view will be enabled with the default colour for this box. If `str` is given then the view is enabled and the string is interpreted as the colour of the debugging view (e.g. `debug_view="green"`). If `False` then the debugging view is disabled for this box, even if the whole slide has the debugging view enabled.
