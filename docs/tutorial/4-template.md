---
description: Typst's tutorial.
en_title: Making a Template
---

# 制作模板

在本教程的前三章，你了解了如何用 Typst 写出一篇文档、添加基础的样式以及根据出版样式要求去深度定制文档的外观。在上一章里你所编写的论文非常成功，有人让你为同一个会议再接着写一篇文章。这次，你还想用上一章里你定义样式，希望将它变为可复用的模板。在本章中，你会了解如何为你和你的团队写出仅需一个 show rule 即可调用的模板。让我们开始吧！

<original>
In the previous three chapters of this tutorial, you have learned how to write a
document in Typst, apply basic styles, and customize its appearance in-depth to
comply with a publisher's style guide. Because the paper you wrote in the
previous chapter was a tremendous success, you have been asked to write a
follow-up article for the same conference. This time, you want to take the style
you created in the previous chapter and turn it into a reusable template. In
this chapter you will learn how to create a template that you and your team can
use with just one show rule. Let's get started!
</original>

## 玩具模板

在 Typst 中，模板是一种可包含整个文档的函数。要想了解如何写出这样一个函数，我们需要先回顾一下函数该怎么写。既然函数可以做几乎任何事情，为什么不来点奇思妙想呢？请看：

<original>
In Typst, templates are functions in which you can wrap your whole document. To
learn how to do that, let's first review how to write your very own functions.
They can do anything you want them to, so why not go a bit crazy?
</original>

```example
#let amazed(term) = box[✨ #term ✨]

You are #amazed[beautiful]!
```

这个函数接受单一的参数 `term`，返回一个用星星包围 `term` 的内容块。我们把结果整体放在一个 box 里，这样，这个在这里令我们啧啧称奇的东西（`term` 参数的内容）就不会因为换行而与星星分开了。

<original>
This function takes a single argument, `term`, and returns a content block with
the `term` surrounded by sparkles. We also put the whole thing in a box so that
the term we are amazed by cannot be separated from its sparkles by a line break.
</original>

许多 Typst 中的函数具有可选的具名参数，我们写的函数也可以带这样的参数。让我们给函数加一个可用来选文字颜色的参数，并给予一个默认的颜色用于它缺省的情况。

<original>
Many functions that come with Typst have optional named parameters. Our
functions can also have them. Let's add a parameter to our function that lets us
choose the color of the text. We need to provide a default color in case the
parameter isn't given.
</original>

```example
#let amazed(term, color: blue) = {
  text(color, box[✨ #term ✨])
}

You are #amazed[beautiful]!
I am #amazed(color: purple)[amazed]!
```

现在，向一个全局 show rule 中传入我们编写的函数，就可以使模板正常工作了。现在用我们写的 `amazed` 函数来试试。

<original>
Templates now work by using an "everything" show rule that applies the custom
function to our whole document. Let's do that with our `amazed` function.
</original>

```example
>>> #let amazed(term, color: blue) = {
>>>   text(color, box[✨ #term ✨])
>>> }
#show: amazed
I choose to focus on the good
in my life and let go of any
negative thoughts or beliefs.
In fact, I am amazing!
```

这样，整个文档都会被传入到 `amazed` 函数中，这与将所有内容用函数包含是一样的效果。这个函数看起来并不是很有用，但如果我们与 set rules 和具名参数相结合，它也可以变得非常强力。

<original>
Our whole document will now be passed to the `amazed` function, as if we
wrapped it around it. This is not especially useful with this particular
function, but when combined with set rules and named arguments, it can be very
powerful.
</original>

## 嵌入 set rules 和 show rules

要向模板添加 set 和 show rules，我们可以在函数中的内容块里调用 `set` 和 `show`，然后将文档放在里面。

<original>
To apply some set and show rules to our template, we can use `set` and `show`
within a content block in our function and then insert the document into
that content block.
</original>

```example
#let template(doc) = [
  #set text(font: "Inria Serif")
  #show "something cool": [Typst]
  #doc
]

#show: template
I am learning something cool today.
It's going great so far!
```

