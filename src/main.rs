use dioxus::prelude::AssetVariant::Folder;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronRight, FileText, FolderOpen, FolderPlus, NotebookPen};
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Fonts {  }

        div { 
            class: "app",
            Explorer {  }
            Editor {  }
        }
        

    }
}

#[component]
fn Fonts() -> Element {
    rsx! {
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }

        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }

        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap",
        }
    }
}

#[component]
fn Editor() -> Element {

    rsx! {
        div {
            class: "editor",
            div { 
                contenteditable: true,
                spellcheck: true,
                class: "editor-field editor-field-title",
                "Enter title..."
            }
            div {
                contenteditable: true,
                spellcheck: true,
                class: "editor-field editor-field-txt",
                "Start typing..."
            }
        }
        
    }
}

#[component]
fn Explorer() -> Element {
    let mut selected_folder = use_signal(|| None::<PathBuf>);
    let mut files = use_signal(Vec::<PathBuf>::new);
    rsx! {
        div { 
            class: "explorer",
            header { 
                button { 
                    class: "explorer-button",
                    onclick: move |_| {
                        if let Some(folder) = FileDialog::new().pick_folder() {
                            println!("Selected folder: {:#?}", folder);

                            selected_folder.set(Some(folder.clone()));

                            let folder_files = fs::read_dir(&folder)
                                .ok()
                                .into_iter()
                                .flatten()
                                .filter_map(|entry| entry.ok())
                                .map(|entry| entry.path())
                                .collect::<Vec<_>>();
                            
                            println!("Folder files: {:#?}", folder_files);

                            files.set(folder_files);
                        }
                    },
                    FolderOpen {
                        size: 20
                    }
                },
                div { class: "explorer-divider" }
                button { 
                    class: "explorer-button",
                    NotebookPen {
                        size: 20
                    }
                },
                button { 
                    class: "explorer-button",
                    FolderPlus {
                        size: 20
                    }
                },
            

            },

            div { 
                class: "explorer-files",

                for path in files.read().iter() {
                    button {
                        class: "explorer-file",

                        div { 
                            class: "explorer-file-icon",

                            if path.is_dir() {
                                ChevronRight {
                                    size: 16
                                }
                            }
                        }  

                        "{path.file_name().unwrap_or_default().to_string_lossy()}"
                    }
                }
            }

        }
    }
}