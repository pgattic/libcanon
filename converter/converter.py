import book_pb2
import protobuf_frontend as pb
from google.protobuf.json_format import MessageToJson, Parse

def create_sample_data():
    # Create some DocumentElements
    verse = pb.el.verse("In the beginning, God created the heavens and the earth.")
    paragraph = pb.el.paragraph("This is the first paragraph.")
    heading = pb.el.heading("Genesis")
    subheading = pb.el.subheading("Creation of the world")

    # Create a Document
    document = pb.document_item("1", [verse, paragraph, heading, subheading])

    # Create a Grouping
    group = pb.grouping_item("The Holy Bible", ["Bible", "hb"], True, "Book", [])
    
    print(type(document))
    print(type(43))
    print(type(verse))
    print(type(group))

    return group

def serialize_to_pb(item, filename):
    with open(filename, 'wb') as f:
        f.write(item.SerializeToString())

def serialize_to_json(item, filename):
    json_str = MessageToJson(item)
    with open(filename, 'w') as f:
        f.write(json_str)

def main():
    sample_data = create_sample_data()
    serialize_to_json(sample_data, "book_data.json")
    serialize_to_pb(sample_data, 'book_data.pb')

if __name__ == "__main__":
    main()