就像我们在前面的章节里看到的那样，set rules 会作用于当前内容块里的所有内容。由于这个全局 show rule 将我们的整个文档传给了 `template` 函数，这里针对文本的 set rule 和针对字符串的 show rule 会作用于整个文章。据此，我们可以创建一个模板来复现我们上一章里论文所用的正文样式。

<original>
Just like we already discovered in the previous chapter, set rules will apply to
everything within their content block. Since the everything show rule passes our
whole document to the `template` function, the text set rule and string show
rule in our template will apply to the whole document. Let's use this knowledge
to create a template that reproduces the body style of the paper we wrote in the
previous chapter.
</original>

```example
#let conf(title, doc) = {
  set page(
    paper: "us-letter",
>>> margin: auto,
    header: align(
      right + horizon,
      title
    ),
<<<     ...
  )
  set par(justify: true)
  set text(
    font: "Libertinus Serif",
    size: 11pt,
  )

  // Heading show rules.
<<<   ...
>>>  show heading.where(
>>>    level: 1
>>>  ): it => block(
>>>    align(center,
>>>      text(
>>>        13pt,
>>>        weight: "regular",
>>>        smallcaps(it.body),
>>>      )
>>>    ),
>>>  )
>>>  show heading.where(
>>>    level: 2
>>>  ): it => box(
>>>    text(
>>>      11pt,
>>>      weight: "regular",
>>>      style: "italic",
>>>      it.body + [.],
>>>    )
>>>  )

  columns(2, doc)
}

#show: doc => conf(
  [Paper title],
  doc,
)

= Introduction
#lorem(90)

<<< ...
>>> == Motivation
>>> #lorem(140)
>>>
>>> == Problem Statement
>>> #lorem(50)
>>>
>>> = Related Work
>>> #lorem(200)
```

我们从上一章里复制粘贴了大部分的代码。唯一的两个区别是，这次我们把所有内容都包围在了 `conf` 函数中，并直接在 `doc` 参数上调用了分栏函数，因为它已经包含了文档的内容。此外，我们还用到了一个花括号包围的代码块而非内容块。通过这种方式，我们不再需要在每一个 rule 和函数调用前加 `#` 了，不过我们也不能再直接在里边写 markup 了。

<original>
We copy-pasted most of that code from the previous chapter. The only two
differences are that we wrapped everything in the function `conf` and are
calling the columns function directly on the `doc` argument as it already
contains the content of the document. Moreover, we used a curly-braced code
block instead of a content block. This way, we don't need to prefix all set
rules and function calls with a `#`. In exchange, we cannot write markup
directly into it anymore.
</original>

另外注意这个标题是从哪里来的：我们先前将它放在了一个变量里。现在我们将所需标题放在了模板函数参数的第一个的位置，因此我们应该在调用模板函数的 show rule 里面填入它。

<original>
Also note where the title comes from: We previously had it inside of a variable.
Now, we are receiving it as the first parameter of the template function.
Thus, we must specify it in the show rule where we call the template.
</original>

## 带有具名参数的模板

上一章的论文中还包含了一个大标题和一个作者列表，我们现在将它们加到模板里。除了标题外，我们还想要让模板接受一组作者名称和隶属关系以及一个论文摘要。为了使调用时更清晰，我们将它们定义为具名参数：

<original>
Our paper in the previous chapter had a title and an author list. Let's add these
things to our template. In addition to the title, we want our template to accept
a list of authors with their affiliations and the paper's abstract. To keep
things readable, we'll add those as named arguments. In the end, we want it to
work like this:
</original>

```typ
#show: doc => conf(
  title: [Towards Improved Modelling],
  authors: (
    (
      name: "Theresa Tungsten",
      affiliation: "Artos Institute",
      email: "tung@artos.edu",
    ),
    (
      name: "Eugene Deklan",
      affiliation: "Honduras State",
      email: "e.deklan@hstate.hn",
    ),
  ),
  abstract: lorem(80),
  doc,
)

...
```

我们现在开始写这个函数。首先，我们为 `title` 参数添加一个默认值，这样我们在调用模板的时候也可以不带标题。我们还为具名的 `authors` 和 `abstract` 参数添加了空的默认值。然后，我们从上一章将生成标题、摘要和作者表的相关代码复制到模板里，替换掉那些固定的内容。

