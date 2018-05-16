extern crate phf_codegen;
extern crate injection;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use injection::libinjection_xss::Attribute;

const X_ACTION: &[u8] = b"action";
const X_ATTRIBUTENAME: &[u8] = b"attributename";
const X_BY: &[u8] = b"by";
const X_BACKGROUND: &[u8] = b"background";
const X_DATAFORMATAS: &[u8] = b"dataformatas";
const X_DATASRC: &[u8] = b"datasrc";
const X_DYNSRC: &[u8] = b"dynsrc";
const X_FILTER: &[u8] = b"filter";
const X_FORMACTION: &[u8] = b"formaction";
const X_FOLDER: &[u8] = b"folder";
const X_FROM: &[u8] = b"from";
const X_HANDLER: &[u8] = b"handler";
const X_HREF: &[u8] = b"href";
const X_LOWSRC: &[u8] = b"lowsrc";
const X_POSTER: &[u8] = b"poster";
const X_SRC: &[u8] = b"src";
const X_STYLE: &[u8] = b"style";
const X_TO: &[u8] = b"to";
const X_VALUES: &[u8] = b"values";
const X_XLINKHREF: &[u8] = b"xlink:href";

fn main() {
    attrs();
    tags();
    generate_tag_blacklist();
    generate_attr_blacklist();
}

struct NodeGen<T> {
    byte: u8,
    term: Option<T>,
    children: Vec<NodeGen<T>>,
}

fn generate_tag_blacklist() {
    let mut root: Vec<NodeGen<bool>> = vec![];

    add(b"applet", true, &mut root, 0);
    add(b"base", true, &mut root, 0);
    add(b"comment", true, &mut root, 0); /* IE http://html5sec.org/#38 */
    add(b"embed", true, &mut root, 0);
    add(b"frame", true, &mut root, 0);
    add(b"frameset", true, &mut root, 0);
    add(b"handler", true, &mut root, 0); /* Opera SVG, effectively a script tag */
    add(b"iframe", true, &mut root, 0);
    add(b"import", true, &mut root, 0);
    add(b"isindex", true, &mut root, 0);
    add(b"link", true, &mut root, 0);
    add(b"listener", true, &mut root, 0);
    add(b"meta", true, &mut root, 0);
    add(b"noscript", true, &mut root, 0);
    add(b"object", true, &mut root, 0);
    add(b"script", true, &mut root, 0);
    add(b"style", true, &mut root, 0);
    add(b"vmlframe", true, &mut root, 0);
    add(b"xml", true, &mut root, 0);
    add(b"xss", true, &mut root, 0);
    //add(b"xsss", true, &mut root, 0); //to support overlap unit test

    write(&mut root, "bool", "BLACKLISTED_TAGS");
}

fn generate_attr_blacklist() {
    let mut root: Vec<NodeGen<Attribute>> = vec![];

    add(X_ACTION, Attribute::TypeAttrUrl, &mut root, 0); /* form */
        add(X_ATTRIBUTENAME, Attribute::TypeAttrIndirect, &mut root, 0); /* SVG allow indirection of Attribute names */
        add(X_BY, Attribute::TypeAttrUrl, &mut root, 0); /* SVG */
        add(X_BACKGROUND, Attribute::TypeAttrUrl, &mut root, 0); /* IE6 O11 */
        add(X_DATAFORMATAS, Attribute::TypeBlack, &mut root, 0); /* IE */
        add(X_DATASRC, Attribute::TypeBlack, &mut root, 0); /* IE */
        add(X_DYNSRC, Attribute::TypeAttrUrl, &mut root, 0); /* Obsolete img Attribute */
        add(X_FILTER, Attribute::TypeStyle, &mut root, 0); /* Opera SVG inline style */
        add(X_FORMACTION, Attribute::TypeAttrUrl, &mut root, 0); /* HTML 5 */
        add(X_FOLDER, Attribute::TypeAttrUrl, &mut root, 0); /* Only on A tags IE-only */
        add(X_FROM, Attribute::TypeAttrUrl, &mut root, 0); /* SVG */
        add(X_HANDLER, Attribute::TypeAttrUrl, &mut root, 0); /* SVG Tiny Opera */
        add(X_HREF, Attribute::TypeAttrUrl, &mut root, 0);
        add(X_LOWSRC, Attribute::TypeAttrUrl, &mut root, 0); /* Obsolete img Attribute */
        add(X_POSTER, Attribute::TypeAttrUrl, &mut root, 0); /* Opera 10 11 */
        add(X_SRC, Attribute::TypeAttrUrl, &mut root, 0);
        add(X_STYLE, Attribute::TypeStyle, &mut root, 0);
        add(X_TO, Attribute::TypeAttrUrl, &mut root, 0); /* SVG */
        add(X_VALUES, Attribute::TypeAttrUrl, &mut root, 0); /* SVG */
        add(X_XLINKHREF, Attribute::TypeAttrUrl, &mut root, 0);
        add(b"xmlns", Attribute::TypeBlack, &mut root, 0);
        add(b"xlink", Attribute::TypeBlack, &mut root, 0);

    write(&mut root, "Attribute", "BLACKLISTED_ATTRIBUTES");
}

