---
description: Typst's tutorial.
en_title: Advanced Styling
---

# 高级样式

在本教程的前两章里，你了解了如何用 Typst 编写文档以及修改其格式。不仅这篇你用前两章的知识编写的报告直接获得了一个 A，而且你的导师还想要基于它编写一篇会议论文！这样的话，这篇报告就需要遵循该会议的样式标准。让我们看看该如何达成这一点。

<original>
In the previous two chapters of this tutorial, you have learned how to write a
document in Typst and how to change its formatting. The report you wrote
throughout the last two chapters got a straight A and your supervisor wants to
base a conference paper on it! The report will of course have to comply with the
conference's style guide. Let's see how we can achieve that.
</original>

在我们开始之前，首先我们在 app 里创建一个团队，然后邀请你的导师加入这个团队。你可以通过编辑器左上角的左箭头按钮回到 app 控制台，然后，点击左侧工具栏里的加号创建一个团队，再点击这个新的团队，最后点击团队名称旁边的 manage team（管理团队）进入设置，在这里你可以通过电子邮件邀请你的导师。

<original>
Before we start, let's create a team, invite your supervisor and add them to the
team. You can do this by going back to the app dashboard with the back icon in
the top left corner of the editor. Then, choose the plus icon in the left
toolbar and create a team. Finally, click on the new team and go to its settings
by clicking 'manage team' next to the team name. Now you can invite your
supervisor by email.
</original>

![The team settings](3-advanced-team-settings.png)

接下来，将你的项目移动到团队里。先打开你的项目，点击左侧工具栏里的齿轮进入设置，然后在 owners（所有者）下拉框里选择你创建的新团队。不要忘了保存你的更改！

<original>
Next, move your project into the team: Open it, going to its settings by
choosing the gear icon in the left toolbar and selecting your new team from the
owners dropdown. Don't forget to save your changes!
</original>

