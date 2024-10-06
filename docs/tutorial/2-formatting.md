---
description: Typst's tutorial.
en_title: Formatting
---

# 格式化

目前，你已经在报告中写了一些文本、公式并插入了一些图片，但是它仍然看起来很平淡。你的助教（译者注：可理解为批改作业的人）现在还不知道你正用着一套新的排版系统，而你希望你的报告能和其他同学提交的相称。在这一张里，我们将了解如何使用 Typst 的样式系统来安排你的报告格式。

<original>
So far, you have written a report with some text, a few equations and images.
However, it still looks very plain. Your teaching assistant does not yet know
that you are using a new typesetting system, and you want your report to fit in
with the other student's submissions. In this chapter, we will see how to format
your report using Typst's styling system.
</original>

## Set rules

前面的章节中提到，Typst 既有可 _插入_ 内容的函数（例如 [`image`]），也有 _修改_ 作为参数接收的内容的函数（例如 [`align`]）。假定你现在想要对齐报告中的每一行，你的第一个念头或许就是找到相应的函数并将整个文档包围在（参数）里面。

<original>
As we have seen in the previous chapter, Typst has functions that _insert_
content (e.g. the [ function) and others that _manipulate_ content that
they received as arguments (e.g. the [`align`] function). The first impulse you
might have when you want, for example, to justify the report, could be to look
for a function that does that and wrap the complete document in it.
</original>

```example
#par(justify: true)[
  = Background
  In the case of glaciers, fluid
  dynamics principles can be used
  to understand how the movement
  and behaviour of the ice is
  influenced by factors such as
  temperature, pressure, and the
  presence of other fluids (such as
  water).
]
```

等等，函数的参数难道不应该用圆括号括起来吗？为什么在圆括号的 _后面_ 又有一个方括号呢？答案是，由于向函数传递内容是 Typst 中十分常见的一件事情，因此它存在一种特殊的语法：并非将内容放到参数列表里，而是可以直接写在方括号里，并接在所有的一般参数之后。这样做节省了标点。

<original>
Wait, shouldn't all arguments of a function be specified within parentheses? Why
is there a second set of square brackets with content _after_ the parentheses?
The answer is that, as passing content to a function is such a common thing to
do in Typst, there is special syntax for it: Instead of putting the content
inside of the argument list, you can write it in square brackets directly after
the normal arguments, saving on punctuation.
</original>

如上所示，这样做是可行的。[`par`] 函数将输入的所有行都对齐了。然而将文档包围在数不清的函数里的同时选择性地原地添加样式，文档很快就会变得臃肿。

<original>
As seen above, that works. The [`par`] function justifies all paragraphs within
it. However, wrapping the document in countless functions and applying styles
selectively and in-situ can quickly become cumbersome.
</original>

幸运的是，Typst 中有一个更优雅的解法。利用 _set rules_，你可以将样式属性一次性应用到特定的所有内容上。可通过输入 `{set}` 关键字编写 set rules，其后跟上你想要设置的属性的相关函数，和一个用圆括号括起来的包含参数的列表。

<original>
Fortunately, Typst has a more elegant solution. With _set rules,_ you can apply
style properties to all occurrences of some kind of content. You write a set
rule by entering the `{set}` keyword, followed by the name of the function whose
properties you want to set, and a list of arguments in parentheses.
</original>

```example
#set par(justify: true)

= Background
In the case of glaciers, fluid
dynamics principles can be used
to understand how the movement
and behaviour of the ice is
influenced by factors such as
temperature, pressure, and the
presence of other fluids (such as
water).
```

<div class="info-box">
想从更技术的角度来了解这里发生了什么吗？

Set rules 可以解释为如下概念：为将来的所有对该函数的调用设置部分参数的默认值。
</div>

<original>
<div class="info-box">

Want to know in more technical terms what is happening here?

Set rules can be conceptualized as setting default values
for some of the parameters of a function for all future
uses of that function.
</div>
</original>

## 自动补全面板

如果你跟随着步骤，在 app 中尝试了一些功能，你可能会发现每次在你输入 `#` 后，总会弹出一个向你展示可用函数的面板，而在参数列表中，它会向你展示可用的参数。这就是自动补全面板。在你编写文档的过程中，自动补全面板的用处很大：你可以按下回车键来应用它的建议，也可以使用箭头键选定你想要的那个补全。这个面板可以用 Esc 键来关闭，再次输入 `#` 或者按下 <kbd>Ctrl</kbd>+<kbd>空格</kbd> 时会重新打开。使用自动补全面板可以发觉函数的正确参数。大多数的建议下伴随有小字用途介绍。

<original>
If you followed along and tried a few things in the app, you might have noticed
that always after you enter a `#` character, a panel pops up to show you the
available functions, and, within an argument list, the available parameters.
That's the autocomplete panel. It can be very useful while you are writing your
document: You can apply its suggestions by hitting the Return key or navigate to
the desired completion with the arrow keys. The panel can be dismissed by
hitting the Escape key and opened again by typing `#` or hitting
<kbd>Ctrl</kbd> + <kbd>Space</kbd>. Use the autocomplete panel to discover the
right arguments for functions. Most suggestions come with a small description of
what they do.
</original>

![Autocomplete panel](2-formatting-autocomplete.png)

## Set up the page { #page-setup }

回到 set rules 上面：在编写一个规则的时候，你会按照你想要定义样式的元素类型来选择函数。下面列出了一些在 set rules 中常用的函数。

<original>
Back to set rules: When writing a rule, you choose the function depending on
what type of element you want to style. Here is a list of some functions that
are commonly used in set rules:
</original>

- [`text`] 用于设置字体族（font family）、大小、颜色以及其他文本的属性
- [`page`] 用于设置页面大小、边距、标题，以及启用列（columns）和页脚（footers）
- [`par`] 用于对齐段落，设置行距等
- [`heading`] 用于设置标题的外观和启用标号（numbering）
- [`document`] 用于设置输出的 PDF 文件的元数据，例如标题属性和作者属性

<original>
- [`text`] to set font family, size, color, and other properties of text
- [`page`] to set the page size, margins, headers, enable columns, and footers
- [`par`] to justify paragraphs, set line spacing, and more
- [`heading`] to set the appearance of headings and enable numbering
- [`document`] to set the metadata contained in the PDF output, such as title
  and author
</original>

并不是所有的函数参数都能被设置。总地来说，只有那些能够指导函数 _如何_ 去完成一件事情的参数可以被设置，而不包括函数完成事情所需的 _数据_ 参数。每个函数的参考页里都指出了哪些参数是可以设置的。

<original>
Not all function parameters can be set. In general, only parameters that tell
a function _how_ to do something can be set, not those that tell it _what_ to
do it with. The function reference pages indicate which parameters are settable.
</original>

现在让我们为文档添加更多的样式。我们希望边距更大，并且使用衬线的字体。为了展示例子，我们还会设置一个页面大小。

<original>
Let's add a few more styles to our document. We want larger margins and a serif
font. For the purposes of the example, we'll also set another page size.
</original>

```example
#set text(
  font: "New Computer Modern",
  size: 10pt
)
#set page(
  paper: "a6",
  margin: (x: 1.8cm, y: 1.5cm),
)
#set par(
  justify: true,
  leading: 0.52em,
)

= Introduction
In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behaviour of these natural structures.

>>> Glacier displacement is influenced
>>> by a number of factors, including
>>> + The climate
>>> + The topography
>>> + The geology
>>>
>>> This report will present a physical
>>> model of glacier displacement and
>>> dynamics, and will explore the
>>> influence of these factors on the
>>> movement of large bodies of ice.
<<< ...

#align(center + bottom)[
  #image("glacier.jpg", width: 70%)

  *Glaciers form an important
  part of the earth's climate
  system.*
]
```

有几件事需要注意。

第一个，是 [`page`] 这个 set rule。它接受两个参数：页面大小和边距。页面的大小事一个字符串，Typst 接受许多[标准的页面大小]($page.paper)，不过你也可以自定义。边距以[字典]($dictionary)的形式指定。字典是一组键值对（key-value pairs）。例子中的键是 `x` 和 `y`，值分别是水平和垂直的边距。我们也可以通过在字典中带上 `{left}`、`{right}`、`{top}` 和 `{bottom}` 来指定每个方向的边距。

<original>
There are a few things of note here.

First is the [`page`] set rule. It receives two arguments: the page size and
margins for the page. The page size is a string. Typst accepts [many standard
page sizes,]($page.paper) but you can also specify a custom page size. The
margins are specified as a [dictionary.]($dictionary) Dictionaries are a
collection of key-value pairs. In this case, the keys are `x` and `y`, and the
values are the horizontal and vertical margins, respectively. We could also have
specified separate margins for each side by passing a dictionary with the keys
`{left}`, `{right}`, `{top}`, and `{bottom}`.

</original>


然后是 [`text`] set rule。这里我们将字体大小设定为了 `{10pt}`，字体族为 `{"New Computer Modern"}`。Typst app 中包含了许多你可以在文档中尝试的字体。你在 text 函数的参数列表中输入时，可以通过自动补全面板查看可用的字体。

<original>
Next is the set [`text`] set rule. Here, we set the font size to `{10pt}` and
font family to `{"New Computer Modern"}`. The Typst app comes with many fonts
that you can try for your document. When you are in the text function's argument
list, you can discover the available fonts in the autocomplete panel.
</original>

我们还设置了行与行之间的间隙，即行距。它以一个 [length] 类型的值指定。在这里我们用到了 `em` 这个单位来相对于字体大小指定行距：`{1em}` 相当于当前的字体大小（默认为 `{11pt}`）。

<original>
We have also set the spacing between lines (a.k.a. leading): It is specified as
a [length] value, and we used the `em` unit to specify the leading relative to
the size of the font: `{1em}` is equivalent to the current font size (which
defaults to `{11pt}`).
</original>

最后，我们通过在居中设置中添加垂直的对齐，将图片居底。可通过 `{+}` 操作符来将垂直和水平的对齐结合在一起，最终产生一个二维对齐（2D alignment）。

<original>
Finally, we have bottom aligned our image by adding a vertical alignment to our
center alignment. Vertical and horizontal alignments can be combined with the
`{+}` operator to yield a 2D alignment.
</original>

## 一些经验的提示

为了让文档结构更为清晰，我们现在想要将标题标号。这可以通过设置 [`heading`] 函数的 `numbering` 参数来实现。

<original>
To structure our document more clearly, we now want to number our headings. We
can do this by setting the `numbering` parameter of the [`heading`] function.
</original>

```example
>>> #set text(font: "New Computer Modern")
#set heading(numbering: "1.")

= Introduction
#lorem(10)

== Background
#lorem(12)

== Methods
#lorem(15)
```

我们指定了 `{"1."}` 这一字符串作为标号参数，这告诉 Typst 用阿拉伯数字去标号并在标题和数字之间加上一个点。除此之外，还可以用[字母、罗马数字以及符号]($numbering)。

<original>
We specified the string `{"1."}` as the numbering parameter. This tells Typst to
number the headings with arabic numerals and to put a dot between the number of
each level. We can also use [letters, roman numerals, and symbols]($numbering)
for our headings:
</original>

```example
>>> #set text(font: "New Computer Modern")
#set heading(numbering: "1.a")

= Introduction
#lorem(10)

== Background
#lorem(12)

== Methods
#lorem(15)
```

这个例子中用到了 [`lorem`] 函数来生成占位文本。该函数接受一个数字参数，含有与该数字同等单词数的 _乱数假文_。

<original>
This example also uses the [`lorem`] function to generate some placeholder text.
This function takes a number as an argument and generates that many words of
_Lorem Ipsum_ text.
</original>

<div class="info-box">

你是否好奇为什么 headings 和 text 的 set rules 可以应用到所有的文本和标题，即使这些文本和标题并不是由其对应的函数生成的呢？

每次当你写下 `[= Conclusion]` 的时候，Typst 内部都会调用 `heading` 函数。实际上，`[#heading[Conclusion]]` 这一调用和前面的标题标记（译者注：即 `=` 后跟标题的格式）是等价的。其它的标记也类似，它们都只是对应函数调用的 _语法糖_。
</div>

<original>
<div class="info-box">

Did you wonder why the headings and text set rules apply to all text and headings,
even if they are not produced with the respective functions?

Typst internally calls the `heading` function every time you write
`[= Conclusion]`. In fact, the function call `[#heading[Conclusion]]` is
equivalent to the heading markup above. Other markup elements work similarly,
they are only _syntax sugar_ for the corresponding function calls.
</div>
</original>

## Show rules

你对目前的结果已经很满意了。但是还有一件事需要解决：你的报告是为一个更大的项目而编写的，即使是在简单陈述中，这个项目的名称应当随时和 Logo 一同出现，。

<original>
You are already pretty happy with how this turned out. But one last thing needs
to be fixed: The report you are writing is intended for a larger project and
that project's name should always be accompanied by a logo, even in prose.
</original>

你思考着一些做法。或许，你可以通过查找与替换在每个 Logo 文本出现的地方添加一个 `[#image("logo.svg")]` 调用。但这看起来十分冗余。反之，你还可以[自己定义一个函数]($function/#defining-functions)，使它总是返回 Logo 文本和图案。不过，有一个更简单的方法：

<original>
You consider your options. You could add an `[#image("logo.svg")]` call before
every instance of the logo using search and replace. That sounds very tedious.
Instead, you could maybe
[define a custom function]($function/#defining-functions) that always yields the
logo with its image. However, there is an even easier way:
</original>

利用 show rules，你可以重新定义 Typst 展示某些元素的方式。你可以指定哪些元素应该被特殊展示，以及相应的展示效果。Show rules 可以应用到文本实例，各种函数乃至整个文档。

<original>
With show rules, you can redefine how Typst displays certain elements. You
specify which elements Typst should show differently and how they should look.
Show rules can be applied to instances of text, many functions, and even the
whole document.
</original>

```example
#show "ArtosFlow": name => box[
  #box(image(
    "logo.svg",
    height: 0.7em,
  ))
  #name
]

This report is embedded in the
ArtosFlow project. ArtosFlow is a
project of the Artos Institute.
```

这个例子里有很多新的语法：我们写下 `{show}` 关键字并在其后跟上一个想要特殊展示的字符串，带一个冒号。然后编写了一个函数，以被展示内容作为参数。在这里，我们称该参数为 `name`。现在我们可以在函数体中用 `name` 这个参数来输出 ArtosFlow 这一名称。这里的 show rule 在名称的前面加上了 Logo 图标，并将最终结果放进了一个 box 里，从而避免图标与名称之间出现换行。为了避免独占一行，图标本身也被放在了 box 里。

<original>
There is a lot of new syntax in this example: We write the `{show}` keyword,
followed by a string of text we want to show differently and a colon. Then, we
write a function that takes the content that shall be shown as an argument.
Here, we called that argument `name`. We can now use the `name` variable in the
function's body to print the ArtosFlow name. Our show rule adds the logo image
in front of the name and puts the result into a box to prevent linebreaks from
occurring between logo and name. The image is also put inside of a box, so that
it does not appear in its own paragraph.
</original>

之所以不需要在第一个 box 函数和 image 函数的调用前面带 `#`，是因为它们没有直接嵌在标记（译者注：也就是前文所说的内容块）中。当 Typst 在识别代码而非标记的时候，函数、关键字以及变量前的 `#` 就不需要带了。这一点可以在参数列表、函数定义和[代码块]($scripting)中观察到。

<original>
The calls to the first box function and the image function did not require a
leading `#` because they were not embedded directly in markup. When Typst
expects code instead of markup, the leading `#` is not needed to access
functions, keywords, and variables. This can be observed in parameter lists,
function definitions, and [code blocks]($scripting).
</original>

## 回顾

现在你知道如何向你的 Typst 文档添加一些基础的格式了。你学会了如何设置字体、对齐段落、设置页面大小以及利用 set rules 为标题添加标号。你还了解到如何用基础的 set rule 来改变全文中文本的展示形式。

<original>
You now know how to apply basic formatting to your Typst documents. You learned
how to set the font, justify your paragraphs, change the page dimensions, and
add numbering to your headings with set rules. You also learned how to use a
basic show rule to change how text appears throughout your document.
</original>

你上交了你的报告。你的导师看了非常高兴，他们一行人想要将它改编成一篇会议论文（conference paper）！在下一章，我们会了解如何利用更高阶的 show rules 和函数，将你的文档排版成一篇论文。

<original>
You have handed in your report. Your supervisor was so happy with it that they
want to adapt it into a conference paper! In the next section, we will learn how
to format your document as a paper using more advanced show rules and functions.
</original>