<original>
Let's build this new template function. First, we add a default value to the
`title` argument. This way, we can call the template without specifying a title.
We also add the named `authors` and `abstract` parameters with empty defaults.
Next, we copy the code that generates title, abstract and authors from the
previous chapter into the template, replacing the fixed details with the
parameters.
</original>

新的 `authors` 参数接受一个 [字典]($dictionary) 数组，其中的字典具有 `name`、`affiliation` 和 `email` 键。由于作者的数量是任意的，我们需要动态地推断作者列表是需要一栏两栏还是三栏。我们先用 `authors` 数组的 [`.len()`]($array.len) 方法来获取作者的数量，然后将其与 3 之间的最小值（即做计算 min(count, 3)）设为最终的列数，这样栏数就不会超过三。如果作者数量超过三个，会自动另起一行。我们还向 `grid` 函数传入了一个 `row-gutter` 参数，这样行与行之间就不至于过于紧凑。要从字典中取出作者的各项数据，我们用到了 [字段访问语法]($scripting/#fields)。

<original>
The new `authors` parameter expects an [array] of [dictionaries]($dictionary)
with the keys `name`, `affiliation` and `email`. Because we can have an
arbitrary number of authors, we dynamically determine if we need one, two or
three columns for the author list. First, we determine the number of authors
using the [`.len()`]($array.len) method on the `authors` array. Then, we set the
number of columns as the minimum of this count and three, so that we never
create more than three columns. If there are more than three authors, a new row
will be inserted instead. For this purpose, we have also added a `row-gutter`
parameter to the `grid` function. Otherwise, the rows would be too close
together. To extract the details about the authors from the dictionary, we use
the [field access syntax]($scripting/#fields).
</original>

我们仍需要为每位作者向 grid 传递参数，这时数组（array）的 [`map` 方法]($array.map)正好能派上用场。它接受一个函数，然后会用数组的每一项作为参数调用一遍该函数。我们向 map 传递一个格式化每位作者的详细信息并返回一组内容值的函数。现在得到了一组我们想传入 grid 参数（用到 [`spread` 操作符]($arguments)）的值。`spread` 操作符接受一个数组，它将该数组中的每一项分别传入函数参数的对应位置。

<original>
We still have to provide an argument to the grid for each author: Here is where
the array's [`map` method]($array.map) comes in handy. It takes a function as an
argument that gets called with each item of the array. We pass it a function
that formats the details for each author and returns a new array containing
content values. We've now got one array of values that we'd like to use as
multiple arguments for the grid. We can do that by using the
[`spread` operator]($arguments). It takes an array and applies each of its items
as a separate argument to the function.
</original>

我们编写的结果是这样的：

<original>
The resulting template function looks like this:
</original>

```typ
#let conf(
  title: none,
  authors: (),
  abstract: [],
  doc,
) = {
  // Set and show rules from before.
<<<   ...

  set align(center)
  text(17pt, title)

  let count = authors.len()
  let ncols = calc.min(count, 3)
  grid(
    columns: (1fr,) * ncols,
    row-gutter: 24pt,
    ..authors.map(author => [
      #author.name \
      #author.affiliation \
      #link("mailto:" + author.email)
    ]),
  )

  par(justify: false)[
    *Abstract* \
    #abstract
  ]

  set align(left)
  columns(2, doc)
}
```

## 单文件

大多数情况下，模板是在一个单独的文件里编写的，使用时在文档中导入。这样，你所编写的主文件就不会变乱，同时也方便了模板的调用。我们在文件面板中点击加号，创建一个新的文本文档，命名为 `conf.typ`。然后将 `conf` 函数的定义移动到这个新建的文件里。现在你可以通过在 show rule 前面加上一句导入语句（`{import} 路径: 你想要导入函数的名称`）来使用它。

<original>
Most of the time, a template is specified in a different file and then imported
into the document. This way, the main file you write in is kept clutter free and
your template is easily reused. Create a new text file in the file panel by
clicking the plus button and name it `conf.typ`. Move the `conf` function
definition inside of that new file. Now you can access it from your main file by
adding an import before the show rule. Specify the path of the file between the
`{import}` keyword and a colon, then name the function that you
want to import.
</original>

```example:single
>>> #let conf(
>>>   title: none,
>>>   authors: (),
>>>   abstract: [],
>>>   doc,
>>> ) = {
>>>  set text(font: "Libertinus Serif", 11pt)
>>>  set par(justify: true)
>>>  set page(
>>>    "us-letter",
>>>    margin: auto,
>>>    header: align(
>>>      right + horizon,
>>>      title
>>>    ),
>>>    numbering: "1",
>>>  )
>>>
>>>  show heading.where(
>>>    level: 1
>>>  ): it => block(
>>>    align(center,
>>>      text(
>>>        13pt,
>>>        weight: "regular",
>>>        smallcaps(it.body),
>>>      )
>>>    ),
>>>  )
>>>  show heading.where(
>>>    level: 2
>>>  ): it => box(
>>>    text(
>>>      11pt,
>>>      weight: "regular",
>>>      style: "italic",
>>>      it.body + [.],
>>>    )
>>>  )
>>>
>>>  set align(center)
>>>  text(17pt, title)
>>>
>>>  let count = calc.min(authors.len(), 3)
>>>  grid(
>>>    columns: (1fr,) * count,
>>>    row-gutter: 24pt,
>>>    ..authors.map(author => [
>>>      #author.name \
>>>      #author.affiliation \
>>>      #link("mailto:" + author.email)
>>>    ]),
>>>  )
>>>
>>>  par(justify: false)[
>>>    *Abstract* \
>>>    #abstract
>>>  ]
>>>
>>>  set align(left)
>>>  columns(2, doc)
>>>}
<<< #import "conf.typ": conf
#show: doc => conf(
  title: [
    Towards Improved Modelling
  ],
  authors: (
    (
      name: "Theresa Tungsten",
      affiliation: "Artos Institute",
      email: "tung@artos.edu",
    ),
    (
      name: "Eugene Deklan",
      affiliation: "Honduras State",
      email: "e.deklan@hstate.hn",
    ),
  ),
  abstract: lorem(80),
  doc,
)

= Introduction
#lorem(90)

== Motivation
#lorem(140)

== Problem Statement
#lorem(50)

= Related Work
#lorem(200)
```

现在我们已经成功把一篇会议论文转成了可复用的模板！为什么不在 [Typst 的 Discord](https://discord.gg/2uDybryKPe) 上分享一番呢？这样其他人也可以用到它了。

<original>
We have now converted the conference paper into a reusable template for that
conference! Why not share it on
[Typst's Discord server](https://discord.gg/2uDybryKPe) so that others can use
it too?
</original>

## 回顾

恭喜，你已经读完了 Typst 的入门教程！在这一部分，你了解了如何自己定义函数，如何创建和应用（定义了可复用文档样式的）模板。你现在的所学已经十分深入了。现在你可以用 Typst 编写你自己的文档，然后分享给他人了。

<original>
Congratulations, you have completed Typst's Tutorial! In this section, you have
learned how to define your own functions and how to create and apply templates
that define reusable document styles. You've made it far and learned a lot. You
can now use Typst to write your own documents and share them with others.
</original>

我们目前仍然是一个非常年轻的项目，期待着任何反馈。如果你有任何问题或建议，或者你发现了 bug，请在 [Typst 的 Discord](https://discord.gg/2uDybryKPe)、[联系表](https://typst.app/contact)或者[社交媒体](https://twitter.com/typstapp)上告诉我们。

<original>
We are still a super young project and are looking for feedback. If you have any
questions, suggestions or you found a bug, please let us know on
[Typst's Discord server](https://discord.gg/2uDybryKPe), on our
[contact form](https://typst.app/contact), or on
[social media.](https://twitter.com/typstapp)
</original>

所以你还在等什么呢？现在[注册](https://typst.app)然后写点什么吧！

<original>
So what are you waiting for? [Sign up](https://typst.app) and write something!
</original>
