---
en_title: Scripting
description: Automate your document with Typst's scripting capabilities.
---

# 脚本编写

Typst 内置有强大的脚本语言，你可以用代码来自动化排版以及创建更多广泛适用的样式。下面是对脚本相关概念的简单介绍。

<original>
Typst embeds a powerful scripting language. You can automate your documents and
create more sophisticated styles with code. Below is an overview over the
scripting concepts.
</original>

## 表达式

在 Typst 中，标记（markup）与代码（code）是互相融合在一起的。几乎所有常见元素都是用 _函数_ 创建的。为尽可能的方便，Typst
提供了将代码表达式嵌入 markup 中的紧凑语法：用 `#` 开头即可写出表达式，表达式之外的内容仍然作为正常的 markup
识别。如果接下来要输入的内容并非表达式，但仍然会被算作表达式的一部分，那么可以用分号 `;` 来强制终止表达式。

<original>
In Typst, markup and code are fused into one. All but the most common elements
are created with _functions._ To make this as convenient as possible, Typst
provides compact syntax to embed a code expression into markup: An expression is
introduced with a hash (`#`) and normal markup parsing resumes after the
expression is finished. If a character would continue the expression but should
be interpreted as text, the expression can forcibly be ended with a semicolon
(`;`).
</original>

```example
#emph[Hello] \
#emoji.face \
#"hello".len()
```

