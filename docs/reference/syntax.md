---
en_title: Syntax
description: |
  A compact reference for Typst's syntax. Learn more about the language within
  markup, math, and code mode.
---

# 语法

Typst 是一门标记语言，这意味着你可以用一些简单语法来完成基础的布局操作。这种轻量的标记语法是用 set rules 与 show rules
来实现的，你可以用它们来轻松、灵活地定义文档的样式。而这一切都基于内部紧密集成的一种脚本语言，它提供了许多有用的内置函数，用户亦可以自行编写函数。

<original>
Typst is a markup language. This means that you can use simple syntax to
accomplish common layout tasks. The lightweight markup syntax is complemented by
set and show rules, which let you style your document easily and automatically.
All this is backed by a tightly integrated scripting language with built-in and
user-defined functions.
</original>

## 句法模式

Typst 中存在三种句法模式：markup（标记）、math（数学）和 code（代码）。markup 模式是文档的默认句法模式，在 math 模式中可写数学公式，code
模式中可调用 Typst 的各种脚本特性。

<original>
Typst has three syntactical modes: Markup, math, and code. Markup mode is the
default in a Typst document, math mode lets you write mathematical formulas, and
code mode lets you use Typst's scripting features.
</original>

参考下表中的内容，你可以随时随地切换到特定的模式中：

<original>
You can switch to a specific mode at any point by referring to the following
table:
</original>

| 句法模式   | 语法              | 例子                              |
|--------|-----------------|---------------------------------|
| Code   | 在具体代码前加 `#`     | `[Number: #(1 + 2)]`            |
| Math   | 用 `[$..$]` 包围式子 | `[$-x$ is the opposite of $x$]` |
| Markup | 用 `[[..]]` 包围内容 | `{let name = [*Typst!*]}`       |

<original>
| New mode | Syntax                          | Example                         |
|----------|---------------------------------|---------------------------------|
| Code     | Prefix the code with `#`        | `[Number: #(1 + 2)]`            |
| Math     | Surround equation with `[$..$]` | `[$-x$ is the opposite of $x$]` |
| Markup   | Surround markup with `[[..]]`   | `{let name = [*Typst!*]}`       |
</original>

用 `#` 进入 code 模式以后，只要没有切换回 markup 或者 math 模式，就不需要再使用任何井号。

<original>
Once you have entered code mode with `#`, you don't need to use further hashes
unless you switched back to markup or math mode in between.
</original>

## markup 模式

Typst 为最常见的文档元素提供了内置的标记支持。这些语法大多数是对应函数的简写。下表中列出了所有可用的标记，并配有相关链接。这些链接指向的是了解相关语法用法的最佳参考。

<original>
Typst provides built-in markup for the most common document elements. Most of
the syntax elements are just shortcuts for a corresponding function. The table
below lists all markup that is available and links to the best place to learn
more about their syntax and usage.
</original>

| 名称    | 示例                           | 参见                              |
|-------|------------------------------|---------------------------------|
| 分段    | 空行                           | [`parbreak`]                    |
| 加粗强调  | `[*strong*]`                 | [`strong`]                      |
| 斜体强调  | `[_emphasis_]`               | [`emph`]                        |
| 原始文本  | ``[`print(1)`]``             | [`raw`]                         |
| 链接    | `[https://typst.app/]`       | [`link`]                        |
| 标签    | `[<intro>]`                  | [`label`]                       |
| 参考    | `[@intro]`                   | [`ref`]                         |
| 标题    | `[= Heading]`                | [`heading`]                     |
| 无序列表  | `[- item]`                   | [`list`]                        |
| 有序列表  | `[+ item]`                   | [`enum`]                        |
| 术语表   | `[/ Term: description]`      | [`terms`]                       |
| 数学公式  | `[$x^2$]`                    | [数学公式]($category/math)          |
| 换行    | `[\]`                        | [`linebreak`]                   |
| 智能引用  | `['single' or "double"]`     | [`smartquote`]                  |
| 符号简写  | `[~]`, `[---]`               | [符号]($category/symbols/sym)     |
| 代码表达式 | `[#rect(width: 1cm)]`        | [脚本编写]($scripting/#expressions) |
| 转义字符  | `[Tweet at us \#ad]`         | [见下文](#转义序列)                 |
| 注释    | `[/* block */]`, `[// line]` | [见下文](#注释)                |

<original>
| Name             | Example                      | See                                  |
|------------------|------------------------------|--------------------------------------|
| Paragraph break  | Blank line                   | [`parbreak`]                         |
| Strong emphasis  | `[*strong*]`                 | [`strong`]                           |
| Emphasis         | `[_emphasis_]`               | [`emph`]                             |
| Raw text         | ``[`print(1)`]``             | [`raw`]                              |
| Link             | `[https://typst.app/]`       | [`link`]                             |
| Label            | `[<intro>]`                  | [`label`]                            |
| Reference        | `[@intro]`                   | [`ref`]                              |
| Heading          | `[= Heading]`                | [`heading`]                          |
| Bullet list      | `[- item]`                   | [`list`]                             |
| Numbered list    | `[+ item]`                   | [`enum`]                             |
| Term list        | `[/ Term: description]`      | [`terms`]                            |
| Math             | `[$x^2$]`                    | [Math]($category/math)               |
| Line break       | `[\]`                        | [`linebreak`]                        |
| Smart quote      | `['single' or "double"]`     | [`smartquote`]                       |
| Symbol shorthand | `[~]`, `[---]`               | [Symbols]($category/symbols/sym)     |
| Code expression  | `[#rect(width: 1cm)]`        | [Scripting]($scripting/#expressions) |
| Character escape | `[Tweet at us \#ad]`         | [Below](#escapes)                    |
| Comment          | `[/* block */]`, `[// line]` | [Below](#comments)                   |
</original>

## math 模式

math 模式是一种用来排版数学公式的特殊标记模式，可通过将等式用 `[$]` 包围来进入，markup 和 code
模式中都可以这样用。如果 `[$]` 包围内容的开头和结尾都有至少一个空格，该公式会被排版到单独的块中，如果没有则为行内公式。math
模式下的特殊语法可参考下表：

<original>
Math mode is a special markup mode that is used to typeset mathematical
formulas. It is entered by wrapping an equation in `[$]` characters. This works
both in markup and code. The equation will be typeset into its own block if it
starts and ends with at least one space (e.g. `[$ x^2 $]`). Inline math can be
produced by omitting the whitespace (e.g. `[$x^2$]`). An overview over the
syntax specific to math mode follows:
</original>

| 名称     | 示例                      | 参见                                |
|--------|-------------------------|-----------------------------------|
| 行内公式   | `[$x^2$]`               | [数学公式]($category/math)            |
| 块级公式   | `[$ x^2 $]`             | [数学公式]($category/math)            |
| 下标     | `[$x_1$]`               | [`attach`]($category/math/attach) |
| 上标     | `[$x^2$]`               | [`attach`]($category/math/attach) |
| 分式     | `[$1 + (a+b)/5$]`       | [`frac`]($math.frac)              |
| 换行     | `[$x \ y$]`             | [`linebreak`]                     |
| 对齐点    | `[$x &= 2 \ &= 3$]`     | [数学公式]($category/math)            |
| 访问变量   | `[$#x$, $pi$]`          | [数学公式]($category/math)            |
| 访问字段   | `[$arrow.r.long$]`      | [脚本编写]($scripting/#fields)        |
| 乘法     | `[$x y$]`               | [数学公式]($category/math)            |
| 符号简写   | `[$->$]`, `[$!=$]`      | [符号]($category/symbols/sym)       |
| 文本     | `[$a "is natural"$]`    | [数学公式]($category/math)            |
| 调用数学函数 | `[$floor(x)$]`          | [数学公式]($category/math)            |
| 代码表达式  | `[$#rect(width: 1cm)$]` | [脚本编写]($scripting/#expressions)   |
| 转义     | `[$x\^2$]`              | [见下文](#转义序列)                   |
| 注释     | `[$/* comment */$]`     | [见下文](#注释)                  |

<original>
| Name                   | Example                 | See                                  |
|------------------------|-------------------------|--------------------------------------|
| Inline math            | `[$x^2$]`               | [Math]($category/math)               |
| Block-level math       | `[$ x^2 $]`             | [Math]($category/math)               |
| Bottom attachment      | `[$x_1$]`               | [`attach`]($category/math/attach)    |
| Top attachment         | `[$x^2$]`               | [`attach`]($category/math/attach)    |
| Fraction               | `[$1 + (a+b)/5$]`       | [`frac`]($math.frac)                 |
| Line break             | `[$x \ y$]`             | [`linebreak`]                        |
| Alignment point        | `[$x &= 2 \ &= 3$]`     | [Math]($category/math)               |
| Variable access        | `[$#x$, $pi$]`          | [Math]($category/math)               |
| Field access           | `[$arrow.r.long$]`      | [Scripting]($scripting/#fields)      |
| Implied multiplication | `[$x y$]`               | [Math]($category/math)               |
| Symbol shorthand       | `[$->$]`, `[$!=$]`      | [Symbols]($category/symbols/sym)     |
| Text/string in math    | `[$a "is natural"$]`    | [Math]($category/math)               |
| Math function call     | `[$floor(x)$]`          | [Math]($category/math)               |
| Code expression        | `[$#rect(width: 1cm)$]` | [Scripting]($scripting/#expressions) |
| Character escape       | `[$x\^2$]`              | [Below](#escapes)                    |
| Comment                | `[$/* comment */$]`     | [Below](#comments)                   |
</original>

## code 模式

在具体的代码块和表达式中，新写的表达式可以不用带 `#`。有许多句法元素是表达式专有的。下标列出了 code 模式中可用的所有语法：

<original>
Within code blocks and expressions, new expressions can start without a leading
`#` character. Many syntactic elements are specific to expressions. Below is
a table listing all syntax that is available in code mode:
</original>

| 名称                   | 示例                              | 参见                               |
|----------------------|---------------------------------|----------------------------------|
| 空值（None）             | `{none}`                        | [`none`]                         |
| 自动值（Auto）            | `{auto}`                        | [`auto`]                         |
| 布尔值                  | `{false}`, `{true}`             | [`bool`]                         |
| 整型                   | `{10}`, `{0xff}`                | [`int`]                          |
| 浮点数                  | `{3.14}`, `{1e5}`               | [`float`]                        |
| 长度                   | `{2pt}`, `{3mm}`, `{1em}`, ..   | [`length`]                       |
| 角度                   | `{90deg}`, `{1rad}`             | [`angle`]                        |
| 分度                   | `{2fr}`                         | [`fraction`]                     |
| 比例                   | `{50%}`                         | [`ratio`]                        |
| 字符串                  | `{"hello"}`                     | [`str`]                          |
| 标签                   | `{<intro>}`                     | [`label`]                        |
| 数学公式                 | `[$x^2$]`                       | [数学公式]($category/math)           |
| 原始文本                 | ``[`print(1)`]``                | [`raw`]                          |
| 访问变量                 | `{x}`                           | [脚本编写]($scripting/#blocks)       |
| 代码块                  | `{{ let x = 1; x + 2 }}`        | [脚本编写]($scripting/#blocks)       |
| 内容块                  | `{[*Hello*]}`                   | [脚本编写]($scripting/#blocks)       |
| 括号表达式                | `{(1 + 2)}`                     | [脚本编写]($scripting/#blocks)       |
| 数组                   | `{(1, 2, 3)}`                   | [数组]($array)                     |
| 字典                   | `{(a: "hi", b: 2)}`             | [字典]($dictionary)                |
| 一元运算符                | `{-x}`                          | [脚本编写]($scripting/#operators)    |
| 二元运算符                | `{x + y}`                       | [脚本编写]($scripting/#operators)    |
| 赋值                   | `{x = 1}`                       | [脚本编写]($scripting/#operators)    |
| 字段访问                 | `{x.y}`                         | [脚本编写]($scripting/#fields)       |
| 方法调用                 | `{x.flatten()}`                 | [脚本编写]($scripting/#methods)      |
| 函数调用                 | `{min(x, y)}`                   | [函数]($function)                  |
| 参数展开                 | `{min(..nums)}`                 | [参数]($arguments)                 |
| 无名（unnamed）函数        | `{(x, y) => x + y}`             | [函数]($function)                  |
| let 绑定               | `{let x = 1}`                   | [脚本编写]($scripting/#bindings)     |
| 具名函数                 | `{let f(x) = 2 * x}`            | [函数]($function)                  |
| set rule             | `{set text(14pt)}`              | [样式定义]($styling/#set-rules)      |
| set-if rule          | `{set text(..) if .. }`         | [样式定义]($styling/#set-rules)      |
| show-set rule        | `{show heading: set block(..)}` | [样式定义]($styling/#show-rules)     |
| 带函数的 show rule       | `{show raw: it => {..}}`        | [样式定义]($styling/#show-rules)     |
| show-everything rule | `{show: template}`              | [样式定义]($styling/#show-rules)     |
| 上下文表达式               | `{context text.lang}`           | [上下文]($context)                  |
| 条件                   | `{if x == 1 {..} else {..}}`    | [脚本编写]($scripting/#conditionals) |
| for 循环               | `{for x in (1, 2, 3) {..}}`     | [脚本编写]($scripting/#loops)        |
| while 循环             | `{while x < 10 {..}}`           | [脚本编写]($scripting/#loops)        |
| 循环控制语句               | `{break, continue}`             | [脚本编写]($scripting/#loops)        |
| 函数返回                 | `{return x}`                    | [函数]($function)                  |
| 引入（include）模块        | `{include "bar.typ"}`           | [脚本编写]($scripting/#modules)      |
| 导入（import）模块         | `{import "bar.typ"}`            | [脚本编写]($scripting/#modules)      |
| 从模块中导入项目             | `{import "bar.typ": a, b, c}`   | [脚本编写]($scripting/#modules)      |
| 注释                   | `{/* block */}`, `{// line}`    | [见下文](#注释)                 |

## 注释

Typst 会忽略掉注释，它不会在最终输出中显示。这在需要去掉旧版本或者添加注解的时候很有用。要注释单行，在行前面加上 `//`。

<original>
Comments are ignored by Typst and will not be included in the output. This is
useful to exclude old versions or to add annotations. To comment out a single
line, start it with `//`:
</original>

```example
// our data barely supports
// this claim

We show with $p < 0.05$
that the difference is
significant.
```

注释也可以用 `/*` 和 `*/` 包围。下面例子中的注释可以延伸数行：

<original>
Comments can also be wrapped between `/*` and `*/`. In this case, the comment
can span over multiple lines:
</original>

```example
Our study design is as follows:
/* Somebody write this up:
   - 1000 participants.
   - 2x2 data design. */
```

## 转义序列

转义序列用于在 Typst 插入那些难以输入或者具有特殊含义的字符。要转义一个字符，在它前面加上一个反斜杠 `\`。如需插入 Unicode codepoint，可用十六进制转义序列实现：`[\u{1f600}]`。这种转义序列写法在[字符串]($str)里同样适用。

<original>
Escape sequences are used to insert special characters that are hard to type or
otherwise have special meaning in Typst. To escape a character, precede it with
a backslash. To insert any Unicode codepoint, you can write a hexadecimal escape
sequence: `[\u{1f600}]`. The same kind of escape sequences also work in
[strings]($str).
</original>

```example
I got an ice cream for
\$1.50! \u{1f600}
```

## 路径

Typst 中有许多特性需要借由文件路径来引用外部的资源，例如图片、typ 文件或者其他数据。路径以[字符串]($str)表示，分为两种：相对路径和绝对路径。

<original>
Typst has various features that require a file path to reference external
resources such as images, Typst files, or data files. Paths are represented as
[strings]($str). There are two kinds of paths: Relative and absolute.
</original>

- **相对路径**从其所在文件的位置开始搜索
  ```typ
  #image("images/logo.png")
  ```
- **绝对路径**从项目的 _根目录_ 进行搜索，以 `/` 开头。
  ```typ
  #image("/assets/logo.png")
  ```

<original>
- A **relative path** searches from the location of the Typst file where the
  feature is invoked. It is the default:
  ```typ
  #image("images/logo.png")
  ```
- An **absolute path** searches from the _root_ of the project. It starts with a
  leading `/`:
  ```typ
  #image("/assets/logo.png")
  ```
</original>

### 项目根目录

默认情况下，项目根目录是主 Typst 文件的父级目录。出于安全考虑，不允许读取根目录以外的任何文件。

<original>
By default, the project root is the parent directory of the main Typst file.
For security reasons, you cannot read any files outside of the root directory.
</original>

如果你想自定义项目根目录，可以在命令行中用 `--root` 这个 flag 来指定。请务必确保主文件是包含在你定义的根目录中的。

<original>
If you want to set a specific folder as the root of your project, you can use
the CLI's `--root` flag. Make sure that the main file is contained in the
folder's subtree!
</original>

```bash
typst compile --root .. file.typ
```

在 web app 中，项目本身就是根目录，你可以随时读取其中的文件，无论它们是否在编辑器里打开（即点击 Typst 文件旁的眼睛按钮的操作）。

<original>
In the web app, the project itself is the root directory. You can always read
all files within it, no matter which one is previewed (via the eye toggle next
to each Typst file in the file panel).
</original>

### 路径与包

包只能加载存在于它自身目录中的文件。在包中，绝对路径指向的是包的根目录，而不是项目的根目录。因此，它们不能直接加载项目目录里的文件。如果包需要项目中的资源（例如 logo 图片），则必须传入已经加载的图片，例如作为具名参数 `{logo: image("mylogo.svg")}`。你仍然可以在包中用 set rule 自定义图片的外观。

<original>
A package can only load files from its own directory. Within it, absolute paths
point to the package root, rather than the project root. For this reason, it
cannot directly load files from the project directory. If a package needs
resources from the project (such as a logo image), you must pass the already
loaded image, e.g. as a named parameter `{logo: image("mylogo.svg")}`. Note that
you can then still customize the image's appearance with a set rule within the
package.
</original>

将来，路径可能会成为[一个独立于字符串的数据类型](https://github.com/typst/typst/issues/971)，这样它们就可以保留构造之时的各种信息，资源就可以在不同的根目录中被加载。

<original>
In the future, paths might become a
[distinct type from strings](https://github.com/typst/typst/issues/971), so that
they can retain knowledge of where they were constructed. This way, resources
could be loaded from a different root.
</original>
