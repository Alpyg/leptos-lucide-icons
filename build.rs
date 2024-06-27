use std::{env, fs::{self, File}, io::{self, Write}, path::Path};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use walkdir::WalkDir;
use heck::ToPascalCase;

fn main() -> io::Result<()> {

    let icon_dir = "lucide/icons";
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("icons.rs");
    let mut file = File::create(&dest_path)?;

    writeln!(file, "use leptos::*;")?;
    
    for entry in WalkDir::new(icon_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("svg") {
                let svg = fs::read_to_string(path)?;
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let component_name = format_ident!("{}", file_stem.to_pascal_case());

                let svg_tokens: TokenStream = svg.parse().unwrap();

                let component_code = quote! {
                    pub fn #component_name() -> impl IntoView {
                        view! {
                            #svg_tokens
                        }
                    }
                };

                writeln!(file, "{}", component_code)?;
            }
        }
    }

    Ok(())
}
