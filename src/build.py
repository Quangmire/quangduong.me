import argparse
import datetime as dt
import itertools
import os
import shutil

import sass

from process_markdown import process_markdown, fix_pre_code_indent
from process_tags import process_tags
from utils import get_env, calc_min_max_page, paginate

def clean(output):
    # Remove all old files
    if os.path.exists(output):
        for fn in os.listdir(output):
            path = os.path.join(output, fn)
            if os.path.isdir(path):
                shutil.rmtree(path)
            else:
                os.remove(path)

    os.makedirs(output, exist_ok=True)

    shutil.copyfile(src='CNAME', dst=os.path.join(output, 'CNAME'))
    with open(os.path.join(output, '.nojekyll'), 'w'):
        pass

def read_data(output, posts):
    data = {}
    newest_revision = []

    for root, _, file_names in os.walk(posts):
        for file_name in file_names:
            # Preprocess post data
            file_path = os.path.abspath(os.path.join(root, file_name))
            html, metadata = process_markdown(file_path)
            metadata['content'] = html
            metadata['card_tags'] = process_tags(metadata['card_tags'])
            metadata['summary'] = ' '.join(metadata['summary'])
            if '\\\\(' in html or '$$' in html:
                metadata['needs_latex'] = True

            output_path = os.path.join(output, metadata['path'])

            if output_path not in data:
                data[output_path] = []

            data[output_path].append(metadata)

    for output_path in data:
        data[output_path] = sort_by_revision(data[output_path])
        newest_revision.append(data[output_path][-1])

    newest_revision = [newest_revision[i] for i in argsort_by_date(newest_revision)]
    for i in range(len(newest_revision)):
        for revision in data[os.path.join(output, newest_revision[i]['path'])]:
            if i > 0:
                revision['older_path'] = newest_revision[i - 1]['path']
            if i < len(newest_revision) - 1:
                revision['newer_path'] = newest_revision[i + 1]['path']

    return data, newest_revision

def argsort_by_date(data):
    dates = [dt.datetime.strptime(datum['card_date'], '%B %d, %Y | %I:%M %p')
                for datum in data]

    # Argsort
    idx = sorted(range(len(dates)), key=dates.__getitem__)

    return idx

def sort_by_revision(revisions):
    idx = argsort_by_date(revisions)

    for i, old_i in enumerate(idx):
        revisions[old_i]['revision'] = i + 1
        revisions[old_i]['num_revisions'] = len(idx)

    return [revisions[i] for i in idx]

def create_path(path):
    # Create parent dirs
    if not os.path.exists(os.path.dirname(path)):
        try:
            os.makedirs(os.path.dirname(path))
        except OSError as e:
            '''
            If directory created while above call happens it would
            have an errno of EEXIST, so the bottom raise occurs
            otherwise
            '''
            if e.errno != errno.EEXIST:
                raise

def write_data(output, data, template, num_revisions):
    revision = data['revision']
    if revision != num_revisions:
        output_path = os.path.join(output, data['path'], f'r{revision}', 'index.html')
    else:
        output_path = os.path.join(output, data['path'], 'index.html')

    create_path(output_path)

    # Write data
    with open(output_path, 'w') as f:
        html = template.render(**data)
        html = fix_pre_code_indent(html)
        print(html, file=f)

def sort_by_descending_date(data):
    idx = argsort_by_date(data)
    return [data[i] for i in reversed(idx)]

# Currently does not comment out extraneous white space in html since no code is
# in the description as of right now
def write_data_by_tag(output, tag, data, template, num_cards):
    data = sort_by_descending_date(data)
    paginated_data = list(paginate(data, num_cards))
    for i, posts in enumerate(paginated_data):
        output_path = os.path.join(output, 'tag', tag, str(i + 1), 'index.html')
        create_path(output_path)

        with open(output_path, 'w') as f:
            cur_page = i + 1
            num_pages = len(paginated_data)
            min_page, max_page = calc_min_max_page(cur_page, num_pages)

            html = template.render(posts=posts, cur_page=cur_page,
                    num_pages=num_pages, min_page=min_page, max_page=max_page,
                    path=os.path.join('/tag', tag), tag=tag.upper())
            print(html, file=f)

