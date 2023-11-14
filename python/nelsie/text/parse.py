from typing import Sequence

from nelsie.export import ExportStyledText
from nelsie.steps.stepsexport import export_step_value
from .texttypes import StyledText, StyledSpan, StyledLine
from .textstyle import TextStyle
from .manager import TextStyleManager, update_stepped_text_style


def _find_first(string, start, c1, c2) -> int:
    p1 = string.find(c1, start)
    p2 = string.find(c2, start)
    if p1 == -1:
        return p2
    if p2 == -1:
        return p1
    return min(p1, p2)


def parse_styled_text(
    text: str,
    delimiters: Sequence[str],
    base_style: TextStyle,
    style_manager: TextStyleManager,
) -> StyledText:
    start_sequence, start_block, end_block = delimiters
    assert start_sequence != start_block
    assert start_sequence != end_block

    styled_lines: list[StyledLine] = []

    style_name_stack: list[str] = []

    style_names: list[list[str]] = []
    styles: list[TextStyle] = []

    def add_chunk(added_text):
        if added_text:
            try:
                style_idx = style_names.index(style_name_stack)
            except ValueError:
                style_idx = len(style_names)
                style_names.append(style_name_stack[:])
                style = base_style
                for name in style_name_stack:
                    style = update_stepped_text_style(
                        style, style_manager.get_style(name)
                    )
                styles.append(style)
            spans.append(StyledSpan(len(raw_line), len(added_text), style_idx))
        return added_text

    for line_idx, line in enumerate(text.split("\n")):
        spans: list[StyledSpan] = []
        raw_line = ""
        buffer = ""
        last_pos = 0
        while True:
            if style_name_stack:
                pos = _find_first(line, last_pos, start_sequence, end_block)
            else:
                pos = line.find(start_sequence, last_pos)
            if pos == -1:
                raw_line += add_chunk(buffer + line[last_pos:])
                break

            buffer += line[last_pos:pos]

            if line[pos] == start_sequence:
                if pos + 1 < len(line) and line[pos + 1] == start_sequence:
                    buffer += start_sequence
                    last_pos = pos + 2
                    continue
                raw_line += add_chunk(buffer)
                buffer = ""
                block_pos = line.find(start_block, pos)
                if block_pos == -1:
                    raise ValueError(f"Invalid style format on line {line_idx + 1}")
                style_name = line[pos + 1 : block_pos]

                style_name_stack.append(style_name)
                last_pos = block_pos + 1
            else:  # line[pos] == end_block
                raw_line += add_chunk(buffer)
                buffer = ""
                style_name_stack.pop()
                last_pos = pos + 1

        styled_lines.append(StyledLine(spans=spans, text=raw_line))
        # abs_pos_index += len(raw_line) + 1  # Becuase of \n

    return StyledText(
        styled_lines=styled_lines,
        styles=styles,
        default_line_spacing=base_style.line_spacing,
        default_font_size=base_style.size,
    )


def export_styled_text(slide, styled_text: StyledText) -> ExportStyledText:
    return ExportStyledText(
        styled_lines=styled_text.styled_lines,
        styles=[
            export_step_value(
                style,
                slide,
            )
            for style in styled_text.styles
        ],
        default_font_size=styled_text.default_font_size,
        default_line_spacing=styled_text.default_line_spacing,
    )