现在，你的导师也可以编辑这个项目了，并且你们也可以实时查看更改。你可以加入我们的 [Discord](https://discord.gg/2uDybryKPe)，在里面寻找其他用户，与他们一起尝试团队功能。

<original>
Now, your supervisor can also edit the project and you can both see the changes
in real time. You can join our [Discord server](https://discord.gg/2uDybryKPe)
to find other users and try teams with them!
</original>

## 来自会议的指导规则

会议的官网上有着布局的指导规则，我们来具体看看：

<original>
The layout guidelines are available on the conference website. Let's take a look
at them:
</original>

- 文本应当是 11pt 的衬线字体
- 标题应当是 17pt 的粗体
- 论文包含单栏的摘要和两栏的正文
- 摘要应该居中
- 正文应该对齐
- 一级标题大小应为 13pt，居中，且用小型大写字母展示
- 二级标题应当在行内，斜体，且与正文字号相同
- 页面大小遵从 US letter 标准，页脚的中心位置标页数，每一页的右上角应包含论文标题

<original>
- The font should be an 11pt serif font
- The title should be in 17pt and bold
- The paper contains a single-column abstract and two-column main text
- The abstract should be centered
- The main text should be justified
- First level section headings should be 13pt, centered, and rendered in small
  capitals
- Second level headings are run-ins, italicized and have the same size as the
  body text
- Finally, the pages should be US letter sized, numbered in the center of the
  footer and the top right corner of each page should contain the title of the
  paper
</original>

对于其中的一部分要求，我们已经知道如何实现了，但是对于其它的部分，我们还需要了解更多的技巧才行。

<original>
We already know how to do many of these things, but for some of them, we'll need
to learn some new tricks.
</original>

## 使用恰当的 set rules 编写

我们先为文档编写一些 set rules。

<original>
Let's start by writing some set rules for the document.
</original>

```example
#set page(
>>>  margin: auto,
  paper: "us-letter",
  header: align(right)[
    A fluid dynamic model for
    glacier flow
  ],
  numbering: "1",
)
#set par(justify: true)
#set text(
  font: "Libertinus Serif",
  size: 11pt,
)

#lorem(600)
```

你应该已经很熟悉这里的大部分内容在做什么了。我们将文本的大小设置为 `{11pt}`，字体设置为 Libertinus Serif，同时启用了段落的对齐，将页面大小也设置为了 US letter。

<original>
You are already familiar with most of what is going on here. We set the text
size to `{11pt}` and the font to Libertinus Serif. We also enable paragraph
justification and set the page size to US letter.
</original>

这里的 `header` 参数是之前没有遇到过的。我们可以通过它来设置每一页页眉的内容。于是我们按照会议的标准在页眉中设定了论文的标题，并用 `align` 函数将文本右对齐。

<original>
The `header` argument is new: With it, we can provide content to fill the top
margin of every page. In the header, we specify our paper's title as requested
by the conference style guide. We use the `align` function to align the text to
the right.
</original>

然后是 `numbering` 参数。我们可以传入一个[标号模板]($numbering)来定义页面的标号形式。将它设为 `{"1"}` 时，Typst 只会直接显示页数，而设为 `{"(1/1)"}` 时，最终会显示出用括号包围的当前页数和总页数。我们甚至还可以传入一个完全自定义的函数来将页数展示为我们想要的样式。

<original>
Last but not least is the `numbering` argument. Here, we can provide a
[numbering pattern]($numbering) that defines how to number the pages. By
setting into to `{"1"}`, Typst only displays the bare page number. Setting it to
`{"(1/1)"}` would have displayed the current page and total number of pages
surrounded by parentheses. And we could even have provided a completely custom
function here to format things to our liking.
</original>

## 创建标题和摘要

先从标题开始。我们将其居中展示，并用 `[*星号*]` 包围转为粗体：

<original>
Now, let's add a title and an abstract. We'll start with the title. We center
align it and increase its font weight by enclosing it in `[*stars*]`.
</original>

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
#align(center, text(17pt)[
  *A fluid dynamic model
  for glacier flow*
])
```

看起来没问题。我们就地用到了 `text` 函数来覆盖先前的 text set rule。<br/>接下来添加作者列表。这篇论文是我们和导师们一起写的，所以我们加上自己和导师的名字。

<original>
This looks right. We used the `text` function to override the previous text
set rule locally, increasing the size to 17pt for the function's argument. Let's
also add the author list: Since we are writing this paper together with our
supervisor, we'll add our own and their name.
</original>

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
>>> #align(center, text(17pt)[
>>>   *A fluid dynamic model
>>>   for glacier flow*
>>> ])
#grid(
  columns: (1fr, 1fr),
  align(center)[
    Therese Tungsten \
    Artos Institute \
    #link("mailto:tung@artos.edu")
  ],
  align(center)[
    Dr. John Doe \
    Artos Institute \
    #link("mailto:doe@artos.edu")
  ]
)
```

这两个包含作者的块是并排排列的，这一布局方式用 [`grid`] 来实现。借助 grid，我们可以设置每一列的大小，并控制哪些列该放哪些内容。`columns` 参数接受一个 [相对长度]($relative) 或者 [分数]($fraction)（fraction）类型的参数。在这个例子中，我们传入了两个相等的分数值，使它将空闲区域分割为两个同等大小的列。<br/>随后我们向 grid 函数传入了两个内容参数，第一个是我们自己的具体信息，第二个是导师的。我们再次用到了 `align` 函数来居中列的内容。grid 函数接受任意多的内容参数，它会根据需要自动新增行，你也可以用 `rows` 参数手动设置行的大小。

