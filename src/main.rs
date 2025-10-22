use std::{env, fs, path::PathBuf};

use clap::Parser;
use markdown::{
    mdast::{Node, Yaml},
    Constructs, ParseOptions,
};
use yaml_rust2::YamlLoader;

#[derive(Parser, Debug)]
#[clap(author = "TaeruAlethea", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short)]
    verbose: bool,

    #[arg(short)]
    input_file: Option<PathBuf>,

    #[arg(short)]
    output_dir: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }

    let input_file = args.input_file.expect("No input file provided!");
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let target = current_dir.join(&input_file);
    println!("target: {:?}", target);
    let data = fs::read_to_string(target).expect(
        "Unable to read file. Upward directory traversal is not allowed, so verify its not that.",
    );

    let options = &ParseOptions {
        constructs: CONSTRUCTS_FM_CF_ONLY,
        gfm_strikethrough_single_tilde: false,
        math_text_single_dollar: false,
        ..ParseOptions::default()
    };

    let mdastdata = markdown::to_mdast(data.as_str(), options).unwrap();

    let root = match mdastdata {
        Node::Root(root) => root,
        other => unimplemented!("{other:?}"),
    };

    println!("success! Root is {root:#?}");

    let yaml = match root.children.get(0) {
        Some(Node::Yaml(yaml)) => yaml,
        other => unimplemented!("No Yaml Node found: {other:#?}"),
    };

    let yaml_value = yaml.clone().value;
    println!("Yaml Value: {yaml_value:?}");

    let doc = YamlLoader::load_from_str(yaml_value.as_str()).unwrap();
    println!("doc: {doc:#?}")
}

const CONSTRUCTS_FM_CF_ONLY: Constructs = Constructs {
    frontmatter: true,
    code_fenced: true,
    code_indented: true,
    code_text: true,

    character_reference: true,

    attention: false,
    autolink: false,
    block_quote: false,
    character_escape: false,
    definition: false,
    gfm_autolink_literal: false,
    gfm_footnote_definition: false,
    gfm_label_start_footnote: false,
    gfm_strikethrough: false,
    gfm_table: false,
    gfm_task_list_item: false,
    hard_break_escape: false,
    hard_break_trailing: false,
    heading_atx: false,
    heading_setext: false,
    html_flow: false,
    html_text: false,
    label_start_image: false,
    label_start_link: false,
    label_end: false,
    list_item: false,
    math_flow: false,
    math_text: false,
    mdx_esm: false,
    mdx_expression_flow: false,
    mdx_expression_text: false,
    mdx_jsx_flow: false,
    mdx_jsx_text: false,
    thematic_break: false,
};
