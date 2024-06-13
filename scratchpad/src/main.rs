
#[derive(Debug)]
enum Item { // Acts as a tree
    Group(Grouping), // Branch
    Document(Document), // Leaf
}

#[derive(Debug)]
struct Grouping { // Node
    title: String, // "Standard Works", "New Testament", "Matthew"
    aliases: Vec<String>, // "OT", "Matt"
    named_in_ref: bool, // Whether or not the name/aliases should be searched through when locating a reference

    children_prefix: Option<String>,
    children: Vec<Item>,
}

#[derive(Debug)]
struct Document {
    title: String, // "1"
    elements: Vec<DocumentElement>,
}

#[derive(Debug)]
enum DocumentElement {
    Verse(String),
    Paragraph(String),
    Heading(String),
    Subheading(String),
    //LIST(DocumentList),
}

//#[derive(Debug)]
//struct DocumentList {
//    list_type: DocumentListType,
//    text: Vec<String>,
//}
//
//#[derive(Debug)]
//enum DocumentListType {
//    ORDERED,
//    UNORDERED,
//}

fn main() {
    //println!("Hello, world!");
    let standard_works = Item::Group(Grouping {
        // Example data as it would be unmarshaled from a protobuf
        title: "Standard Works".to_string(),
        aliases: vec![],
        named_in_ref: false,
        children_prefix: None,
        children: vec![
            Item::Group( Grouping {
                title: "New Testament".to_string(),
                aliases: vec!["NT".to_string()],
                named_in_ref: false,
                children_prefix: None,
                children: vec![
                    Item::Document( Document{
                        title: "Title Page".to_string(),
                        elements: vec![
                            DocumentElement::Heading("The New Testament".to_string()),
                            DocumentElement::Subheading("Of Our LORD and Saviour Jesus Christ".to_string()),
                            DocumentElement::Subheading("Translated out of the original Greek ...".to_string())
                        ],
                    }),
                    Item::Group( Grouping {
                        title: "Matthew".to_string(),
                        aliases: vec!["Matt".to_string(), "Mat".to_string()],
                        named_in_ref: true,
                        children_prefix: Some("Chapter".to_string()),
                        children: vec![
                            Item::Document( Document {
                                title: "1".to_string(),
                                elements: vec![
                                    DocumentElement::Heading("The gospel according to St. Matthew".to_string()),
                                    DocumentElement::Verse("The book of the generation of Jesus Christ, the son of David, the son of Abraham.".to_string()),
                                ]
                            }),
                        ]
                    }),
                ]
            }),
            Item::Group( Grouping {
                title: "Book of Mormon".to_string(),
                //long_title: Some("The Book of Mormon: Another Testament of Jesus Christ".to_string()),
                //subtitle: Some("An account written by the hand of Mormon upon plates taken from the plates of Nephi".to_string()),
                aliases: vec!["BoM".to_string()],
                named_in_ref: false,
                children_prefix: None,
                children: vec![
                    Item::Group( Grouping {
                        title: "Introduction and Witnesses".to_string(),
                        aliases: vec!["I&W".to_string()],
                        named_in_ref: true,
                        children_prefix: None,
                        children: vec![],
                    }),
                    Item::Group( Grouping {
                        title: "1 Nephi".to_string(),
                        aliases: vec!["1ne".to_string(), "1 Ne.".to_string()],
                        named_in_ref: true,
                        children_prefix: Some("Chapter".to_string()),
                        children: vec![
                            Item::Document( Document {
                                title: "1".to_string(),
                                elements: vec![
                                    DocumentElement::Heading("The first book of Nephi".to_string()),
                                    DocumentElement::Subheading("His reign and ministry".to_string()),
                                    DocumentElement::Paragraph("An account of Lehi and his wife Sariah ...".to_string()),
                                    DocumentElement::Verse("I, Nephi, having been born of goodly parents,...".to_string()),
                                ]
                            }),
                        ]
                    }),
                ]
            }),
        ]
    });
    println!("{:?}", standard_works);
}