<original>
The two author blocks are laid out next to each other. We use the [`grid`]
function to create this layout. With a grid, we can control exactly how large
each column is and which content goes into which cell. The `columns` argument
takes an array of [relative lengths]($relative) or [fractions]($fraction). In
this case, we passed it two equal fractional sizes, telling it to split the
available space into two equal columns. We then passed two content arguments to
the grid function. The first with our own details, and the second with our
supervisors'. We again use the `align` function to center the content within the
column. The grid takes an arbitrary number of content arguments specifying the
cells. Rows are added automatically, but they can also be manually sized with
the `rows` argument.
</original>

现在我们来添加一个摘要。记住，该会议希望摘要是居中展示的，不对齐。

<original>
Now, let's add the abstract. Remember that the conference wants the abstract to
be set ragged and centered.
</original>

```example:0,0,612,317.5
>>> #set text(font: "Libertinus Serif", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(right + horizon)[
>>>     A fluid dynamic model for
>>>     glacier flow
>>>   ],
>>>   numbering: "1",
>>> )
>>>
>>> #align(center, text(17pt)[
>>>   *A fluid dynamic model
>>>   for glacier flow*
>>> ])
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
<<< ...

#align(center)[
  #set par(justify: false)
  *Abstract* \
  #lorem(80)
]
>>> #lorem(600)
```

很好！注意到一点，我们在 `align` 函数的内容参数中用到了一个 set rule 来取消对摘要的对齐。即时是在第一个 set rule 之后指定的，它并不会影响到文档的剩余内容，这是因为内容块会为样式划清范围，内容块里的任何设置都只会影响其本身的内容。

<original>
Well done! One notable thing is that we used a set rule within the content
argument of `align` to turn off justification for the abstract. This does not
affect the remainder of the document even though it was specified after the
first set rule because content blocks _scope_ styling. Anything set within a
content block will only affect the content within that block.
</original>

另外有一个微调是将论文标题存储在一个变量里，这样我们就不需要在页眉和文档标题中反复输入了。我们可以通过 `{let}` 关键字实现这一点。

<original>
Another tweak could be to save the paper title in a variable, so that we do not
have to type it twice, for header and title. We can do that with the `{let}`
keyword:
</original>

```example:single
#let title = [
  A fluid dynamic model
  for glacier flow
]

<<< ...

>>> #set text(font: "Libertinus Serif", 11pt)
>>> #set par(justify: true)
#set page(
>>>   "us-letter",
>>>   margin: auto,
  header: align(
    right + horizon,
    title
  ),
<<<   ...
>>>   numbering: "1",
)

#align(center, text(17pt)[
  *#title*
])

<<< ...

>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>>
>>> #lorem(600)
```

将内容赋值给 `title` 变量后，我们就可以在函数和 markup 中（以 `#` 开头，类似于函数调用）使用了。这样，如果我们想换一个标题，就只需要在一处进行修改。

<original>
After we bound the content to the `title` variable, we can use it in functions
and also within markup (prefixed by `#`, like functions). This way, if we decide
on another title, we can easily change it in one place.
</original>

## 添加分栏和标题

上面的论文看起来很像是一道铅墙。要解决这一点，我们需要向文档添加一些标题，然后将论文切换为两栏布局。幸运的是这很好实现：我们只需要在 `page` set rule 里加上 [`columns`] 参数。

<original>
The paper above unfortunately looks like a wall of lead. To fix that, let's add
some headings and switch our paper to a two-column layout. Fortunately, that's
easy to do: We just need to amend our `page` set rule with the `columns`
argument.
</original>

在添加了 `{columns: 2}` 参数后，整个文档就被包在了两个栏目中。但是这同时也在影响着标题和作者概览。为了让它们保持跨越整个页面（译者注：即依然占整个页面的宽度，不分栏），可将其包围在 [`{place}`]($place) 函数调用中。place 函数接收两个位置参数：对齐方式和对齐内容。同时，借助具名参数 `{scope}` 可以设置这些项目应当相对于当前栏目还是相对于其父级元素（即整个页面）进行定位。另外还有一点可以进行配置：如果没有传入其它参数，`{place}` 会将其处理的内容移出文档流，并在不影响其它内容定位的情况下，将内容放置在其它内容之上（译者注：此处深入理解请参考 CSS 中的 `position: absolute` 布局定位方式）。

