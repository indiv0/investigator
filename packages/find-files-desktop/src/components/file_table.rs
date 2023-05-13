use dioxus::prelude::*;

use crate::model;



// ======================
// === FileEntryProps ===
// ======================

#[derive(Props)]
struct FileEntryProps<'a> {
    id: u32,
    files: &'a UseRef<model::FileModel>,
}



// =================
// === FileEntry ===
// =================

fn file_entry<'a>(cx: Scope<'a, FileEntryProps<'a>>) -> Element {
    let _files = cx.props.files.read();
    let file = _files.files.get(&cx.props.id)?;

    cx.render(rsx! {
        li {
            div {
                label {
                    r#for: "cbg-{file.id}",
                    pointer_events: "none",
                    "{file.path}"
                }
            }
        }
    })
}



// ======================
// === FileTableProps ===
// ======================

#[derive(Props)]
pub struct FileTableProps<'a> {
    files: &'a UseRef<model::FileModel>,
}



// =================
// === FileTable ===
// =================

pub fn file_table<'a>(cx: Scope<'a, FileTableProps<'a>>) -> Element {
    //let files = cx.props.files.read();
    let files = cx.props.files;

    //let file_ids = files.read().files();
    //let file_ids = files.read();
    //let file_ids = file_ids.files();

    let file_list = files.read().files().into_iter().map(|id| {
        rsx!(file_entry {
            id: id,
            files: files
        })
    });

    cx.render(rsx! {
        div {
            class: "bg-gray-900",
            div {
                class: "mx-auto max-w-7xl",
                div {
                    class: "bg-gray-900 py-10",
                    div {
                        class: "px-4 sm:px-6 lg:px-8",
                        div {
                            class: "sm:flex sm:items-center",
                            div {
                                class: "sm:flex-auto",
                                h1 {
                                    class: "text-base font-semibold text-white leading-6 text-white",
                                    "Users"
                                }
                                p {
                                    class: "mt-2 text-sm text-gray-300",
                                    "A list of all the users in your account including their name, title, email, and role."
                                }
                            }
                            div {
                                class: "mt-4 sm:ml-16 sm:mt-0 sm:flex-none",
                                button {
                                    r#type: "button",
                                    class: "block rounded-md bg-indigo-500 px-3 py-2 text-center text-sm font-semibold text-white hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500",
                                    "Add user"
                                }
                            }
                        }
                        div {
                            class: "mt-8 flow-root",
                            div {
                                class: "-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8",
                                div {
                                    class: "inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8",
                                    table {
                                        class: "min-w-full divide-y divide-gray-700",
                                        thead {
                                            tr {
                                                th {
                                                    scope: "col",
                                                    class: "py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-white sm:pl-0",
                                                    "Name"
                                                }
                                                th {
                                                    scope: "col",
                                                    class: "px-3 py-3.5 text-left text-sm font-semibold text-white",
                                                    "Title"
                                                }
                                                th {
                                                    scope: "col",
                                                    class: "px-3 py-3.5 text-left text-sm font-semibold text-white",
                                                    "Email"
                                                }
                                                th {
                                                    scope: "col",
                                                    class: "px-3 py-3.5 text-left text-sm font-semibold text-white",
                                                    "Role"
                                                }
                                                th {
                                                    scope: "col",
                                                    class: "relative py-3.5 pl-3 pr-4 sm:pr-0",
                                                    span {
                                                        class: "sr-only",
                                                        "Edit"
                                                    }
                                                }
                                            }
                                        }
                                        tbody {
                                            class: "divide-y divide-gray-800",
                                            tr {
                                                td {
                                                    class: "whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-white sm:pl-0",
                                                    "Lindsay Walton"
                                                }
                                                td {
                                                    class: "whitespace-nowrap px-3 py-4 text-sm text-gray-300",
                                                    "Front-end Developer"
                                                }
                                                td {
                                                    class: "whitespace-nowrap px-3 py-4 text-sm text-gray-300",
                                                    "lindsay.walton@example.com"
                                                }
                                                td {
                                                    class: "whitespace-nowrap px-3 py-4 text-sm text-gray-300",
                                                    "Member"
                                                }
                                                td {
                                                    class: "relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-0",
                                                    a {
                                                        href: "#",
                                                        class: "text-indigo-400 hover:text-indigo-300",
                                                        "Edit"
                                                        span {
                                                            class: "sr-only",
                                                            ", Lindsay Walton"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
