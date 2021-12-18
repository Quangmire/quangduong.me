from jinja2 import Environment, FileSystemLoader, select_autoescape

def get_env(templates):
    return Environment(
        loader = FileSystemLoader(templates),
        autoescape=select_autoescape(['html']),
        lstrip_blocks=True,
        trim_blocks=True,
    )

def compute_current_pagination(cur_page, num_pages):
    start_page = max(1, cur_page - 2)
    if start_page + 4 > num_pages and num_pages - 4 >= 1:
        start_page = num_pages - 4
    pages = []
    if start_page != 1:
        pages.append('.')
    for page in range(start_page, min(start_page + 5, num_pages + 1)):
        pages.append(page)
    if pages[-1] != num_pages:
        pages.append('.')
    return pages

def paginate(iterable, n):
    ret = []
    for item in iterable:
        ret.append(item)
        if len(ret) == n:
            yield ret
            ret = []
    if ret:
        yield ret
