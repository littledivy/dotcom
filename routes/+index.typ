#import "@preview/html-shim:0.1.0": *

#show: html-shim

#webimg(
  "/static/img/cube.jpg",
  "rotterdam cube houses",
  extraImgClass: "max-h-[200px] w-full object-cover h-[200px]",
)
// #html.elem(
//  "div",
//  attrs: (
//    class: "bg-gradient-to-tr from-love to-foam w-full rounded-md h-40",
//  ),
//  "",
// )

Hiya, I'm Divy. SWE at Deno. I work on the open-source #link("https://deno.com")[Deno] runtime. Compilers, runtimes, cryptography and perf are my jam.

You can reach me at #link("mailto:me@littledivy.com")[me\@littledivy.com].

#show heading.where(level: 1): it => {
  html.elem("h2", attrs: (class: "!text-foreground"), it.body)
}

#html.elem("div", attrs: (id: "contact"), [])

= Contact

#let icon(name: "") = {
  html.elem("span", attrs: (class: "my-auto w-[24px]"), lucide-icon(name: name))
}

#html.elem(
  "div",
  attrs: (class: "font-sans w-full prose-lg"),
  {
    let entry(
      href: "",
      is-link: true,
      newtab: true,
      internal: false,
      body,
    ) = {
      html.elem(
        if is-link { "a" } else { "span" },
        attrs: (
          href: href,
          target: if newtab { "_blank" } else { "" },
          class: "p-1 font-light hover:text-bg hover:bg-love border-b-1 border-b-love text-love decoration-none min-w-full inline-flex justify-between content-center min-h-[50px]",
        ),
        {
          html.elem("span", attrs: (class: "flex gap-2 my-auto"), body)
          if internal {
            icon(name: "move-right")
          } else if is-link {
            icon(name: "external-link")
          }
        },
      )
    }

    entry(href: "https://github.com/littledivy", {
      icon(name: "github")
      [GitHub]
    })

    entry(href: "https://www.linkedin.com/in/divy-srivastava-032939150/", {
      icon(name: "linkedin")
      [LinkedIn]
    })
  },
)