上面的例子展示了一些可用的表达式，包括[函数调用]($function)、[字段访问]($scripting/#fields)
以及[方法调用]($scripting/#methods)。本节接下来的内容会讨论更多表达式类型。有一小部分表达式是不能配合井号来使用的，例如二元运算符表达式，但可以借助括号将其放在
markup 里，就像 `[#(1 + 2)]` 这样。

<original>
The example above shows a few of the available expressions, including
[function calls]($function), [field accesses]($scripting/#fields), and
[method calls]($scripting/#methods). More kinds of expressions are
discussed in the remainder of this chapter. A few kinds of expressions are not
compatible with the hash syntax (e.g. binary operator expressions). To embed
these into markup, you can use parentheses, as in `[#(1 + 2)]`.
</original>

## 块

为了能够合理安排代码以及在代码内加入 markup，Typst 提供了两种 _块_：

<original>
To structure your code and embed markup into it, Typst provides two kinds of
_blocks:_
</original>

- **代码块：** `{{ let x = 1; x + 2 }}` \
  在编写代码的时候，你可能会想要将一系列的计算拆分成多个式子，创建一些中间变量等等。
  代码块可以让你在原本只能放一句代码的地方编写多个表达式。在代码块中，单个语句之间必须以换行或者分号分隔。
  代码块中的表达式的结果最终被加在一起，成为该块的最终值。对于那些不输出有效结果的语句，例如 `{let}` 绑定表达式只会
  返回 `{none}`，它与任何值加在一起时，不会对结果产生任何影响。

<original>
- **Code block:** `{{ let x = 1; x + 2 }}` \
  When writing code, you'll probably want to split up your computation into
  multiple statements, create some intermediate variables and so on. Code blocks
  let you write multiple expressions where one is expected. The individual
  expressions in a code block should be separated by line breaks or semicolons.
  The output values of the individual expressions in a code block are joined to
  determine the block's value. Expressions without useful output, like `{let}`
  bindings yield `{none}`, which can be joined with any value without effect.
</original>

- **内容块：** `{[*Hey* there!]}` \
  利用内容块，你可以将 markup 作为编程中的值进行处理，例如可以将它存在变量里并传入到[函数]($function)中。
  内容块以方括号分隔，其内容是任意的 markup，它最终会产出 [content] 类型的值。内容块可以放在函数
  的后面来传入函数，也就是说 `{list([A], [B])}` 相当于 `{list[A][B]}`。

<original>
- **Content block:** `{[*Hey* there!]}` \
  With content blocks, you can handle markup/content as a programmatic value,
  store it in variables and pass it to [functions]($function). Content
  blocks are delimited by square brackets and can contain arbitrary markup. A
  content block results in a value of type [content]. An arbitrary number of
  content blocks can be passed as trailing arguments to functions. That is,
  `{list([A], [B])}` is equivalent to `{list[A][B]}`.
</original>

内容块和代码块可以嵌套。在下面的例子里，`{[hello ]}` 的值与 `{a + [ the ] + b}`
加在了一起，最终产生 `{[hello from the *world*]}`。

<original>
Content and code blocks can be nested arbitrarily. In the example below,
`{[hello ]}` is joined with the output of  `{a + [ the ] + b}` yielding
`{[hello from the *world*]}`.
</original>

```example
#{
  let a = [from]
  let b = [*world*]
  [hello ]
  a + [ the ] + b
}
```

## 绑定与解构

如前文所述，变量可以用 `{let}` 绑定来定义，它会被赋予 `=`
号之后的表达式的返回值。如果变量没有被手动赋值，则会被默认初始化为 `{none}`。`{let}`
关键字也可以用来创建[自定义的具名函数]($function/#defining-functions)。变量一经赋值，其后的部分都可以访问到它的值（在其所在的块或文件范围内）。

<original>
As already demonstrated above, variables can be defined with `{let}` bindings.
The variable is assigned the value of the expression that follows the `=` sign.
The assignment of a value is optional, if no value is assigned, the variable
will be initialized as `{none}`. The `{let}` keyword can also be used to create
a [custom named function]($function/#defining-functions). Variables can be
accessed for the rest of the containing block (or the rest of the file if there
is no containing block).
</original>

```example
#let name = "Typst"
This is #name's documentation.
It explains #name.

#let add(x, y) = x + y
Sum is #add(2, 3).
```

let 绑定还可以用来解构[数组]($array)和[字典]($dictionary)。在这种情况下，等号左侧在形式上应该与右侧保持一致。`..`
操作符可以用来代替数组和字典中的剩余部分元素。

<original>
Let bindings can also be used to destructure [arrays]($array) and
[dictionaries]($dictionary). In this case, the left-hand side of the
assignment should mirror an array or dictionary. The `..` operator can be used
once in the pattern to collect the remainder of the array's or dictionary's
items.
</original>

```example
#let (x, y) = (1, 2)
The coordinates are #x, #y.

#let (a, .., b) = (1, 2, 3, 4)
The first element is #a.
The last element is #b.

#let books = (
  Shakespeare: "Hamlet",
  Homer: "The Odyssey",
  Austen: "Persuasion",
)

#let (Austen,) = books
Austen wrote #Austen.

#let (Homer: h) = books
Homer wrote #h.

#let (Homer, ..other) = books
#for (author, title) in other [
  #author wrote #title.
]
```

在解构模板（译者注：即等号左侧）中，你可以用下划线来舍弃那些不需要的元素。

<original>
You can use the underscore to discard elements in a destructuring pattern:
</original>

```example
#let (_, y, _) = (1, 2, 3)
The y coordinate is #y.
```

在函数的参数列表中也可以使用解构。

<original>
Destructuring also work in argument lists of functions ...
</original>

```example
#let left = (2, 4, 5)
#let right = (3, 2, 6)
#left.zip(right).map(
  ((a,b)) => a + b
)
```

在一般的赋值语句左侧也可以用解构。这在需要交换变量的值时很有用。

<original>
... and on the left-hand side of normal assignments. This can be useful to
swap variables among other things.
</original>

```example
#{
  let a = 1
  let b = 2
  (a, b) = (b, a)
  [a = #a, b = #b]
}
```

## 条件语句

借助条件语句，你可以根据条件是否满足与否，来选择展示内容或者进行计算。Typst 支持 `{if}`、`{else if}` 和 `{else}`
表达式。当条件取值为 `{true}` 时，条件语句会返回 if 后的内容，否则返回 else 后的内容。

<original>
With a conditional, you can display or compute different things depending on
whether some condition is fulfilled. Typst supports `{if}`, `{else if}` and
`{else}` expression. When the condition evaluates to `{true}`, the conditional
yields the value resulting from the if's body, otherwise yields the value
resulting from the else's body.
</original>

```example
#if 1 < 2 [
  This is shown
] else [
  This is not.
]
```

每一个分支都可以以一个代码块或者内容块为主体。

<original>
Each branch can have a code or content block as its body.
</original>

- `{if condition {..}}`
- `{if condition [..]}`
- `{if condition [..] else {..}}`
- `{if condition [..] else if condition {..} else [..]}`

## 循环

借助循环，你可以重复显示一些内容，或者迭代计算一些东西。Typst 支持 `{for}` 和 `{while}`
两种循环。前者用来遍历一个指定的集合，而后者持续循环直到条件不再满足。循环会将每一次遍历的结果加起来作为一个值返回。

<original>
With loops, you can repeat content or compute something iteratively. Typst
supports two types of loops: `{for}` and `{while}` loops. The former iterate
over a specified collection whereas the latter iterate as long as a condition
stays fulfilled. Just like blocks, loops _join_ the results from each iteration
into one value.
</original>

在下面的例子中，使用 for 循环创建的三个句子最终被加在一起作为单个内容值，while 循环中长度为 1 的数组最终被合并成为更大的数组。

<original>
In the example below, the three sentences created by the for loop join together
into a single content value and the length-1 arrays in the while loop join
together into one larger array.
</original>

```example
#for c in "ABC" [
  #c is a letter.
]

#let n = 2
#while n < 10 {
  n = (n * 2) - 1
  (n,)
}
```

for 循环可以遍历许多类型的集合：

<original>
For loops can iterate over a variety of collections:
</original>

- `{for value in array {..}}` \
  遍历[数组]($array)中的每一项。在[绑定与解构](#绑定与解构)中提到的解构语法也可以在这里使用。

<original>
- `{for value in array {..}}` \
  Iterates over the items in the [array]. The destructuring syntax described in
  [Let binding]($scripting/#bindings) can also be used here.
</original>

- `{for pair in dict {..}}` \
  遍历[字典]($dictionary)的键值对。键值对可以用 `{for (key, value) in dict {..}}` 进行解构。
  这种写法比 `{for pair in dict.pairs() {..}}` 效率更高，后者额外创建了包含所有键值对的临时数组。

<original>
- `{for pair in dict {..}}` \
  Iterates over the key-value pairs of the [dictionary]. The pairs can also be
  destructured by using `{for (key, value) in dict {..}}`. It is more efficient
  than `{for pair in dict.pairs() {..}}` because it doesn't create a temporary
  array of all key-value pairs.
</original>

- `{for letter in "abc" {..}}` \
  遍历[字符串]($str)的每个字符。严格来说，它是在遍历字符串的字素组合（grapheme clusters）。
  多数情况下，一个字素组合就是一个 codepoint，不过也可能包含多个，
  例如旗帜的 emoji（译者注：旗帜的 emoji 可以看成是普通旗帜的 codepoint 与对应国家代号的 codepoint 的组合）。

<original>
- `{for letter in "abc" {..}}` \
  Iterates over the characters of the [string]($str). Technically, it iterates
  over the grapheme clusters of the string. Most of the time, a grapheme cluster
  is just a single codepoint. However, a grapheme cluster could contain multiple
  codepoints, like a flag emoji.
</original>

- `{for byte in bytes("😀") {..}}` \
  遍历所有[字节]($bytes)。所谓字节，可从[字符串]($str)转换而来，也可以通过读取文件（不进行编码）获得。
  这里的字节是一个介于 `{0}` 和 `{255}` 之间的[整数]($int)。

<original>
- `{for byte in bytes("😀") {..}}` \
  Iterates over the [bytes], which can be converted from a [string]($str) or
  [read] from a file without encoding. Each byte value is an [integer]($int)
  between `{0}` and `{255}`.
</original>

为了控制循环的执行过程，Typst 提供了 `{break}` 和 `{continue}` 语句。前者会中断循环，后者会跳过当前的循环流程直接进行下一个流程。

<original>
To control the execution of the loop, Typst provides the `{break}` and
`{continue}` statements. The former performs an early exit from the loop while
the latter skips ahead to the next iteration of the loop.
</original>

```example
#for letter in "abc nope" {
  if letter == " " {
    break
  }

  letter
}
```

循环体可以是代码块或者内容块。

<original>
The body of a loop can be a code or content block:
</original>

- `{for .. in collection {..}}`
- `{for .. in collection [..]}`
- `{while condition {..}}`
- `{while condition [..]}`

## 字段

你可以使用 _点号_ 来访问一个值的字段。对于 [`content`] 类型的值，你还可以用 [`fields`]($content.fields) 函数来列出它的所有字段。

<original>
You can use _dot notation_ to access fields on a value. For values of type
[`content`], you can also use the [`fields`]($content.fields) function to list
the fields.
</original>

如需访问一个值的字段，这个值应当为下面列出的任意一种：

- [字典]($dictionary)，目标键必须存在
- [符号]($symbol)，目标修饰器必须有效
- [模组]($module)，目标定义必须存在
- 包含指定字段的元素所组成的[内容]($content)。元素上存在的字段与构建该元素时传入[元素函数]($function/#element-functions)
  的那些参数保持一致。

<original>
The value in question can be either:
- a [dictionary] that has the specified key,
- a [symbol] that has the specified modifier,
- a [module] containing the specified definition,
- [content] consisting of an element that has the specified field. The
  available fields match the arguments of the
  [element function]($function/#element-functions) that were given when the
  element was constructed.
</original>

```example
#let it = [= Heading]
#it.body \
#it.depth \
#it.fields()

#let dict = (greet: "Hello")
#dict.greet \
#emoji.face

```

## 方法

_方法调用_ 是调用某种数据类型上专有的函数的便捷方法。例如，我们可以通过下面两种等价的方式调用 [`str.len`]($str.len) 函数：

<original>
A _method call_ is a convenient way to call a function that is scoped to a
value's [type]. For example, we can call the [`str.len`]($str.len) function in
the following two equivalent ways:
</original>

```example
#str.len("abc") is the same as
#"abc".len()
```

方法调用的写法是 `{value.method(..args)}`，等价的完整函数调用写法是 `{type(value).method(value, ..args)}`
。每种数据类型的文档中都列出了它们所带有的函数。方法是不能自行定义的。

<original>
The structure of a method call is `{value.method(..args)}` and its equivalent
full function call is `{type(value).method(value, ..args)}`. The documentation
of each type lists it's scoped functions. You cannot currently define your own
methods.
</original>

```example
#let values = (1, 2, 3, 4)
#values.pop() \
#values.len() \

#("a, b, c"
    .split(", ")
    .join[ --- ])

#"abc".len() is the same as
#str.len("abc")
```

有一些函数会修改调用它们时所指的目标，这些函数 _必须_ 以方法的形式被调用。某些情况下，如果一个方法调用只是为了它的副作用（side
effect），其返回值应当被忽略，不应参与后续的叠加。
舍弃一个值的通用方法是这样一个 let 绑定：`{let _ = array.remove(1)}`

<original>
There are a few special functions that modify the value they are called on (e.g.
[`array.push`]($array.push)). These functions _must_ be called in method form.
In some cases, when the method is only called for its side effect, its return
value should be ignored (and not participate in joining). The canonical way to
discard a value is with a let binding: `{let _ = array.remove(1)}`.
</original>

## 模块

你可以将你的 Typst 项目细分为许多个文件，称为 _模块_。一个模块可以用下面几种方式来引用其它模块中的内容和定义：

- **引入（Including）：** `{include "bar.typ"}` \
  解析 `bar.typ` 路径所指的文件，返回[内容]($content)作为结果。
- **导入（Importing）：** `{import "bar.typ"}` \
  解析 `bar.typ` 路径所指的文件，将[模块]($module)结果插入到当前的作用域与文件名同名（除去扩展名）的变量中，例如 `bar`。
  你可以用 `as` 关键字来手动指定导入模块的名称：`{import "bar.typ" as baz}`
  ，还可以用点号来导入嵌套的项目： `{import "bar.typ": baz.a}`
- **导入项目：** `{import "bar.typ": a, b}` \
  解析 `bar.typ` 路径所指的文件，将其中的 `a` 和 `b`（也就是在 `bar.typ` 中已经用 `{let}` 绑定等方法定义过的某些变量）取出并赋值到当前文件中的同名变量中。
  将 `a, b` 换成 `*` 将导入模块中定义的全部变量。你可以用 `as`
  关键字来重命名单个项目：`{import "bar.typ": a as one, b as two}`

<original>
You can split up your Typst projects into multiple files called _modules._ A
module can refer to the content and definitions of another module in multiple
ways:
- **Including:** `{include "bar.typ"}` \
  Evaluates the file at the path `bar.typ` and returns the resulting [content].
- **Import:** `{import "bar.typ"}` \
  Evaluates the file at the path `bar.typ` and inserts the resulting [module]
  into the current scope as `bar` (filename without extension). You can use the
  `as` keyword to rename the imported module: `{import "bar.typ" as baz}`. You
  can import nested items using dot notation: `{import "bar.typ": baz.a}`.
- **Import items:** `{import "bar.typ": a, b}` \
  Evaluates the file at the path `bar.typ`, extracts the values of the variables
  `a` and `b` (that need to be defined in `bar.typ`, e.g. through `{let}`
  bindings) and defines them in the current file. Replacing `a, b` with `*`
  loads all variables defined in a module. You can use the `as` keyword to
  rename the individual items: `{import "bar.typ": a as one, b as two}`
</original>

除了路径，你还可以用[模块值]($module)来导入模块，如：

<original>
Instead of a path, you can also use a [module value]($module), as shown in the
following example:
</original>

```example
#import emoji: face
#face.grin
```

## 包

为了在不同的项目中复用所构建的块，你可以创建并导入 Typst _包_。包的导入包括命名空间、名称和版本三个信息。

<original>
To reuse building blocks across projects, you can also create and import Typst
_packages._ A package import is specified as a triple of a namespace, a name,
and a version.
</original>

```example
>>> #let add(x, y) = x + y
<<< #import "@preview/example:0.1.0": add
#add(2, 7)
```

`preview` 这一命名空间中包含了社区中共享的各种包，你可以在 [Typst Universe]($universe) 上找到可用的社区包。

<original>
The `preview` namespace contains packages shared by the community. You can find
all available community packages on [Typst Universe]($universe).
</original>

如果你是在本地使用 Typst，你还可以自行创建本地包。要了解更多，参考 GitHub
上的 [package 仓库](https://github.com/typst/packages)。

<original>
If you are using Typst locally, you can also create your own system-local
packages. For more details on this, see the
[package repository](https://github.com/typst/packages).
</original>

## 运算符

下表列出了所有可用的一元和二元运算符以及它们的作用、操作元数和优先级（越高越优先）。

<original>
The following table lists all available unary and binary operators with effect,
arity (unary, binary) and precedence level (higher binds stronger).
</original>

|  运算符  | 效果          | 元数 | 优先级 |
|:----------:|-----------------|:-----:|:----------:|
|   `{-}`    | 取反              |  一元   |     7      |
|   `{+}`    | 无作用（考虑对称性保留该符号） |  一元   |     7      |
|   `{*}`    | 乘               |  二元   |     6      |
|   `{/}`    | 除               |  二元   |     6      |
|   `{+}`    | 加               |  二元   |     5      |
|   `{-}`    | 减               |  二元   |     5      |
|   `{==}`   | 等值判断            |  二元   |     4      |
|   `{!=}`   | 不等值判断           |  二元   |     4      |
|   `{<}`    | 小于判断            |  二元   |     4      |
|   `{<=}`   | 小于或等于判断         |  二元   |     4      |
|   `{>}`    | 大于判断            |  二元   |     4      |
|   `{>=}`   | 大于或等于判断         |  二元   |     4      |
|   `{in}`   | 判断是否存在于集合中      |  二元   |     4      |
| `{not in}` | 判断是否不存在于集合中     |  二元   |     4      |
|  `{not}`   | 逻辑非             |  一元   |     3      |
|  `{and}`   | 短路逻辑与           |  二元   |     3      |
|   `{or}`   | 短路逻辑或           |  二元   |     2      |
|   `{=}`    | 赋值              |  二元   |     1      |
|   `{+=}`   | 加赋值             |  二元   |     1      |
|   `{-=}`   | 减赋值             |  二元   |     1      |
|   `{*=}`   | 乘赋值             |  二元   |     1      |
|   `{/=}`   | 除赋值             |  二元   |     1      |

[semver]: https://semver.org/
