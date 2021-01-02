class Tag:

    TAGS = {
        'about': 'var(--primary-color)',
        'blog': 'var(--primary-color)',
        'notes': 'var(--primary-color)',
        'projects': 'var(--primary-color)',
    }

    DEFAULT_COLOR = 'var(--default-tag-color)'

    def __init__(self, name):
        self.name = name.upper()
        self.orig_name = name
        self.color = Tag.TAGS.get(name, Tag.DEFAULT_COLOR)

def process_tags(tags):
    for i in range(len(tags)):
        tags[i] = Tag(tags[i])
    return tags
