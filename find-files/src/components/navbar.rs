use dioxus::prelude::*;



// ==============
// === Navbar ===
// ==============

pub fn navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "bg-white shadow",
            div {
                class: "mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
                div {
                    class: "relative flex h-16 justify-between",
                    div {
                        class: "absolute inset-y-0 left-0 flex items-center sm:hidden",
                        // Mobile menu button.
                        button {
                            r#type: "button",
                            class: "inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500",
                            aria_controls: "mobile-menu",
                            aria_expanded: "false",
                            span {
                                class: "sr-only",
                                "Open main menu"
                            }
                            // Icon when menu is closed.
                            //
                            // Menu open: "hidden", Menu closed: "block".
                            svg {
                                class: "block h-6 w-6",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                // FIXME [NP]: why is this not working?
                                //aria_hidden: "true",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                                }
                            }
                            // Icon when menu is open.
                            //
                            // Menu open: "block", Menu closed: "hidden".
                            svg {
                                class: "hidden h-6 w-6",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                // FIXME [NP]: why is this not working?
                                //aria_hidden: "true",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M6 18L18 6M6 6l12 12",
                                }
                            }
                        }
                    }
                    div {
                        class: "flex flex-1 items-center justify-center sm:items-stretch sm:justify-start",
                        div {
                            class: "flex flex-shrink-0 items-center",
                            img {
                                class: "block h-8 w-auto lg:hidden",
                                // FIXME [NP]: use a local image.
                                src: "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600",
                                alt: "Your Company",
                            }
                            img {
                                class: "hidden h-8 w-auto lg:block",
                                // FIXME [NP]: use a local image.
                                src: "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600",
                                alt: "Your Company",
                            }
                        }
                        div {
                            class: "hidden sm:ml-6 sm:flex sm:space-x-8",
                            // Current: "border-indigo-500 text-gray-900", Default: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700".
                            a {
                                href: "#",
                                class: "inline-flex items-center border-b-2 border-indigo-500 px-1 pt-1 text-sm font-medium text-gray-900",
                                "Dashboard"
                            }
                            a {
                                href: "#",
                                class: "inline-flex items-center border-b-2 border-transparent px-1 pt-1 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700",
                                "Team"
                            }
                            a {
                                href: "#",
                                class: "inline-flex items-center border-b-2 border-transparent px-1 pt-1 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700",
                                "Projects"
                            }
                            a {
                                href: "#",
                                class: "inline-flex items-center border-b-2 border-transparent px-1 pt-1 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700",
                                "Calendar"
                            }
                        }
                    }
                    div {
                        class: "absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0",
                        button {
                            r#type: "button",
                            class: "rounded-full bg-white p-1 text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                            span {
                                class: "sr-only",
                                "View notifications"
                            }
                            svg {
                                class: "h-6 w-6",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                // FIXME [NP]: why is this not working?
                                //aria_hidden: "true",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0",
                                }
                            }
                        }
                        // Profile dropdown.
                        div {
                            class: "relative ml-3",
                            div {
                                button {
                                    r#type: "button",
                                    class: "flex rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                                    id: "user-menu-button",
                                    aria_expanded: "false",
                                    aria_haspopup: "true",
                                    span {
                                        class: "sr-only",
                                        "Open user menu"
                                    }
                                    img {
                                        class: "h-8 w-8 rounded-full",
                                        // FIXME [NP]: use a local image.
                                        src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80",
                                        alt: "",
                                    }
                                }
                            }
                            // Dropdown menu, show/hide based on menu state.
                            //
                            // Entering: "transition ease-out duration-200"
                            //   From: "transform opacity-0 scale-95"
                            //   To: "transform opacity-100 scale-100"
                            // Leaving: "transition ease-in duration-75"
                            //   From: "transform opacity-100 scale-100"
                            //   To: "transform opacity-0 scale-95"
                            div {
                                class: "absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none",
                                role: "menu",
                                aria_orientation: "vertical",
                                aria_labelledby: "user-menu-button",
                                tabindex: "-1",
                                // Active: "bg-gray-100", Not Active: "".
                                a {
                                    href: "#",
                                    class: "block px-4 py-2 text-sm text-gray-700",
                                    role: "menuitem",
                                    tabindex: "-1",
                                    id: "user-menu-item-0",
                                    "Your Profile"
                                }
                                a {
                                    href: "#",
                                    class: "block px-4 py-2 text-sm text-gray-700",
                                    role: "menuitem",
                                    tabindex: "-1",
                                    id: "user-menu-item-1",
                                    "Settings"
                                }
                                a {
                                    href: "#",
                                    class: "block px-4 py-2 text-sm text-gray-700",
                                    role: "menuitem",
                                    tabindex: "-1",
                                    id: "user-menu-item-2",
                                    "Sign out"
                                }
                            }
                        }
                    }
                }
            }
            // Mobile menu, show/hide based on menu state.
            div {
                class: "sm:hidden",
                id: "mobile-menu",
                div {
                    class: "space-y-1 pb-4 pt-2",
                    // Current: "bg-indigo-50 border-indigo-500 text-indigo-700", Default: "border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700".
                    a {
                        href: "#",
                        class: "block border-l-4 border-indigo-500 bg-indigo-50 py-2 pl-3 pr-4 text-base font-medium text-indigo-700",
                        "Dashboard"
                    }
                    a {
                        href: "#",
                        class: "block border-l-4 border-transparent py-2 pl-3 pr-4 text-base font-medium text-gray-500 hover:border-gray-300 hover:bg-gray-50 hover:text-gray-700",
                        "Team"
                    }
                    a {
                        href: "#",
                        class: "block border-l-4 border-transparent py-2 pl-3 pr-4 text-base font-medium text-gray-500 hover:border-gray-300 hover:bg-gray-50 hover:text-gray-700",
                        "Projects"
                    }
                    a {
                        href: "#",
                        class: "block border-l-4 border-transparent py-2 pl-3 pr-4 text-base font-medium text-gray-500 hover:border-gray-300 hover:bg-gray-50 hover:text-gray-700",
                        "Calendar"
                    }
                }
            }
        }
    })
}