<original>
By adding `{columns: 2}` to the argument list, we have wrapped the whole
document in two columns. However, that would also affect the title and authors
overview. To keep them spanning the whole page, we can wrap them in a function
call to [`{place}`]($place). Place expects an alignment and the content it
should place as positional arguments. Using the named `{scope}` argument, we can
decide if the items should be placed relative to the current column or its
parent (the page). There is one more thing to configure: If no other arguments
are provided, `{place}` takes its content out of the flow of the document and
positions it over the other content without affecting the layout of other
content in its container:
</original>

```example
#place(
  top + center,
  rect(fill: black),
)
#lorem(30)
```

如果在这里没有用到 `{place}`，这个正方形会另起一行，但在这里它与之后的几行文字重叠了，这几行文字表现得如同该正方形不存在一般。要改变这种状态，我们可以传入 `{float: true}` 参数来确保定位到页面顶部或者底部的项目的空间不会被其它内容所占据。

<original>
If we hadn't used `{place}` here, the square would be in its own line, but here
it overlaps the few lines of text following it. Likewise, that text acts like as
if there was no square. To change this behavior, we can pass the argument
`{float: true}` to ensure that the space taken up by the placed item at the top
or bottom of the page is not occupied by any other content.
</original>

```example:single
>>> #let title = [
>>>   A fluid dynamic model
>>>   for glacier flow
>>> ]
>>>
>>> #set text(font: "Libertinus Serif", 11pt)
>>> #set par(justify: true)
>>>
#set page(
>>> margin: auto,
  paper: "us-letter",
  header: align(
    right + horizon,
    title
  ),
  numbering: "1",
  columns: 2,
)

#place(
  top + center,
  float: true,
  scope: "parent",
  clearance: 2em,
)[
>>>  #text(
>>>    17pt,
>>>    weight: "bold",
>>>    title,
>>>  )
>>>
>>>  #grid(
>>>    columns: (1fr, 1fr),
>>>    [
>>>      Therese Tungsten \
>>>      Artos Institute \
>>>      #link("mailto:tung@artos.edu")
>>>    ],
>>>    [
>>>      Dr. John Doe \
>>>      Artos Institute \
>>>      #link("mailto:doe@artos.edu")
>>>    ]
>>>  )
<<<   ...

  #par(justify: false)[
    *Abstract* \
    #lorem(80)
  ]
]

= Introduction
#lorem(300)

= Related Work
#lorem(200)
```

在这个例子中，我们还用到了 `{place}` 函数的 `clearance` 参数在元素和正文之间加上空隙，而没有用到 [`{v}`]($v) 函数。部分显式的 `{align(center, ..)}` 调用也可以去掉，因为这些部分继承了定位语句中规定的居中布局。

<original>
In this example, we also used the `clearance` argument of the `{place}` function
to provide the space between it and the body instead of using the [`{v}`]($v)
function. We can also remove the explicit `{align(center, ..)}` calls around the
various parts since they inherit the center alignment from the placement.
</original>

现在只有一件事情要做：为标题设置样式。我们需要将它们居中，并采用小型大写字母（small capitals）展示。由于 `heading` 函数并没有提供任何相关的功能，我们需要自行编写标题的 show rule。

<original>
Now there is only one thing left to do: Style our headings. We need to make them
centered and use small capitals. Because the `heading` function does not offer
a way to set any of that, we need to write our own heading show rule.
</original>

