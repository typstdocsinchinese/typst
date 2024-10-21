---
en_title: Page setup guide
description: |
  An in-depth guide to setting page dimensions, margins, and page numbers in
  Typst. Learn how to create appealing and clear layouts and get there quickly.
---

# 页面配置指南

页面配置很大程度上决定了你的文档给人的第一印象。行长、边距以及栏目的安排都会影响到文章的[观感](https://practicaltypography.com/page-margins.html)和[可读性](https://designregression.com/article/line-length-revisited-following-the-research)，同时正确的页眉和页脚信息能够让读者更轻松地翻阅你的文档。这篇指南会指导你自定义页面本身及其边距、页眉页脚和页码，让它们与你的文档相称，以便你开展接下来的编写工作。

<original>
Your page setup is a big part of the first impression your document gives. Line
lengths, margins, and columns influence
[appearance](https://practicaltypography.com/page-margins.html) and
[legibility](https://designregression.com/article/line-length-revisited-following-the-research)
while the right headers and footers will help your reader easily navigate your
document. This guide will help you to customize pages, margins, headers,
footers, and page numbers so that they are the right fit for your content and
you can get started with writing.
</original>

在 Typst 中，每一页面都有长度、宽度以及四个方向上的边距。上下边距的空白区域内可包含页眉和页脚。这一系列配置工作都在 [`{page}`]($page) 元素的 set rule 里完成。当你对该 set rule 进行修改时，Typst 会始终确保一个遵循该配置的空页面存在于最后，因此该 set rule 可能导致页面的切换。因此，最好在你文档的开头处插入 [`{page}`]($page) set rule。

<original>
In Typst, each page has a width, a height, and margins on all four sides. The
top and bottom margins may contain a header and footer. The set rule of the
[`{page}`]($page) element is where you control all of the page setup. If you
make changes with this set rule, Typst will ensure that there is a new and
conforming empty page afterward, so it may insert a page break. Therefore, it is
best to specify your [`{page}`]($page) set rule at the start of your document or
in your template.
</original>

```example
#set rect(
  width: 100%,
  height: 100%,
  inset: 4pt,
)
>>> #set text(6pt)
>>> #set page(margin: auto)

#set page(
  paper: "iso-b7",
  header: rect(fill: aqua)[Header],
  footer: rect(fill: aqua)[Footer],
  number-align: center,
)

#rect(fill: aqua.lighten(40%))
```

该例子让页面的正文、页眉和页脚的部分分别显形。正文部分是页面整体大小（在这里是 ISO B7 规定的）减去每个方向上的默认边距得来的。上下边距内有用带边框的矩形标记出的页眉和页脚部分，它们不会影响到正文，而是根据各自的边距进行 30% 的偏移。
你可以用 [`header-ascent`]($page.header-ascent) 和 [`footer-descent`]($page.footer-descent) 参数来修改这一偏移量。

<original>
This example visualizes the dimensions for page content, headers, and footers.
The page content is the page size (ISO B7) minus each side's default margin. In
the top and the bottom margin, there are stroked rectangles visualizing the
header and footer. They do not touch the main content, instead, they are offset
by 30% of the respective margin. You can control this offset by specifying the
[`header-ascent`]($page.header-ascent) and
[`footer-descent`]($page.footer-descent) arguments.
</original>

接下来，本指南会用例子来更加深入地陈述如何满足一些常见的页面配置需求。

<original>
Below, the guide will go more into detail on how to accomplish common page setup
requirements with examples.
</original>

## 自定页面大小和边距

Typst 默认的页面大小和 A4 纸相同。考虑到地域以及实际情况的不同，你可能需要改变页面大小。这需要你编写一个 [`{page}`]($page) set rule，然后传入一个字符串参数来指定一个常见的页面大小标准。该参数支持整个 ISO 216 标准（比如 `"iso-a4"`、`"iso-c2"` 等）以及一些像 `"us-legal"`、`"us-letter"` 这样习惯上的美式格式等。参阅 [page 函数的 paper 参数]($page.paper)来了解所有可选值。

<original>
Typst's default page size is A4 paper. Depending on your region and your use
case, you will want to change this. You can do this by using the
[`{page}`]($page) set rule and passing it a string argument to use a common page
size. Options include the complete ISO 216 series (e.g. `"iso-a4"`, `"iso-c2"`),
customary US formats like `"us-legal"` or `"us-letter"`, and more. Check out the
reference for the [page's paper argument]($page.paper) to learn about all
available options.
</original>

```example
>>> #set page(margin: auto)
#set page("us-letter")

This page likes freedom.
```

如果你需要在长宽上修改页面大小，也可以指定 [`width`]($page.width) 和 [`height`]($page.height) 这两个具名参数。

<original>
If you need to customize your page size to some dimensions, you can specify the
named arguments [`width`]($page.width) and [`height`]($page.height) instead.
</original>

```example
>>> #set page(margin: auto)
#set page(width: 12cm, height: 12cm)

This page is a square.
```

### 更改页边距

边距的设置是一个好排版的重要组成部分：[排印专家认为长度大致在 45 到 75 个英文字符之间的段落最清晰易读](http://webtypography.net/2.1.2)，而边距和[分栏](#添加分栏)可用于调整一行的宽度。默认情况下，Typst 会按照比例，相对于文档页面大小来设置边距。如需自定边距的值，你需要在[`{page}`]($page) set rule 里用到 [`margin`]($page.margin) 参数。

<original>
Margins are a vital ingredient for good typography:
[Typographers consider lines that fit between 45 and 75 characters best length
for legibility](http://webtypography.net/2.1.2) and your margins and
[columns](#columns) help define line widths. By default, Typst will create
margins proportional to the page size of your document. To set custom margins,
you will use the [`margin`]($page.margin) argument in the [`{page}`]($page) set
rule.
</original>

如果你想将所有的边距都设定为同一数值的话，可向 `margin` 参数传入一个长度值。然而，一般情况下每一侧的边距是不一样的，这时需要传入一个字典：

<original>
The `margin` argument will accept a length if you want to set all margins to the
same width. However, you often want to set different margins on each side. To do
this, you can pass a dictionary:
</original>

```example
#set page(margin: (
  top: 3cm,
  bottom: 2cm,
  x: 1.5cm,
))

#lorem(100)
```

页边距字典中可以包括每一个方向的键（`top`、`bottom`、`left` 和 `right`），你还可以直接用 `x` 键来同时控制左右边距，就像例子中的那样。同理，上下边距可以用 `y` 键来统一调整。

<original>
The page margin dictionary can have keys for each side (`top`, `bottom`, `left`,
`right`), but you can also control left and right together by setting the `x`
key of the margin dictionary, like in the example. Likewise, the top and bottom
margins can be adjusted together by setting the `y` key.
</original>

如果你没有在字典规定每个方向上的边距，那么原先的边距将保留。如果不想这样而是需要让没有特地设置的边距都为另外一个值，可以用 `rest` 键来规定。例如，`[#set page(margin: (left: 1.5in, rest: 1in))]` 会将左边距设置为 1.5 英寸，其余的边距设定为一英寸。

<original>
If you do not specify margins for all sides in the margin dictionary, the old
margins will remain in effect for the unset sides. To prevent this and set all
remaining margins to a common size, you can use the `rest` key. For example,
`[#set page(margin: (left: 1.5in, rest: 1in))]` will set the left margin to 1.5
inches and the remaining margins to one inch.
</original>

### 为奇、偶页面设置不同的边距

有时你需要为奇偶页面设置不同的横向边距，例如你可能会想在靠近书脊的地方而不是外部留下更多的边距空白。Typst 在页面变量中保存了页面是在左侧还是在右侧的相关信息，所以你可以利用这些信息，并设置边距字典的 `inside` 和 `outside` 键来控制边距。`inside` 代表的是靠近书脊的部分边距，`outside` 则是靠近书的外边缘部分边距。

<original>
Sometimes, you'll need to alternate horizontal margins for even and odd pages,
for example, to have more room towards the spine of a book than on the outsides
of its pages. Typst keeps track of whether a page is to the left or right of the
binding. You can use this information and set the `inside` or `outside` keys of
the margin dictionary. The `inside` margin points towards the spine, and the
`outside` margin points towards the edge of the bound book.
</original>

```typ
#set page(margin: (inside: 2.5cm, outside: 2cm, y: 1.75cm))
```

Typst 会假设按照从左到右顺序书写的文档按左侧装订，而按照从右到左顺序书写的文档则按右侧装订。不过，有时你可能需要改变这一点，例如当你的第一页内容是由其它程序生成的，其装订顺序与 Typst 的恰好相反，这是就需要进行修改了。另外，在有些书中，例如英文漫画书，即使英文本身是从左向右书写的，有时这些书也特地按右侧装订。如需手动修改装订方式并定义 `inside` 和 `outside` 的具体指向，请在设置 [`{page}`]($page) set rule 中的 [`binding`]($page.binding) 参数。

<original>
Typst will assume that documents written in Left-to-Right scripts are bound on
the left while books written in Right-to-Left scripts are bound on the right.
However, you will need to change this in some cases: If your first page is
output by a different app, the binding is reversed from Typst's perspective.
Also, some books, like English-language Mangas are customarily bound on the
right, despite English using Left-to-Right script. To change the binding side
and explicitly set where the `inside` and `outside` are, set the
[`binding`]($page.binding) argument in the [`{page}`]($page) set rule.
</original>

```typ
// Produce a book bound on the right,
// even though it is set in Spanish.
#set text(lang: "es")
#set page(binding: right)
```

如果 `binding` 的值是 `left`，`inside` 边距在奇数页上位于左侧，以此类推（译者注：原文此处是“反之亦然” vice versa）。

If `binding` is `left`, `inside` margins will be on the left on odd pages, and
vice versa.

## 添加页眉和页脚

页眉和页脚会插入到每个页面的顶部和底部。你可以自定义其内容，或者单纯地放上页码。

<original>
Headers and footers are inserted in the top and bottom margins of every page.
You can add custom headers and footers or just insert a page number.
</original>

考虑到你可能不仅仅需要放页码，插入页眉和页脚的最好办法其实是 [`{page}`]($page) set rule 的 [`header`]($page.header) 和 [`footer`]($page.footer) 参数，它们可以传入任意值。

<original>
In case you need more than just a page number, the best way to insert a header
and a footer are the [`header`]($page.header) and [`footer`]($page.footer)
arguments of the [`{page}`]($page) set rule. You can pass any content as their
values:
</original>

```example
>>> #set page("a5", margin: (x: 2.5cm, y: 3cm))
#set page(header: [
  _Lisa Strassner's Thesis_
  #h(1fr)
  National Academy of Sciences
])

#lorem(150)
```

页眉的内容默认情况下是置底的，这样它们就不会抵着页面的上缘了。你可以将页眉用 [`{align}`]($align) 函数来调整其定位方式。

<original>
Headers are bottom-aligned by default so that they do not collide with the top
edge of the page. You can change this by wrapping your header in the
[`{align}`]($align) function.
</original>

### 针对特定页面设置页眉页脚

在某些页面上你可能需要不一样的页眉页脚。比如说，在包含了标题的页面上你可能需要去掉它们。下面的例子展示了如何按照条件删掉第一页的页眉。

<original>
You'll need different headers and footers on some pages. For example, you may
not want a header and footer on the title page. The example below shows how to
conditionally remove the header on the first page:
</original>

```typ
>>> #set page("a5", margin: (x: 2.5cm, y: 3cm))
#set page(header: context {
  if counter(page).get().first() > 1 [
    _Lisa Strassner's Thesis_
    #h(1fr)
    National Academy of Sciences
  ]
})

#lorem(150)
```

这个例子里的写法看起来可能很吓人，但我们可以将其逐步拆解：我们利用 `{context}` 关键字来表示页眉与其在文档中的位置有关。然后我们判断了页面的[计数器]($counter)是否大于我们目前（根据上下文）的位置。页面的计数器是从 1 开始的，所以我们在第一页上跳过了页眉的插入。计数器可能有多层，这是像标题这样的元素的特性，但是计数器总会有第一层，所以我们只需要考虑它即可。

<original>
This example may look intimidating, but let's break it down: By using the
`{context}` keyword, we are telling Typst that the header depends on where we
are in the document. We then ask Typst if the page [counter] is larger than one
at our (context-dependent) current position. The page counter starts at one, so
we are skipping the header on a single page. Counters may have multiple levels.
This feature is used for items like headings, but the page counter will always
have a single level, so we can just look at the first one.
</original>

当然，你还可以在这个例子的代码中加上一个 `else` 来为第一页添加一个不同的页眉。

<original>
You can, of course, add an `else` to this example to add a different header to
the first page instead.
</original>

### 在具有特定元素的页面上修改页眉页脚

在上一节中提到的方法经过修改，利用 Typst 的标签功能，可以实现更多复杂的需求。例如，那些展示了大表格的页面上的页眉应当省略，避免凌乱。我们用一个 `<big-table>` [标签]($label)标记该表，然后用[查询系统]($query)判断当前页面上是否存在一个带有该标签的元素。

<original>
The technique described in the previous section can be adapted to perform more
advanced tasks using Typst's labels. For example, pages with big tables could
omit their headers to help keep clutter down. We will mark our tables with a
`<..>` [label] and use the [query system]($query) to find out if such a
label exists on the current page:
</original>

```typ
>>> #set page("a5", margin: (x: 2.5cm, y: 3cm))
#set page(header: context {
  let page-counter =
  let matches = query(<big-table>)
  let current = counter(page).get()
  let has-table = matches.any(m =>
    counter(page).at(m.location()) == current
  )

  if not has-table [
    _Lisa Strassner's Thesis_
    #h(1fr)
    National Academy of Sciences
  ]
}))

#lorem(100)
#pagebreak()

#table(
  columns: 2 * (1fr,),
  [A], [B],
  [C], [D],
) <big-table>
```

在这里我们查询了所有带 `<big-table>` 标签的实实例，然后判断了我们当前的位置是否存在表格，如果不存在，就输出页眉。这个例子里用到了变量来保持简洁。和上面一样，你也可以用 `else` 来添加另一个页眉而不是单纯地去掉它。

<original>
Here, we query for all instances of the `<..>` label. We then check that
none of the tables are on the page at our current position. If so, we print the
header. This example also uses variables to be more concise. Just as above, you
could add an `else` to add another header instead of deleting it.
</original>

## 页码的添加与自定义

页码可帮助读者更好地跟进和引用你的文档。添加页码最简单的方法是利用 [`{page}`]($page) set rule 的 [`numbering`]($page.numbering) 参数。你可以插入一个[_标号模板_]($numbering.numbering)字符串来表示你想要以何种形式标记页码。

<original>
Page numbers help readers keep track of and reference your document more easily.
The simplest way to insert page numbers is the [`numbering`]($page.numbering)
argument of the [`{page}`]($page) set rule. You can pass a
[_numbering pattern_]($numbering.numbering) string that shows how you want your
pages to be numbered.
</original>

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(numbering: "1")

This is a numbered page.
```

在上面你可以看到一个构思出的最简单例子。它在页脚的中间添加了一个单一的阿拉伯数字页码。你可以填入除了 `"1"` 以外的其他字符来得到其他的编号。例如，`"i"` 会产生小写的罗马数字。任何没有被识别为数字的字符会原样输出。例如，当你想要在页码两侧加上横杠，可以这样写：

<original>
Above, you can check out the simplest conceivable example. It adds a single
Arabic page number at the center of the footer. You can specify other characters
than `"1"` to get other numerals. For example, `"i"` will yield lowercase Roman
numerals. Any character that is not interpreted as a number will be output
as-is. For example, put dashes around your page number by typing this:
</original>

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(numbering: "— 1 —")

This is a — numbered — page.
```

在标号字符串中填入第二个数字，即可插入页面总数。

<original>
You can add the total number of pages by entering a second number character in
the string.
</original>

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(numbering: "1 of 1")

This is one of many numbered pages.
```

查看 [`{numbering}` 函数的参考信息]($numbering.numbering)来了解更多你可以传入的参数。

<original>
Go to the [`{numbering}` function reference]($numbering.numbering) to learn more
about the arguments you can pass here.
</original>

考虑到你可能需要将页码置于左侧或者右侧，可通过 [`{page}`]($page) set rule 的 [`number-align`]($page.number-align) 参数来实现。但目前不能用此参数在奇偶页面上切换定位的方向。要实现这个，你需要按照上面选择性地去掉页眉页脚的方法，插入自定义的页脚并调用页面计数器。

<original>
In case you need to right- or left-align the page number, use the
[`number-align`]($page.number-align) argument of the [`{page}`]($page) set rule.
Alternating alignment between even and odd pages is not currently supported
using this property. To do this, you'll need to specify a custom footer with
your footnote and query the page counter as described in the section on
conditionally omitting headers and footers.
</original>

### 利用页码自定义页脚

有时你可能还需要在页脚里添加页码以外的内容。但是，一旦页脚被指定，[`{page}`]($page) set rule 的 [`numbering`]($page.numbering) 参数就会被忽略掉。本节将展示如何添加一个不仅有页码还有其它内容的页脚。

<original>
Sometimes, you need to add other content than a page number to your footer.
However, once a footer is specified, the [`numbering`]($page.numbering) argument
of the [`{page}`]($page) set rule is ignored. This section shows you how to add
a custom footer with page numbers and more.
</original>

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(footer: context [
  *American Society of Proceedings*
  #h(1fr)
  #counter(page).display(
    "1/1",
    both: true,
  )
])

This page has a custom footer.
```

首先我们在左侧添加了一些着重强调的文字（译者注：此处指粗体），然后占满中间的空间，再调用 `counter(page)` 获取当前页面的计数器，借它的 `display` 方法输出它当前的值。我们还将 `both` 设置成了 `{true}`，这样标号格式就会应用到当前和最终的页码上。

<original>
First, we add some strongly emphasized text on the left and add free space to
fill the line. Then, we call `counter(page)` to retrieve the page counter and
use its `display` function to show its current value. We also set `both` to
`{true}` so that our numbering pattern applies to the current _and_ final page
number.
</original>

对于页码，我们还可以继续发挥想象力，比如说可以在每一页插入一个圆圈。

<original>
We can also get more creative with the page number. For example, let's insert a
circle for each page.
</original>

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(footer: context [
  *Fun Typography Club*
  #h(1fr)
  #let (num,) = counter(page).get()
  #let circles = num * (
    box(circle(
      radius: 2pt,
      fill: navy,
    )),
  )
  #box(
    inset: (bottom: 1pt),
    circles.join(h(1pt))
  )
])

This page has a custom footer.
```

在这个例子里，我们用当前页码创建了一串[圆圈]($circle)，它们被包围在了一个 [box] 里，这样它们就会排在同一行里，否则就会另起行。这个[数组]($array)的长度取决于当前的页码数值。

<original>
In this example, we use the number of pages to create an array of
[circles]($circle). The circles are wrapped in a [box] so they can all appear on
the same line because they are blocks and would otherwise create paragraph
breaks. The length of this [array] depends on the current page number.
</original>

然后，我们将这些圆圈放在了页脚的右侧，并在它们彼此之间加上了 1pt 的空隙。数组的 join 函数会尝试将数组中的不同值伴随着参数[合并]($scripting/#blocks)到一个值里去。在我们的例子中，最终合并的值是有空隙的一系列圆圈，这些空隙可以用 align 函数进行对齐。最后，我们用到了另一个 box 来确保文本和圆圈在同一行，还用到 [`inset` 参数]($box.inset)来略微提高圆圈的位置使得它们能与文本更好地对齐。

<original>
We then insert the circles at the right side of the footer, with 1pt of space
between them. The join method of an array will attempt to
[_join_]($scripting/#blocks) the different values of an array into a single
value, interspersed with its argument. In our case, we get a single content
value with circles and spaces between them that we can use with the align
function. Finally, we use another box to ensure that the text and the circles
can share a line and use the [`inset` argument]($box.inset) to raise the circles
a bit so they line up nicely with the text.
</original>

### 重置页码与跳过页面

在文档的某些位置，你是否需要重置页码的数值呢？或许你想要从带标题的页面之后才开始编号，又或者你需要跳过一些页码以便在最后的成品中自己手动加纸进去。

<original>
Do you, at some point in your document, need to reset the page number? Maybe you
want to start with the first page only after the title page. Or maybe you need
to skip a few page numbers because you will insert pages into the final printed
product.
</original>

修改页码的正确方式是去调控页面的[计数器]($counter)，最简单地，就是将计数器回调为 1。

<original>
The right way to modify the page number is to manipulate the page [counter]. The
simplest manipulation is to set the counter back to 1.
</original>

```typ
#counter(page).update(1)
```

这行代码会将页面的计数器重置为 1。它应该放在页面的开头，否则可能会导致分页。你还可以基于原值来修改计数器的值，此时需传入一个函数：

<original>
This line will reset the page counter back to one. It should be placed at the
start of a page because it will otherwise create a page break. You can also
update the counter given its previous value by passing a function:
</original>

```typ
#counter(page).update(n => n + 5)
```

在这个例子里，我们跳过了五页。`n` 是当前页面计数器的值，`n + 5` 是我们函数的返回值。

<original>
In this example, we skip five pages. `n` is the current value of the page
counter and `n + 5` is the return value of our function.
</original>


你还可能需要获取到真实页码而非当前页的计数器，这一点可以通过调用 [`here`] 函数返回值上的 [`page`]($location.page) 方法来实现。

<original>
In case you need to retrieve the actual page number instead of the value of the
page counter, you can use the [`page`]($location.page) method on the return
value of the [`here`] function:
</original>

```example
#counter(page).update(n => n + 5)

// This returns one even though the
// page counter was incremented by 5.
#context here().page()
```

你还可以用 `here` 返回值上的 [`page-numbering`]($location.page-numbering) 方法获取到当前页面的标号模板。

<original>
You can also obtain the page numbering pattern from the location returned by
`here` with the [`page-numbering`]($location.page-numbering) method.
</original>

## 添加分栏

在你的文档中添加分栏可以使其布局更加合理，同时也能保持行宽，维持可读性。分栏本质上是空白分隔的垂直文本块，这些空白部分严格称为 gutter。

<original>
Add columns to your document to fit more on a page while maintaining legible
line lengths. Columns are vertical blocks of text which are separated by some
whitespace. This space is called the gutter.
</original>

要在栏目中填入你的内容，只需在 [`{page}`]($page.columns) set rule 中指定对应的栏目序号。通过添加一个针对 [`columns` 函数]($columns)的 set rule 并指定 `gutter` 参数，你可以调整栏与栏之间的空隙大小。

<original>
To lay out your content in columns, just specify the desired number of columns
in a [`{page}`]($page.columns) set rule. To adjust the amount of space between
the columns, add a set rule on the [`columns` function]($columns), specifying
the `gutter` parameter.
</original>

```example
>>> #set page(height: 120pt)
#set page(columns: 2)
#set columns(gutter: 12pt)

#lorem(30)
```

正常情况下，科研论文的标题和摘要都是单栏的，正文则是双栏的。要达到这种效果，可用到 [`place` 函数]($place) 并指定 `{float: true}` 和 `{scope: "parent"}`，它可以使内容暂时脱离两栏布局。

<original>
Very commonly, scientific papers have a single-column title and abstract, while
the main body is set in two-columns. To achieve this effect, Typst's [`place`
function]($place) can temporarily escape the two-column layout by specifying
`{float: true}` and `{scope: "parent"}`:
</original>

```example:single
>>> #set page(height: 180pt)
#set page(columns: 2)
#set par(justify: true)

#place(
  top + center,
  float: true,
  scope: "parent",
  text(1.4em, weight: "bold")[
    Impacts of Odobenidae
  ],
)

== About seals in the wild
#lorem(80)
```

_浮动定位_ 是指那些被推至栏目或者整个页面的顶部或者底部的元素（的定位方式），其余内容在它们之间排列。这种定位方式经常用于[图表]($figure.placement)上。

<original>
_Floating placement_ refers to elements being pushed to the top or bottom of the
column or page, with the remaining content flowing in between. It is also
frequently used for [figures]($figure.placement).
</original>

### 在文档的任何地方使用分栏

若要在一个嵌套的布局（比如一个矩形）里加入分栏，你可以直接调用 [`columns` 函数]($columns)。不过，它只应在嵌套布局里使用。在页面的层级中更应该使用 page set rule，这样页面级的浮动、脚注以及行标号等处理起来更方便。

<original>
To create columns within a nested layout, e.g. within a rectangle, you can use
the [`columns` function]($columns) directly. However, it really should only be
used within nested layouts. At the page-level, the page set rule is preferrable
because it has better interactions with things like page-level floats,
footnotes, and line numbers.
</original>

```example
#rect(
  width: 6cm,
  height: 3.5cm,
  columns(2, gutter: 12pt)[
    In the dimly lit gas station,
    a solitary taxi stood silently,
    its yellow paint fading with
    time. Its windows were dark,
    its engine idle, and its tires
    rested on the cold concrete.
  ]
)
```

### 平衡分栏

如果文档最后一页的栏目在长度上差别很大，文档的布局就会显得不平衡不好看。这就是为什么排印家们经常会让最后一页的各栏目长度大致相同，以达到一种被称为平衡分栏的效果。Typst 目前还不能自动地去平衡分栏，但你可以在内容的恰当位置加入一个 [`[#colbreak()]`]($colbreak) 来手动创建合适的分栏。

<original>
If the columns on the last page of a document differ greatly in length, they may
create a lopsided and unappealing layout. That's why typographers will often
equalize the length of columns on the last page. This effect is called balancing
columns. Typst cannot yet balance columns automatically. However, you can
balance columns manually by placing [`[#colbreak()]`]($colbreak) at an
appropriate spot in your markup, creating the desired column break manually.
</original>


## 一次性的修改

如果你需要插入一个具有不同配置的页面，并不需要去覆盖当前的页面设置。比如，你可能需要插入一个翻转的页面用于容纳一个大型表格，或者调整标题页的边距与分栏。在这种情况下，你可以用函数的方式来调用 [`{page}`]($page)，传入你的内容，然后在其它参数上指定要覆盖的设置。这样，这些内容就会以调整后的设置被插入到文档中，并且会生成足量的页面。在这一调用之后，Typst 会将页面恢复到 set rule 里的配置。

<original>
You do not need to override your page settings if you need to insert a single
page with a different setup. For example, you may want to insert a page that's
flipped to landscape to insert a big table or change the margin and columns for
your title page. In this case, you can call [`{page}`]($page) as a function with
your content as an argument and the overrides as the other arguments. This will
insert enough new pages with your overridden settings to place your content on
them. Typst will revert to the page settings from the set rule after the call.
</original>

```example
>>> #set page("a6")
#page(flipped: true)[
  = Multiplication table

  #table(
    columns: 5 * (1fr,),
    ..for x in range(1, 10) {
      for y in range(1, 6) {
        (str(x*y),)
      }
    }
  )
]
```
