import json
import book_pb2
import protobuf_frontend as pb
from google.protobuf.json_format import MessageToJson, Parse

aliases = (
    ["1Ne", "1Ne.", "1 Ne", "1 Ne.", "1Nephi", "1 Nephi"],
    ["2Ne", "2Ne.", "2 Ne", "2 Ne.", "2Nephi", "2 Nephi"],
    ["Jac", "Jac.", "Jacob"],
    ["Enos", "Eno", "Eno."],
    ["Jar", "Jar.", "Jarom"],
    ["Omni", "Omn", "Omn."],
    ["WofM", "W of M", "Words of Mormon", "WMN"],
    ["Mos", "Mos.", "Mosiah", "Msh", "Msh."],
    ["Alma", "Alm", "Alm."],
    ["Hel", "Hel.", "Helaman"],
    ["3Ne", "3Ne.", "3 Ne", "3 Ne.", "3Nephi", "3 Nephi"],
    ["4Ne", "4Ne.", "4 Ne", "4 Ne.", "4Nephi", "4 Nephi"],
    ["Morm", "Morm.", "Mormon", "Mor", "Mor."],
    ["Eth", "Eth.", "Ether"],
    ["Moro", "Moro.", "Moroni", "Mni", "Mni."],
)

def get_bom_json():
    with open("scriptures-json/book-of-mormon.json") as f:
        data = json.load(f)
    return data

def create_chapter(data, book, chapter):
    elements = []
    if chapter == 0:
        elements.append(pb.el.heading(data["books"][book]["full_title"]))

    if chapter == 0 and "heading" in data["books"][book]:
        elements.append(pb.el.paragraph(data["books"][book]["heading"]))

    if "heading" in data["books"][book]["chapters"][chapter]:
        elements.append(pb.el.paragraph(data["books"][book]["chapters"][chapter]["heading"]))

    for i in range(len(data["books"][book]["chapters"][chapter]["verses"])):
        elements.append(pb.el.verse(data["books"][book]["chapters"][chapter]["verses"][i]["text"]))
    return pb.document_item(str(chapter+1), elements)

def create_book(data, book):
    elements = []
    for i in range(len(data["books"][book]["chapters"])):
        elements.append(create_chapter(data, book, i))
    return pb.grouping_item(data["books"][book]["book"], aliases[book], True, "", elements)

def marshal_json_to_pb(data):
    print(data["title"])
    print(data["books"][0]["chapters"][0]["verses"][0]["text"])

    bom_children = [
        pb.grouping_item(
            title = "Introduction and Witnesses",
            aliases = [],
            named_in_ref = False,
            children_prefix = "",
            children = [
                pb.document_item("Title Page", [
                    pb.el.heading(data["title"]),
                    pb.el.subheading(data["title_page"]["subtitle"]),
                    pb.el.paragraph(data["title_page"]["text"][0]),
                    pb.el.paragraph(data["title_page"]["text"][1]),
                    pb.el.subheading(data["title_page"]["translated_by"]),
                ]),
                pb.document_item("Testimony of Three Witnesses", [
                    pb.el.heading(data["testimonies"][0]["title"]),
                    pb.el.paragraph(data["testimonies"][0]["text"]),
                    pb.el.subheading(data["testimonies"][0]["witnesses"][0]),
                    pb.el.subheading(data["testimonies"][0]["witnesses"][1]),
                    pb.el.subheading(data["testimonies"][0]["witnesses"][2]),
                ]),
                pb.document_item("Testimony of Eight Witnesses", [
                    pb.el.heading(data["testimonies"][1]["title"]),
                    pb.el.paragraph(data["testimonies"][1]["text"]),
                    pb.el.subheading(data["testimonies"][1]["witnesses"][0]),
                    pb.el.subheading(data["testimonies"][1]["witnesses"][1]),
                    pb.el.subheading(data["testimonies"][1]["witnesses"][2]),
                    pb.el.subheading(data["testimonies"][1]["witnesses"][3]),
                    pb.el.subheading(data["testimonies"][1]["witnesses"][4]),
                    pb.el.subheading(data["testimonies"][1]["witnesses"][5]),
                    pb.el.subheading(data["testimonies"][1]["witnesses"][6]),
                    pb.el.subheading(data["testimonies"][1]["witnesses"][7]),
                ]),
            ]
        ),
    ]

    for i in range(len(data["books"])):
        bom_children.append(create_book(data, i))

    group = pb.grouping_item(
        title = data["title"],
        aliases = [],
        named_in_ref = False,
        children_prefix = "Book",
        children = bom_children,
            # pb.grouping_item(data["books"][0]["book"], ["1ne", "1ne.", "Nephi"], True, "Chapter", [
            #     create_chapter(data, 0, 0),
            #     create_chapter(data, 0, 1),
                # pb.document_item("1", [
                    # pb.el.heading(data["books"][0]["full_title"]),
                    # pb.el.subheading("His reign and ministry"),
                    # pb.el.paragraph(data["books"][0]["heading"]),
                    # pb.el.verse(data["books"][0]["chapters"][0]["verses"][0]["text"]),
                    # pb.el.verse(data["books"][0]["chapters"][0]["verses"][1]["text"]),
                    # pb.el.verse(data["books"][0]["chapters"][0]["verses"][2]["text"]),
                    # pb.el.verse(data["books"][0]["chapters"][0]["verses"][3]["text"]),
                    # pb.el.verse(data["books"][0]["chapters"][0]["verses"][4]["text"]),
                    # pb.el.verse(data["books"][0]["chapters"][0]["verses"][5]["text"]),
                    # pb.el.verse(data["books"][0]["chapters"][0]["verses"][6]["text"]),
                    # pb.el.verse(data["books"][0]["chapters"][0]["verses"][7]["text"]),
                    # pb.el.verse(data["books"][0]["chapters"][0]["verses"][8]["text"]),
                # ])
            # ])
    )
    return group

def serialize_to_pb(item, filename):
    with open(filename, 'wb') as f:
        f.write(item.SerializeToString())

def serialize_to_json(item, filename):
    json_str = MessageToJson(item)
    with open(filename, 'w') as f:
        f.write(json_str)

def main():
    bom_json = get_bom_json()
    pb_data = marshal_json_to_pb(bom_json)
    serialize_to_json(pb_data, "book_data.json")
    serialize_to_pb(pb_data, 'book_data.pb')

if __name__ == "__main__":
    main()


