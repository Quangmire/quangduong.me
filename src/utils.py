from jinja2 import Environment, FileSystemLoader, select_autoescape

def get_env(templates):
    return Environment(
        loader = FileSystemLoader(templates),
        autoescape=select_autoescape(['html']),
        lstrip_blocks=True,
        trim_blocks=True,
    )

def calc_min_max_page(cur_page, num_pages):
    min_page = max(0, cur_page - 3)
    max_page = min(num_pages, cur_page + 2)

    if cur_page < 4:
        min_page += 1
        max_page = min(num_pages, cur_page + (5 - cur_page))

    if cur_page > num_pages - 3:
        max_page -= 1
        min_page = max(0, cur_page - (5 - (num_pages - cur_page)))

    return (min_page, max_page)

def paginate(iterable, n):
    ret = []
    for item in iterable:
        ret.append(item)
        if len(ret) == n:
            yield ret
            ret = []
    if ret:
        yield ret

