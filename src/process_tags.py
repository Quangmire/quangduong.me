class Tag:

    TAGS = {
        'about': 'primary-tag',
        'blog': 'primary-tag',
        'notes': 'primary-tag',
        'note': 'primary-tag',
        'projects': 'primary-tag',
    }

    DEFAULT_TAG_CLASS = 'default-tag'

    def __init__(self, name):
        self.name = name.upper()
        self.orig_name = name
        self.tag_class = Tag.TAGS.get(name, Tag.DEFAULT_TAG_CLASS)

def process_tags(tags):
    for i in range(len(tags)):
        tags[i] = Tag(tags[i])
    return tags
