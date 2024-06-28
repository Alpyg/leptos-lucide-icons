use std::{env, fs::{self, File}, io::{self, Write}, path::Path};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use walkdir::WalkDir;
use heck::ToPascalCase;
use regex::Regex;

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
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let component_name = format_ident!("{}", file_stem.to_pascal_case());

                let re = Regex::new(r"(?s)<svg[^>]*>(.*?)</svg>").unwrap();
                let svg = fs::read_to_string(path).unwrap();
                let svg_children: TokenStream = re.captures(&svg)
                    .and_then(|captures| captures.get(1))
                    .map(|m| m.as_str())
                    .unwrap().parse().unwrap();

                let component_code = quote! {
                    #[component]
                    pub fn #component_name(
                        #[prop(into, optional)] class: MaybeSignal<String>
                    ) -> impl IntoView {
                        view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class=class
                            >
                                #svg_children
                            </svg>
                        }
                    }
                };

                writeln!(file, "{}", component_code)?;
            }
        }
    }

    Ok(())
}