```example:50,250,265,270
>>> #let title = [
>>>   A fluid dynamic model
>>>   for glacier flow
>>> ]
>>>
>>> #set text(font: "Libertinus Serif", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     title
>>>   ),
>>>   numbering: "1",
>>>   columns: 2,
>>> )
#show heading: it => [
  #set align(center)
  #set text(13pt, weight: "regular")
  #block(smallcaps(it.body))
]

<<< ...
>>>
>>> #place(
>>>   top + center,
>>>   float: true,
>>>   scope: "parent",
>>>   clearance: 2em,
>>> )[
>>>   #text(
>>>     17pt,
>>>     weight: "bold",
>>>     title,
>>>   )
>>>
>>>   #grid(
>>>     columns: (1fr, 1fr),
>>>     [
>>>       Therese Tungsten \
>>>       Artos Institute \
>>>       #link("mailto:tung@artos.edu")
>>>     ],
>>>     [
>>>       Dr. John Doe \
>>>       Artos Institute \
>>>       #link("mailto:doe@artos.edu")
>>>     ]
>>>   )
>>>
>>>   #par(justify: false)[
>>>     *Abstract* \
>>>     #lorem(80)
>>>   ]
>>> ]
>>>
>>> = Introduction
>>> #lorem(35)
>>>
>>> == Motivation
>>> #lorem(45)
```

看起来真不错！<br/>我们用到了一个控制所有标题的 show rule，给它了一个将标题作为参数传递的函数。该参数可以被直接当作文本内容使用，但它还附带了一些数据，包括 `title`、`numbers` 和 `level`，我们可以利用这些数据来创建我们的自定义外观。这里，我们将标题居中显示，由于标题默认是粗体，随后我们将字重设定为 `{"regular"}`，再用 [`smallcaps`] 函数来以小型大写字母渲染标题内容。

<original>
This looks great! We used a show rule that applies to all headings. We give it a
function that gets passed the heading as a parameter. That parameter can be used
as content but it also has some fields like `title`, `numbers`, and `level` from
which we can compose a custom look. Here, we are center-aligning, setting the
font weight to `{"regular"}` because headings are bold by default, and use the
[`smallcaps`] function to render the heading's title in small capitals.
</original>

