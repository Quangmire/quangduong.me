import markdown2 as md
import json

# Currently assumes max of one of either <pre> or </pre> on a line
# Should hold so long as none of the code blocks are html with pre inside
def fix_pre_code_indent(html):
    pre_indent = 0
    comment_str = ''
    inside_pre = False
    html = html.split('\n')
    for i in range(len(html)):
        if '<pre>' in html[i]:
            if inside_pre:
                print('Nested <pre> block...currently unsupported')
            inside_pre = True
            pre_indent = len(html[i]) - len(html[i].lstrip(' '))
            comment_str = '<!--' + (' ' * (pre_indent - 7)) + '-->'
        elif '</pre>' in html[i]:
            inside_pre = False
            html[i] = comment_str + html[i][pre_indent:]
        elif inside_pre:
            html[i] = comment_str + html[i][pre_indent:]
    return '\n'.join(html)


def process_markdown(file_name):
    with open(file_name, 'r') as f:
        lines = f.readlines()
    metadata_end = 0
    for i in range(len(lines)):
        if lines[i].strip() == '}':
            metadata_end = i + 1
            break
    metadata = json.loads(''.join(lines[:metadata_end]))
    metadata['markdown'] = ''.join(lines[metadata_end:])
    html = md.markdown(''.join(lines[metadata_end:]),
            extras=['fenced-code-blocks', 'footnotes', 'code-friendly',
                    'header-ids', 'numbering'],
            footnote_title="Jump back to footnote %d in the text.",
            footnote_return_symbol="&#8617;")
    # The return symbol is a backwards hooked arrow
    return html, metadata
