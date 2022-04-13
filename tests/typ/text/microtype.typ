// Test micro-typographical shenanigans.

---
// Test hanging punctuation.
#set page(width: 130pt, margins: 15pt)
#set par(justify: true, linebreaks: "simple")
#set text(lang: "en", size: 9pt)
#rect(fill: rgb(repr(teal) + "00"), width: 100%)[
  This is a little bit of text that builds up to
  hang-ing hyphens and dash---es and then, you know,
  some punctuation in the margin.
]

// Test hanging punctuation with RTL.
#set text(lang: "he")
בנייה נכונה של משפטים ארוכים דורשת ידע בשפה. אז בואו נדבר על מזג האוויר.

---
// Test that lone punctuation doesn't overhang into the margin.
#set page(margins: 0pt)
#set par(align: end)
#set text(dir: rtl)
:
