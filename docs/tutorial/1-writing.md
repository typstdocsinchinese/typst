---
en_title: Writing in Typst
description: Typst's tutorial.
---

# 使用 Typst 编写文档

让我们开始吧！

假设你的大学布置你编写一篇技术报告，这里面会包含文段、数学公式、大小标题以及各种图形。开始时，你在 Typst app 里创建了一个新的项目，接下来页面跳转到编辑器里，在这里你可以看到两个栏目：一个是用来编写你的文档的源代码栏（source panel），另一个是用来查看渲染后文档的预览栏（preview panel）。

<original>
Let's get started! Suppose you got assigned to write a technical report for
university. It will contain prose, maths, headings, and figures. To get started,
you create a new project on the Typst app. You'll be taken to the editor where
you see two panels: A source panel where you compose your document and a
preview panel where you see the rendered document.
</original>

![Typst app screenshot](1-writing-app.png)

现在你的头脑中对这一报告已经有了不错的构思，让我们从编写介绍（Introduction）部分开始。在编辑器栏里输入一些文本，你会发现它们立即出现在了预览页中。

<original>
You already have a good angle for your report in mind. So let's start by writing
the introduction. Enter some text in the editor panel. You'll notice that the
text immediately appears on the previewed page.
</original>

```example
In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behaviour of these natural structures.
```

_在整篇教程中，我们都会按照上面的方式展示代码示例。这里如同 app 中的那样，左栏里写内容，右栏展示渲染效果的预览。_

<original>
_Throughout this tutorial, we'll show code examples like this one. Just like in the app, the first panel contains markup and the second panel shows a preview. [Not translated ->] We shrunk the page to fit the examples so you can see what's going on._
</original>

下一步是添加一些标题以及为一部分文本添加强调的效果。Typst 对于最为常见的格式采用了简单的标记，添加标题可使用 `=` 符号，用斜体来强调时，将文本用 `[_下划线_]` 包围即可。

<original>
The next step is to add a heading and emphasize some text. Typst uses simple
markup for the most common formatting tasks. To add a heading, enter the `=`
character and to emphasize some text with italics, enclose it in
`[_underscores_]`.
</original>

```example
= Introduction
In this report, we will explore the
various factors that influence _fluid
dynamics_ in glaciers and how they
contribute to the formation and
behaviour of these natural structures.
```

很简单吧！

要新增段落，只需在两行文本之间加入一个空行。如果段落需要子标题，可用 `==`（而非 `=`）来生成。`=` 的数量决定了这些标题的嵌套层级。

<original>
That was easy! To add a new paragraph, just add a blank line in between two
lines of text. If that paragraph needs a subheading, produce it by typing `==`
instead of `=`. The number of `=` characters determines the nesting level of the
heading.
</original>

现在，我们想要列出一些影响冰川的动力学的例子。为了达成这一效果，我们会用到有序列表（numbered list，标号列表）。有序列表的每个项目前加的是 `+`，Typst 会自动地将它们按顺序编号。

<original>
Now we want to list a few of the circumstances that influence glacier dynamics.
To do that, we use a numbered list. For each item of the list, we type a `+`
character at the beginning of the line. Typst will automatically number the
items.
</original>

```example
+ The climate
+ The topography
+ The geology
```

如果我们想要添加无序列表（bulleted list，带点的列表），可使用 `-` 替代 `+`。 列表是可以嵌套的，通过缩进（indenting）为上面列表的第一项添加子列表的示例如下：

<original>
If we wanted to add a bulleted list, we would use the `-` character instead of
the `+` character. We can also nest lists: For example, we can add a sub-list to
the first item of the list above by indenting it.
</original>

```example
+ The climate
  - Temperature
  - Precipitation
+ The topography
+ The geology
```

## 添加图片

你觉得在你的报告里加上一些图表会更好。现在我们来试着加一个。Typst 支持 PNG、JPEG、GIF 和 SVG 格式的图片。要在项目中新增一个图片文件，首先点击 app 页面左侧的箱形图表打开 _文件栏_。在这里你可以看到项目中的所有文件。目前，这里只有一个文件，也就是你正编写的这个 Typst 文件。要上传文件，点击右上角的箭头图标，这会打开一个上传对话框，你可以在这里挑选要从你电脑上传的一个或多个文件。

<original>
You think that your report would benefit from a figure. Let's add one. Typst
supports images in the formats PNG, JPEG, GIF, and SVG. To add an image file to
your project, first open the _file panel_ by clicking the box icon in the left
sidebar. Here, you can see a list of all files in your project. Currently, there
is only one: The main Typst file you are writing in. To upload another file,
click the button with the arrow in the top-right corner. This opens the upload
dialog, in which you can pick files to upload from your computer. Select an
image file for your report.
</original>

![Upload dialog](1-writing-upload.png)

前面我们已经知晓，Typst 中的某些符号（称为 _标记_）具有特殊的含义。我们分别用 `=`、`-`、`+` 和 `_` 创建标题、列表（无序、有序）和强调的文本。然而，要是真的为所有我们将来想要插入到文档中的元素都单独设置一个特殊符号，使用起来就会变得既晦涩又臃肿。因此，Typst 只为那些最常见的格式保留了标记符号，其余的则使用 _函数_ 来插入。对于我们要展示的图片，我们会用到 Typst 的 [`image`] 函数。

<original>
We have seen before that specific symbols (called _markup_) have specific
meaning in Typst. We can use `=`, `-`, `+`, and `_` to create headings, lists
and emphasized text, respectively. However, having a special symbol for
everything we want to insert into our document would soon become cryptic and
unwieldy. For this reason, Typst reserves markup symbols only for the most
common things. Everything else is inserted with _functions._ For our image to
show up on the page, we use Typst's [`image`] function.
</original>

```example
#image("glacier.jpg")
```

一言蔽之，函数用于根据一组参数产生输出。当你在标记中 _调用_ 一个函数并传入参数后，Typst 就将结果（即函数的 _返回值_）插入到文档中。在我们的这个例子中，`image` 函数接受一个参数：图片文件的路径。要在标记中调用函数，我们使用 `#函数名` 的格式，后跟圆括号（parentheses）括起来的参数列表，参数可以有不同的数据类型。这里的文件路径是一个短的[字符串]($str)，所以我们应该将它的值用双引号包围。

<original>
In general, a function produces some output for a set of _arguments_. When you
_call_ a function within markup, you provide the arguments and Typst inserts the
result (the function's _return value_) into the document. In our case, the
`image` function takes one argument: The path to the image file. To call a
function in markup, we first need to type the `#` character, immediately
followed by the name of the function. Then, we enclose the arguments in
parentheses. Typst recognizes many different data types within argument lists.
Our file path is a short [string of text]($str), so we need to enclose it in
double quotes.
</original>

插入的图片（默认）会占据页面的全部宽度。要改变这一点，将 `width`（宽度）参数传递到 `image` 函数中。这是一个 _具名的_ 参数（named argument），以后会用 `name: value` 这样的格式来指代。多个参数用逗号（英文逗号 `,`）来分隔，所以在这里首先我们要在路径值后面输入一个逗号，然后跟上 `width: {70%}`。

<original>
The inserted image uses the whole width of the page. To change that, pass the
`width` argument to the `image` function. This is a _named_ argument and
therefore specified as a `name: value` pair. If there are multiple arguments,
they are separated by commas, so we first need to put a comma behind the path.
</original>

```example
#image("glacier.jpg", width: 70%)
```

`width` 参数是一个 [相对长度]($relative)，在上面的例子里，我们指定的是一个百分比，它决定了这个图片应当占据页面 `{70%}` 的宽度。相对长度也可以指定为一个绝对值，例如 `{1cm}` 或者 `{0.7in}`。

<original>
The `width` argument is a [relative length]($relative). In our case, we
specified a percentage, determining that the image shall take up `{70%}` of the
page's width. We also could have specified an absolute value like `{1cm}` or
`{0.7in}`.
</original>

和文本一样，默认情况下我们的图片是左对齐于页面的。严格来说，它现在还缺少说明文字（caption），让我们用 [figure] 函数来修复这一点。这个函数接受一个有序参数（positional argument），它指定了该 figure 的内容；和一个可选的具名参数，它指定了该 figure 的说明文本（caption）。

<original>
Just like text, the image is now aligned at the left side of the page by
default. It's also lacking a caption. Let's fix that by using the [figure]
function. This function takes the figure's contents as a positional argument and
an optional caption as a named argument.
</original>

在 `figure` 函数的参数列表中编辑时，Typst 已经处于代码模式中，这代表你在调用图片函数时应该去掉前面的井号。井号只需要在直接标记中加入，这是为了区分一般的文本与函数。

<original>
Within the argument list of the `figure` function, Typst is already in code
mode. This means, you now have to remove the hash before the image function call.
The hash is only needed directly in markup (to disambiguate text from function
calls).
</original>

图片的说明文本可由任意的标记组成。为了向函数传递标记，我们将它用方括号包围，这种结构称为 _内容块_。

<original>
The caption consists of arbitrary markup. To give markup to a function, we
enclose it in square brackets. This construct is called a _content block._
</original>

```example
#figure(
  image("glacier.jpg", width: 70%),
  caption: [
    _Glaciers_ form an important part
    of the earth's climate system.
  ],
)
```

你继续编写着报告，发现需要在某处引用这张图表。要达到这一效果，首先要为图表加一个标签（label），用于赋予文档中的元素一个唯一的标识。要添加标签，用尖括号包围标签的内容，然后加在元素（例如这里的图表）后边。接下来你可以在文本中用 `[@]标签名` 来引用这张图表。标题和公式也可以像这样打上标签来引用。

<original>
You continue to write your report and now want to reference the figure. To do
that, first attach a label to figure. A label uniquely identifies an element in
your document. Add one after the figure by enclosing some name in angle
brackets. You can then reference the figure in your text by writing an `[@]`
symbol followed by that name. Headings and equations can also be labelled to
make them referenceable.
</original>

```example
Glaciers as the one shown in
@glaciers will cease to exist if
we don't take action soon!

#figure(
  image("glacier.jpg", width: 70%),
  caption: [
    _Glaciers_ form an important part
    of the earth's climate system.
  ],
) <glaciers>
```

<div class="info-box">

截至目前，我们向函数传递了内容块（用方括号包围的标记）和字符串（用双引号包围的文本）。两者似乎都包含着文本。有什么区别呢？

内容块既可以包含文本，也可以包含任意的标记、函数调用等，而字符串仅仅是一串字符，别无他意。

例如，图片函数需要图片文件的路径，这时传入一段文字或者其它图片（译者注：即 `image` 函数的返回值）进去是没有意义的。这就是为什么路径参数里只能填字符串。相反，字符串可以传入任何内容类的参数里，因为单纯的文本就是一种有效的内容（块）。
</div>

<original>
<div class="info-box">

So far, we've passed content blocks (markup in square brackets) and strings
(text in double quotes) to our functions. Both seem to contain text. What's the
difference?

A content block can contain text, but also any other kind of markup, function
calls, and more, whereas a string is really just a _sequence of characters_ and
nothing else.

For example, the image function expects a path to an image file.
It would not make sense to pass, e.g., a paragraph of text or another image as
the image's path parameter. That's why only strings are allowed here.
On the contrary, strings work wherever content is expected because text is a
valid kind of content.
</div>
</original>

## 添加参考书目

随着报告的不断完善，你需要为你的一些阐述做支撑。你可以用 [`bibliography`] 函数为你的文档添加参考书目，这一函数接受一个指向书目文件（bibliography file）的路径。

<original>
As you write up your report, you need to back up some of your claims. You can
add a bibliography to your document with the [`bibliography`] function. This
function expects a path to a bibliography file.
</original>

Typst 中原生的引用格式是 [Hayagriva](https://github.com/typst/hayagriva/blob/main/docs/file-format.md) 但是为了兼容性你也可以使用 BibLaTeX 文件。现在你的同学已经完成了文献综述（literature survey）流程并发给你了一个 `.bib` 格式的文件，你将采用这个文件，于是你将它上传到了 Typst 中。

<original>
Typst's native bibliography format is
[Hayagriva](https://github.com/typst/hayagriva/blob/main/docs/file-format.md),
but for compatibility you can also use BibLaTeX files. As your classmate has
already done a literature survey and sent you a `.bib` file, you'll use that
one. Upload the file through the file panel to access it in Typst.
</original>

当文档中包含这样的书目时，你便可以开始从中进行引用。引用与标签采用同样的语法（译者注：`[@]` 后跟引用内容）。在你引用了来源后，它就会自动出现在文档的引用书目部分。

Typst 支持不同的引用与书目风格。更多细节请[参考这里]($bibliography.style)。

<original>
Once the document contains a bibliography, you can start citing from it.
Citations use the same syntax as references to a label. As soon as you cite a
source for the first time, it will appear in the bibliography section of your
document. Typst supports different citation and bibliography styles. Consult the
[reference]($bibliography.style) for more details.
</original>

```example
= Methods
We follow the glacier melting models
established in @glacier-melt.

#bibliography("works.bib")
```

## 数学公式

完善了 Methods 部分以后，你开始解决文档的又一核心——公式。Typst 有着内置的数学排版，并采用自有的数学记号，让我们从一个简单的公式开始。

我们用 `[$]` 包围公式。

<original>
After fleshing out the methods section, you move on to the meat of the document:
Your equations. Typst has built-in mathematical typesetting and uses its own
math notation. Let's start with a simple equation. We wrap it in `[$]` signs
to let Typst know it should expect a mathematical expression:
</original>

```example
The equation $Q = rho A v + C$
defines the glacial flow rate.
```

这一公式是在行内排版的。如果想要它另起一行，就在公式的开头和末尾加入一个空格。

<original>
The equation is typeset inline, on the same line as the surrounding text. If you
want to have it on its own line instead, you should insert a single space at its
start and end:
</original>

```example
The flow rate of a glacier is
defined by the following equation:

$ Q = rho A v + C $
```

我们可以发现 Typst 原样显示了 `Q`、`A`、`v` 和 `c` 这些字母，而将 `rho` 翻译成了希腊字母。数学模式总会原样地展示单个字母，多个字母默认会被解析为符号、变量或者函数名称。（因此，）要表示字母的乘法，请在字母之间插入空格。

<original>
We can see that Typst displayed the single letters `Q`, `A`, `v`, and `C` as-is,
while it translated `rho` into a Greek letter. Math mode will always show single
letters verbatim. Multiple letters, however, are interpreted as symbols,
variables, or function names. To imply a multiplication between single letters,
put spaces between them.
</original>

如果你想要展示一个名称含多个字母的数学变量，你可以用引号将其包围：

<original>
If you want to have a variable that consists of multiple letters, you can
enclose it in quotes:
</original>

```example
The flow rate of a glacier is given
by the following equation:

$ Q = rho A v + "time offset" $
```

你发现在论文里还需要一个求和公式。我们可以用 `sum` 记号，然后用上下标来指定求和的范围：

<original>
You'll also need a sum formula in your paper. We can use the `sum` symbol and
then specify the range of the summation in sub- and superscripts:
</original>

```example
Total displaced soil by glacial flow:

$ 7.32 beta +
  sum_(i=0)^nabla Q_i / 2 $
```

要向数学变量或符号添加下标，输入 `_` 紧跟下标的内容。类似地，上标用 `^` 来表示。如果你要输入的下标和上标包含多个元素，则必须用圆括号将它们包围。

<original>
To add a subscript to a symbol or variable, type a `_` character and then the
subscript. Similarly, use the `^` character for a superscript. If your
sub- or superscript consists of multiple things, you must enclose them
in round parentheses.
</original>

上面的例子还向我们展示了如何插入分式：只需在分子和分母之间放入一个 `/`，Typst 就会自动地将它变成分式。

Typst 中的括号是自动解析的，因此你可以按照你想要的方式将表达式输入到计算器里进行计算，再复制粘贴到 Typst 中。无需修改，Typst 会自动将带括号的子表达式解析替换为恰当的结果。

<original>
The above example also showed us how to insert fractions: Simply put a `/`
character between the numerator and the denominator and Typst will automatically
turn it into a fraction. Parentheses are smartly resolved, so you can enter your
expression as you would into a calculator and Typst will replace parenthesized
sub-expressions with the appropriate notation.
</original>

```example
Total displaced soil by glacial flow:

$ 7.32 beta +
  sum_(i=0)^nabla
    (Q_i (a_i - epsilon)) / 2 $
```

并不是所有的数学结构都有特殊的语法（译者注：这里特殊的语法是指像上标 `^` 下标 `_` 这样的特殊简易记号）。对于其它结果，我们会利用函数来表达，就像我们前面所见的 `image` 函数一样。例如，要插入一个列向量，我们可以使用 [`vec`]($math.vec) 函数。

在数学模式下，函数调用也不需要以 `#` 开头。

<original>
Not all math constructs have special syntax. Instead, we use functions, just
like the `image` function we have seen before. For example, to insert a column
vector, we can use the [`vec`]($math.vec) function. Within math mode, function
calls don't need to start with the `#` character.
</original>

```example
$ v := vec(x_1, x_2, x_3) $
```

有些函数只在数学模式下存在。例如 [`cal`]($math.cal) 函数用于显示出常用于表示集合的书法字体。

[References 中的 math 部分]($category/math)列出了所有数学模式下可用的函数，可供参考。

<original>
Some functions are only available within math mode. For example, the
[`cal`]($math.cal) function is used to typeset calligraphic letters commonly
used for sets. The [math section of the reference]($category/math) provides a
complete list of all functions that math mode makes available.
</original>

还有一件事：许多符号有不同的变种，比如箭头。你可以通过在这一符号的名称后面加上点（`.`）和修饰名来选择一个变种：

<original>
One more thing: Many symbols, such as the arrow, have a lot of variants. You can
select among these variants by appending a dot and a modifier name to a symbol's
name:
</original>

```example
$ a arrow.squiggly b $
```

这个记号在标记模式下也同样可用，但此时的调用必须以 `#sym.` 开头。转到 [symbols 部分]($category/symbols/sym)来查看完整的可用符号列表。

<original>
This notation is also available in markup mode, but the symbol name must be
preceded with `#sym.` there. See the [symbols section]($category/symbols/sym)
for a list of all available symbols.
</original>

## 回顾

现在你已经知道如何用 Typst 编写一个基本的文档了。在本章中，我们了解了如何去用斜体强调文本、编写列表、插入图片、对齐内容以及排版数学表达式，还了解了 Typst 中函数的内容。除了图片，Typst 中有很多其它类型的内容可供你插入到文档中，例如[表格]($table)、[图形]($category/visualize)和[代码块]($raw)。你可以研读 [reference] 来了解更多特性。

<original>
You have now seen how to write a basic document in Typst. You learned how to
emphasize text, write lists, insert images, align content, and typeset
mathematical expressions. You also learned about Typst's functions. There are
many more kinds of content that Typst lets you insert into your document, such
as [tables]($table), [shapes]($category/visualize), and [code blocks]($raw). You
can peruse the [reference] to learn more about these and other features.
</original>

到目前为止，你已经完成了你的报告。通过点击页面右上角的下载按钮，你保存了一份 PDF 文件。然而，你认为这个报告可能看上去有些过于平淡。在下一章，我们将了解如何自定义文档的外观。

<original>
For the moment, you have completed writing your report. You have already saved a
PDF by clicking on the download button in the top right corner. However, you
think the report could look a bit less plain. In the next section, we'll learn
how to customize the look of our document.
</original>