现在唯一的问题是，所有的标题都变得一样了。Motivation 和 Problem Statement 小节应该是行内的斜体标题，但现在它们和当前大节的标题一样了。我们可以在我们定义的 set rule 上面加上一个 `where` 选择器来解决：这是一个 headings上的[方法]($scripting/#methods)（其他元素上也有），在此可用来根据层级选择标题。我们可以用它来区分大节和小节的标题。

<original>
The only remaining problem is that all headings look the same now. The
"Motivation" and "Problem Statement" subsections ought to be italic run in
headers, but right now, they look indistinguishable from the section headings. We
can fix that by using a `where` selector on our set rule: This is a
[method]($scripting/#methods) we can call on headings (and other
elements) that allows us to filter them by their level. We can use it to
differentiate between section and subsection headings:
</original>

```example:50,250,265,245
>>> #let title = [
>>>   A fluid dynamic model
>>>   for glacier flow
>>> ]
>>>
>>> #set text(font: "Libertinus Serif", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     title
>>>   ),
>>>   numbering: "1",
>>>   columns: 2,
>>> )
>>>
#show heading.where(
  level: 1
): it => block(width: 100%)[
  #set align(center)
  #set text(13pt, weight: "regular")
  #smallcaps(it.body)
]

#show heading.where(
  level: 2
): it => text(
  size: 11pt,
  weight: "regular",
  style: "italic",
  it.body + [.],
)
>>>
>>> #place(
>>>   top + center,
>>>   float: true,
>>>   scope: "parent",
>>>   clearance: 2em,
>>> )[
>>>   #text(
>>>     17pt,
>>>     weight: "bold",
>>>     title,
>>>   )
>>>
>>>  #grid(
>>>    columns: (1fr, 1fr),
>>>    [
>>>      Therese Tungsten \
>>>      Artos Institute \
>>>      #link("mailto:tung@artos.edu")
>>>    ],
>>>    [
>>>      Dr. John Doe \
>>>      Artos Institute \
>>>      #link("mailto:doe@artos.edu")
>>>    ]
>>>  )
>>>
>>>   #par(justify: false)[
>>>     *Abstract* \
>>>     #lorem(80)
>>>   ]
>>> ]
>>>
>>> = Introduction
>>> #lorem(35)
>>>
>>> == Motivation
>>> #lorem(45)
```

不错！我们编写了两个 show rules，它们选择性地作用于第一级和第二级的标题。我们用到了 `where` 选择器来根据层级筛选目标标题，然后将小节的标题设为行内渲染，还在它的最后加上了一个点（period）。

<original>
This looks great! We wrote two show rules that each selectively apply to the
first and second level headings. We used a `where` selector to filter the
headings by their level. We then rendered the subsection headings as run-ins. We
also automatically add a period to the end of the subsection headings.
</original>

我们来回顾一下会议的样式标准：

- 文本应当是 11pt 的衬线字体 ✓
- 标题应当是 17pt 的粗体 ✓
- 论文包含单栏的摘要和两栏的正文 ✓
- 摘要应该居中 ✓
- 正文应该对齐 ✓
- 一级标题大小应为 13pt，居中，且用小型大写字母展示 ✓
- 二级标题应当在行内，斜体，且与正文字号相同 ✓
- 页面大小遵从 US letter 标准，页脚的中心位置标页数，每一页的右上角应包含论文标题 ✓

<original>
Let's review the conference's style guide:
- The font should be an 11pt serif font ✓
- The title should be in 17pt and bold ✓
- The paper contains a single-column abstract and two-column main text ✓
- The abstract should be centered ✓
- The main text should be justified ✓
- First level section headings should be centered, rendered in small caps and in
  13pt ✓
- Second level headings are run-ins, italicized and have the same size as the
  body text ✓
- Finally, the pages should be US letter sized, numbered in the center and the
  top right corner of each page should contain the title of the paper ✓
</original>

我们现在已经遵从了所有样式规则，可以向会议提交我们的论文了！论文的最终效果是这样的：

<original>
We are now in compliance with all of these styles and can submit the paper to
the conference! The finished paper looks like this:
</original>

<img
  src="3-advanced-paper.png"
  alt="The finished paper"
  style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 500px; max-width: 100%; display: block; margin: 24px auto;"
>

## 回顾

在本章中，你学到了如何创建页眉和页脚、如何通过作用域来就地覆盖样式、如何使用 [`grid`] 函数创建更复杂的布局以及如何为单个函数或整个文档编写 show rule。同时，你还了解到如何使用 [`where` 选择器]($styling/#show-rules) 来根据层级筛选标题。

<original>
You have now learned how to create headers and footers, how to use functions and
scopes to locally override styles, how to create more complex layouts with the
[`grid`] function and how to write show rules for individual functions, and the
whole document. You also learned how to use the
[`where` selector]($styling/#show-rules) to filter the headings by their level.
</original>

论文大获成功！你在会议中遇到了许多志同道合的研究人员，并计划了一个新的项目，准备下一年在同一地点发表。你仍需要根据这个风格规则来编写论文。你会不会想要为你自己和团队编写一个可以节省时间的模板呢？

<original>
The paper was a great success! You've met a lot of like-minded researchers at
the conference and are planning a project which you hope to publish at the same
venue next year. You'll need to write a new paper using the same style guide
though, so maybe now you want to create a time-saving template for you and your
team?
</original>

在下一章里，我们会了解如何创建可以在不同文档中复用的模板。这是一个更高阶的议题，所以如果现在你没有精力看的话，可以以后有时间再来看看。

<original>
In the next section, we will learn how to create templates that can be reused in
multiple documents. This is a more advanced topic, so feel free to come back
to it later if you don't feel up to it right now.
</original>
