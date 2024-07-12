import book_pb2

class el:
    @staticmethod
    def verse(text: str):
        result = book_pb2.DocumentElement()
        result.verse = text
        return result

    @staticmethod
    def paragraph(text: str):
        result = book_pb2.DocumentElement()
        result.paragraph = text
        return result

    @staticmethod
    def heading(text: str):
        result = book_pb2.DocumentElement()
        result.heading = text
        return result

    @staticmethod
    def subheading(text: str):
        result = book_pb2.DocumentElement()
        result.subheading = text
        return result

def document_item(title: str, elements: list[book_pb2.DocumentElement]):
    return _item(_document(title, elements))

def _document(title: str, elements: list[book_pb2.DocumentElement]):
    result = book_pb2.Document()
    result.title = title
    result.elements.extend(elements)
    return result

def grouping_item(title: str, aliases: list[str], named_in_ref: bool, children_prefix: str = None, children: list[book_pb2.Item] = []):
    return _item(_grouping(title, aliases, named_in_ref, children_prefix, children))

def _grouping(title: str, aliases: list[str], named_in_ref: bool, children_prefix: str = None, children: list[book_pb2.Item] = []):
    result = book_pb2.Grouping()
    result.title = title
    result.aliases.extend(aliases)
    result.named_in_ref = named_in_ref
    if children_prefix != None:
        result.children_prefix = children_prefix
    result.children.extend(children)
    return result

def _item(item):
    if type(item) is book_pb2.Document:
        result = book_pb2.Item()
        result.document.CopyFrom(item)
        return result

    elif type(item) is book_pb2.Grouping:
        result = book_pb2.Item()
        result.group.CopyFrom(item)
        return result

    raise Exception("Invalid data type entered")

