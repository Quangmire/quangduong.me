class Tag:

    TAGS = {
        'about': 'var(--tag-color)',
        'blog': 'var(--tag-color)',
        'notes': 'var(--tag-color)',
        'projects': 'var(--tag-color)',
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
