use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr,
};

struct MacroInput {
    problem_dir: LitStr,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MacroInput {
            problem_dir: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn define_problems(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MacroInput);
    let problem_dir = input.problem_dir.value();
    let mut output = String::new();
    output.push_str("use super::Solutions;\n\n");
    let problem_files = std::fs::read_dir(problem_dir)
        .unwrap()
        .map(|file| {
            file.unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .trim_end_matches(".rs")
                .to_string()
        })
        .collect::<Vec<_>>();
    for problem_file in problem_files.iter() {
        output.push_str(&format!("mod {};\n", problem_file));
    }
    output.push_str("\n");
    output.push_str("pub fn all_problems() -> Vec<Solutions> {\n");
    output.push_str("    vec![\n");
    for problem_file in problem_files.iter() {
        output.push_str(&format!("        {}::SOLUTIONS,\n", problem_file));
    }
    output.push_str("    ]\n");
    output.push_str("}\n");
    output.parse().unwrap()
}