fn write<T: std::fmt::Debug>(root: & mut Vec<NodeGen<T>>, typ: &str, name: &str) {
    println!("pub static {}: [Node<{}>; {}] = [", name, typ, root.len());
    writex(root,  0);
    println!("];");
}

fn writex<T: std::fmt::Debug>(root: & mut Vec<NodeGen<T>>, depth: usize) {
    root.sort_by(|x, y| x.byte.cmp(&y.byte));
    let indent = ::std::iter::repeat(" ").take(4 * depth).collect::<String>();

    for node in root.iter_mut() {
        println!("{}Node {{", indent);
        println!("{}    byte: b'{}',", indent, node.byte as char);
        println!("{}    term: {:?},", indent, node.term);
        println!("{}    children: &[", indent);
        writex(node.children.as_mut(), depth +  1);
        println!("{}    ]}},", indent);
    }

}

fn add<T: Clone >(bytes: &[u8], val: T, nodes: &mut Vec<NodeGen<T>>, depth: usize) {
    if bytes.len() == depth {  //termination condition :)
        return;
    }
    let byte = bytes[depth];
    let idx = nodes.iter().position(|n| n.byte == byte).unwrap_or_else(|| {
        nodes.push(NodeGen {
            byte: byte,
            term: if depth == bytes.len() - 1 {Some(val.clone())} else {None},
            children: Vec::new(),
        });
        return nodes.len() - 1;
    });
    let child = nodes.get_mut(idx).unwrap();
    child.term =  if depth == bytes.len() - 1 {Some(val.clone())} else {None};
    add(bytes, val.clone(), &mut child.children, depth + 1)
}


fn attrs() {
    let path = Path::new("/tmp").join("blackattrs_gen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "static BLACK_ATTRIBUTES: phf::Map<&[u8], Attribute> = ").unwrap();
    phf_codegen::Map::new()
        .entry(X_ACTION, "Attribute::TypeAttrUrl") /* form */
        .entry(X_ATTRIBUTENAME, "Attribute::TypeAttrIndirect") /* SVG allow indirection of Attribute names */
        .entry(X_BY, "Attribute::TypeAttrUrl") /* SVG */
        .entry(X_BACKGROUND, "Attribute::TypeAttrUrl") /* IE6 O11 */
        .entry(X_DATAFORMATAS, "Attribute::TypeBlack") /* IE */
        .entry(X_DATASRC, "Attribute::TypeBlack") /* IE */
        .entry(X_DYNSRC, "Attribute::TypeAttrUrl") /* Obsolete img Attribute */
        .entry(X_FILTER, "Attribute::TypeStyle") /* Opera") SVG inline style */
        .entry(X_FORMACTION, "Attribute::TypeAttrUrl") /* HTML 5 */
        .entry(X_FOLDER, "Attribute::TypeAttrUrl") /* Only on A tags") IE-only */
        .entry(X_FROM, "Attribute::TypeAttrUrl") /* SVG */
        .entry(X_HANDLER, "Attribute::TypeAttrUrl") /* SVG Tiny") Opera */
        .entry(X_HREF, "Attribute::TypeAttrUrl")
        .entry(X_LOWSRC, "Attribute::TypeAttrUrl") /* Obsolete img Attribute */
        .entry(X_POSTER, "Attribute::TypeAttrUrl") /* Opera 10")11 */
        .entry(X_SRC, "Attribute::TypeAttrUrl")
        .entry(X_STYLE, "Attribute::TypeStyle")
        .entry(X_TO, "Attribute::TypeAttrUrl") /* SVG */
        .entry(X_VALUES, "Attribute::TypeAttrUrl") /* SVG */
        .entry(X_XLINKHREF, "Attribute::TypeAttrUrl")
        .build(&mut file)
        .unwrap();
    write!(&mut file, ";\n").unwrap();
}


fn tags() {
    let path = Path::new("/tmp").join("blacktags_gen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let foo: &[u8] = b"applet";

    write!(&mut file, "static BLACK_TAGS: phf::Map<&[u8], bool> = ").unwrap();
    phf_codegen::Map::new()
        .entry(foo, "true")
        .entry(b"base", "true")
        .entry(b"comment", "true") /* IE http://html5sec.org/#38 */
        .entry(b"embed", "true")
        .entry(b"frame", "true")
        .entry(b"frameset", "true")
        .entry(b"handler", "true") /* Opera SVG, effectively a script tag */
        .entry(b"iframe", "true")
        .entry(b"import", "true")
        .entry(b"isindex", "true")
        .entry(b"link", "true")
        .entry(b"listener", "true")
        .entry(b"meta", "true")
        .entry(b"noscript", "true")
        .entry(b"object", "true")
        .entry(b"script", "true")
        .entry(b"style", "true")
        .entry(b"vmlframe", "true")
        .entry(b"xml", "true")
        .entry(b"xss", "true")
        .entry(b"zzzzznotlisted", "true")
        .build(&mut file)
        .unwrap();
    write!(&mut file, ";\n").unwrap();
}

