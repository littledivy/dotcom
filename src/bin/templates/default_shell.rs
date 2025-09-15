/// The default page shell that provides navigation bars, metadata, etc.
use hypertext::prelude::*;

use super::components::Head;

/// Wide margins for prose or thin margins for a wide content area.
pub enum PageWidth {
  Wide,
  Prose,
}

/// The default "shell" around the page. Renders navigation, footer, and miscellany into place.
pub struct DefaultShell {
  pub head: Head,
  pub width: PageWidth,
}

impl DefaultShell {
  pub fn render_with_children(
    self,
    children: impl Renderable,
  ) -> Rendered<String> {
    let git_version =
      std::env::var("EPILOGUE_GIT_COMMIT").unwrap_or("unstable".to_string());
    let current_time = {
      let time_str = std::env::var("EPILOGUE_LAST_MODIFIED")
        .unwrap_or("sometime".to_string())
        .parse::<i64>()
        .unwrap_or(0);
      let format_description = time::macros::format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second] utc[offset_hour sign:mandatory]"
      );
      time::UtcDateTime::from_unix_timestamp(time_str)
        .expect("couldn't parse datetime from unix timestamp")
        .to_offset(time::macros::offset!(-7))
        .format(&format_description)
        .unwrap()
    };
    let rustc_verison = compile_time::rustc_version_str!();

    let nav_items = maud! {};

    let writeups = maud! {
        li {a class="hover:bg-surface transition-colors" href="/resym" {"Remote stack trace symolication"}}
        li {a class="hover:bg-surface transition-colors" href="/sh-deno" {"Security hardened Deno for macOS"}}
        li {a class="hover:bg-surface transition-colors" href="/turbocall" {"Turbocall JIT"}}
        li {a class="hover:bg-surface transition-colors" href="/sui" {"Inject RO data into executables"}}
    };

    let talks = maud! {
        ul class="space-y-4"
        {
            li {
                a href="https://youtu.be/qt3-3FkPqQ8?t=450"
                  class="block hover:underline transition-all duration-300 ease-out font-medium"
                {
                    "Kernel to runtime: inside a JavaScript runtime"
                }
                span class="block text-sm text-gray-400 mt-1" {
                    "IIT Kanpur (Sept 2025)"
                }
            }

            li {
                a href="https://www.youtube.com/watch?v=vINOqgn_ik8"
                  class="block hover:underline transition-all duration-300 ease-out font-medium"
                {
                    "Deno under the hood: op2"
                }
                span class="block text-sm text-gray-400 mt-1" {
                    "Dublin (Dec 2024)"
                }
            }

            li {
                a href="https://www.youtube.com/watch?v=RKjVcl62J9w"
                  class="block hover:underline transition-all duration-300 ease-out font-medium"
                {
                    "Building games with Deno"
                }
                span class="block text-sm text-gray-400 mt-1" {
                    "Warsaw (Aug 2024)"
                }
            }

            li {
                a href="https://www.youtube.com/watch?v=gA152Hun8cI"
                  class="block hover:underline transition-all duration-300 ease-out font-medium"
                {
                    "WebGPU windowing"
                }
                span class="block text-sm text-gray-400 mt-1" {
                    "Warsaw (Aug 2024)"
                }
            }

            li {
                a href="https://www.youtube.com/watch?v=5wlZDw942J8"
                  class="block hover:underline transition-all duration-300 ease-out font-medium"
                {
                    "How deno compile works"
                }
                span class="block text-sm text-gray-400 mt-1" {
                    "India (Sep 2024)"
                }
            }

            li {
                a href="https://www.youtube.com/watch?v=ssYN4rFWRIU"
                  class="block hover:underline transition-all duration-300 ease-out font-medium"
                {
                    "Blazing fast FFI"
                }
                span class="block text-sm text-gray-400 mt-1" {
                    "Tokyo (Nov 2023)"
                }
            }
        }
    };

    let page_width = match self.width {
      PageWidth::Wide => "",
      PageWidth::Prose => " lg:max-w-[40rem]",
    };

    maud! {
            !DOCTYPE
            html lang="en" {
                (self.head)
                body class="antialiased mt-4 lg:mt-20 leading-relaxed mx-auto max-w-[1200px]" {
                    div class="flex gap-8 px-4 lg:px-6" {
                        aside class="hidden md:block w-64 flex-none" {
                            a href="/" class="inline-flex justify-between gap-4 hover:bg-subtle/50 transition-colors mt-3" {
                                span class="text-[2.5em] select-none -translate-y-[6px]" {"divy"}
                            }
                            nav class="space-y-4 mt-4" {
                                ul class="space-y-2 text-love text-2xl " {
                                    (nav_items)
                                }
                                div class="space-y-1" {
                                    p class="all-smallcaps text-lg" {"Writeups"}
                                    ul class="space-y-2 text-subtle text-base" {
                                        (writeups)
                                    }
                                }
                                div class="space-y-1" {
                                    p class="all-smallcaps text-lg" {"Talks"}
                                    ul class="space-y-2 text-subtle text-base" {
                                        (talks)
                                    }
                                }
                            }
                        }
                        div class="flex-1 md:mt-2" {
                            header class="md:hidden border-b border-dashed border-muted mb-8 pb-8 w-full" {
                                div class="w-full flex justify-center" {
                                    a href="/" class="inline-flex justify-between gap-4 hover:bg-subtle/50 transition-colors mt-8 mx-auto" {
                                        span class="italic text-[3em] text-center select-none -translate-y-2 mx-auto" {"divy"}
                                    }
                                }
                                details class="w-full mt-4" {
                                    summary class="text-center smallcaps text-xl cursor-pointer" {
                                        "menu"
                                    }
                                    nav class="space-y-4 text-2xl mt-3" {
                                        ul class="space-y-3 text-2xl text-love" {
                                            (nav_items)
                                        }
                                        div class="space-y-2" {
                                            span class="all-smallcaps text-lg" {"Writeups"}
                                            ul class="space-y-2 text-subtle text-lg" {
                                                (writeups)
                                            }
                                        }
                                        div class="space-y-2" {
                                            span class="all-smallcaps text-lg" {"Talks"}
                                            ul class="space-y-2 text-subtle text-lg" {
                                                (talks)
                                            }
                                        }
                                    }
                                }
                            }
                            main class=("main-content".to_owned() + page_width) {
                                (children)
                                footer class="mt-8 mb-4 text-sm text-muted py-1" {
                                    p class="all-smallcaps leading-[1.3]" {" "
                                        a
                                            class="text-link"
                                            href=("https://github.com/littledivy/web/commit/".to_owned() + &git_version) 
                                        {(git_version[..8].to_string())}
                                        " build using rustc " {(rustc_verison)} " at " {(current_time)} ". "
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        .render()
  }
}