def group_by_year_month(sorted_data):
    dates = [dt.datetime.strptime(datum['card_date'], '%B %d, %Y | %I:%M %p')
                for datum in sorted_data]

    data = {}

    for i in range(len(dates)):
        date = dates[i]

        if date.year not in data:
            data[date.year] = {}

        month = date.strftime('%B')

        if month not in data[date.year]:
            data[date.year][month] = []

        data[date.year][month].append(sorted_data[i])

    return data

def write_archive(output, template, newest_revision, data_by_tag):
    all_data = sort_by_descending_date(newest_revision)
    year_month_data = group_by_year_month(all_data)

    output_path = os.path.join(output, 'archive', 'index.html')
    create_path(output_path)

    tags = process_tags(list(data_by_tag.keys()))
    for tag in tags:
        tag.num = len(data_by_tag[tag.orig_name])

    # Most tags up front
    tags = sorted(tags, key=lambda tag: -tag.num)

    with open(output_path, 'w') as f:
        html = template.render(year_month_data=year_month_data, tags=tags)
        print(html, file=f)

def write_home(output, template, newest_revision, num_cards):
    all_data = sort_by_descending_date(newest_revision)
    paginated_data = list(paginate(all_data, num_cards))

    output_path = os.path.join(output, 'archive', 'index.html')
    create_path(output_path)

    for i, posts in enumerate(paginated_data):
        if i != 0:
            output_path = os.path.join(output, str(i + 1), 'index.html')
        else:
            output_path = os.path.join(output, 'index.html')

        create_path(output_path)

        with open(output_path, 'w') as f:
            cur_page = i + 1
            num_pages = len(paginated_data)
            min_page = max(2, cur_page - 2)
            max_page = min(num_pages, cur_page + 3)

            html = template.render(posts=posts, cur_page=cur_page,
                    num_pages=num_pages, min_page=min_page, max_page=max_page,
                    path='')
            print(html, file=f)

def compile_sass(sass_file, output_file):
    with open(output_file, 'w') as f:
        print(sass.compile(filename=sass_file, output_style='compressed',
            source_map_filename=(sass_file + '.map'),
            omit_source_map_url=True)[0], file=f)

def build(args):
    clean(args.output)

    env = get_env(args.templates)

    post_template = env.get_template('post.html')
    multipost_template = env.get_template('multipost.html')
    archive_template = env.get_template('archive.html')
    home_template = env.get_template('home.html')
    page_not_found_template = env.get_template('404.html')

    data, newest_revision = read_data(args.output, args.posts)
    data_by_tag = {}

    for output_path in data:
        revisions = data[output_path]

        for tag in revisions[-1]['card_tags']:
            if tag.orig_name not in data_by_tag:
                data_by_tag[tag.orig_name] = []
            data_by_tag[tag.orig_name].append(revisions[-1])

        for i in range(len(revisions)):
            write_data(args.output, revisions[i], post_template, len(revisions))

    for tag in data_by_tag:
        write_data_by_tag(args.output, tag, data_by_tag[tag],
            multipost_template, args.num_cards)

    write_archive(args.output, archive_template, newest_revision, data_by_tag)
    write_home(args.output, home_template, newest_revision, args.num_cards)

    compile_sass(os.path.join(args.templates, 'index.scss'),
                 os.path.join(args.static, 'index.css'))

    shutil.copytree(src = args.static, dst = os.path.join(args.output, 'static'))
    shutil.copyfile(src = 'favicon.ico', dst = os.path.join(args.output, 'favicon.ico'))

    with open(os.path.join(args.output, '404.html'), 'w') as f:
        print(page_not_found_template.render(), file=f)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--posts', default='./posts',
            help='Path to root directory of posts')
    parser.add_argument('--templates', default='./templates',
            help='Path to root directory of templates')
    parser.add_argument('--static', default='./static',
            help='Path to directory of static files')
    parser.add_argument('--output', default='./docs',
            help='Path to root directory to output')
    parser.add_argument('--num-cards', type=int, default=5,
            help='Number of cards to put on a page')

    build(parser.parse_args())

if __name__ == '__main__':
    main()
