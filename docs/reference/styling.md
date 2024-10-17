---
en_title: Styling
description: All concepts needed to style your document with Typst.
---

# 样式定义

Typst 中有一套灵活的样式系统，它会根据你的选择自动将样式应用到文档中。利用 _set rules_，你可以配置元素的基础属性，从而创建出一些常见的样式。然而元素不一定有每一个你想要自定义的属性，因此 Typst 又支持了 show rules，它可以完全重新定义元素的外观。

<original>
Typst includes a flexible styling system that automatically applies styling of
your choice to your document. With _set rules,_ you can configure basic
properties of elements. This way, you create most common styles. However, there
might not be a built-in property for everything you wish to do. For this reason,
Typst further supports _show rules_ that can completely redefine the appearance
of elements.
</original>

## Set rules

借助 set rules，你可以自定义元素的外观。一个 set rule 的具体写法是在 `{set}` 关键字（在 markup 模式中为 `[#set]`）后跟上一个对[元素函数]($function/#element-functions)的[调用]($function)。只有元素函数的可选参数能被传入 set rule 中。至于哪些参数是可选的，请参阅相应函数的文档。在下面的例子中，我们用到了两个 set rules 来修改[字体族]($text.font)和[标题的标号]($heading.numbering)。

<original>
With set rules, you can customize the appearance of elements. They are written
as a [function call]($function) to an [element
function]($function/#element-functions) preceded by the `{set}` keyword (or
`[#set]` in markup). Only optional parameters of that function can be provided
to the set rule. Refer to each function's documentation to see which parameters
are optional. In the example below, we use two set rules to change the
[font family]($text.font) and [heading numbering]($heading.numbering).
</original>

```example
#set heading(numbering: "I.")
#set text(
  font: "New Computer Modern"
)

= Introduction
With set rules, you can style
your document.
```

一个顶层的 set rule 会持续作用至文件结束，而如果它是嵌套在一个块中，则只会作用到块的结束。因此，借助块（block），你可以将某个 rule 的作用限制在文档的一小段中。在下面我们用一个内容块将列表样式限制在了这一特定的列表中。

<original>
A top level set rule stays in effect until the end of the file. When nested
inside of a block, it is only in effect until the end of that block. With a
block, you can thus restrict the effect of a rule to a particular segment of
your document. Below, we use a content block to scope the list styling to one
particular list.
</original>

```example
This list is affected: #[
  #set list(marker: [--])
  - Dash
]

This one is not:
- Bullet
```

有时，你可能希望只在特定的条件下应用 set rule，为此可以使用 _set-if_ rule。

<original>
Sometimes, you'll want to apply a set rule conditionally. For this, you can use
a _set-if_ rule.
</original>

```example
#let task(body, critical: false) = {
  set text(red) if critical
  [- #body]
}

#task(critical: true)[Food today?]
#task(critical: false)[Work deadline]
```

## Show rules

借助 show rule，你可以深度地定制元素的外观。一个 show rule 最基本的形式是一个 _show-set rule_，它的写法是在 `{show}` 关键字后面跟上一个[选择器]($selector)，接着跟上冒号和一个 set rule。选择器最基本的形式就是一个[元素函数]($function/#element-functions)，这使得 set rule 只会影响到被选择到的元素。在下面的例子中，标题变为了深蓝色，其它文本则没有改变。

<original>
With show rules, you can deeply customize the look of a type of element. The
most basic form of show rule is a _show-set rule._ Such a rule is written as the
`{show}` keyword followed by a [selector], a colon and then a set rule. The most
basic form of selector is an [element function]($function/#element-functions).
This lets the set rule only apply to the selected element. In the example below,
headings become dark blue while all other text stays black.
</original>

```example
#show heading: set text(navy)

= This is navy-blue
But this stays black.
```

在 show-set rules 中你可以混用不同函数的属性来达到各种效果，但这仍然受制于 Typst 中预定义内容的多少。如果需要更大的自定义空间，你还可以从零开始在 show rule 中编写格式化元素的方式。要写出这样一个 show rule，可将冒号后面的 set rule 换成一个[函数]($function)，该函数应接受一个内容参数，可返回任意内容。传入函数的元素上所存在的字段与其对应的元素函数一致。在下面我们定义了一个 show rule，它将标题定义为幻想作品百科应有的风格。

<original>
With show-set rules you can mix and match properties from different functions to
achieve many different effects. But they still limit you to what is predefined
in Typst. For maximum flexibility, you can instead write a show rule that
defines how to format an element from scratch. To write such a show rule,
replace the set rule after the colon with an arbitrary [function]. This function
receives the element in question and can return arbitrary content. The available
[fields]($scripting/#fields) on the element passed to the function again match
the parameters of the respective element function. Below, we define a show rule
that formats headings for a fantasy encyclopedia.
---
译注：省略了部分arbitrary含义，无需过于强调“任意”
</original>

```example
#set heading(numbering: "(I)")
#show heading: it => [
  #set align(center)
  #set text(font: "Inria Serif")
  \~ #emph(it.body)
     #counter(heading).display(
       it.numbering
     ) \~
]

= Dragon
With a base health of 15, the
dragon is the most powerful
creature.

= Manticore
While less powerful than the
dragon, the manticore gets
extra style points.
```

和 set rules 一样，show rules 同样作用于它在的块或文件。

<original>
Like set rules, show rules are in effect until the end of the current block or
file.
</original>

除了函数外，show rule 的右侧也可以是字符串字面量或者内容块，它们会替换掉目标元素。另外，show rule 的左侧可以有若干个其它类型的选择器，它们定义了这些变换的适用对象。这些选择器包括：

<original>
Instead of a function, the right-hand side of a show rule can also take a
literal string or content block that should be directly substituted for the
element. And apart from a function, the left-hand side of a show rule can also
take a number of other _selectors_ that define what to apply the transformation
to:
</original>

- **任意元素**： `{show: rest => ..}` \
  选择所有内容。这在你需要将一个复杂的布局应用到整个文档时很有用，
  因为这样你就不需要将一切都包围在一个巨大的函数调用中了。

- **文本**： `{show "Text": ..}` \
  选择对应的文本。可用于定义特定文本的样式，或者执行替换。

- **正则表达式**： `{show regex("\w+"): ..}` \
  根据正则表达式去更灵活地选择文本对象。可参考 [`regex` 类型]($regex)了解更多。

- **函数字段访问**： `{show heading.where(level: 1): ..}` \
  选择具有相应字段的元素。例如选择所有一级的标题（level 为 1）。

- **标签**: `{show <intro>: ..}` \
  选择具有对应标签的元素。可参考 [`label` 类型]($label)了解更多。

<original>
- **Everything:** `{show: rest => ..}` \
  Transform everything after the show rule. This is useful to apply a more
  complex layout to your whole document without wrapping everything in a giant
  function call.
- **Text:** `{show "Text": ..}` \
  Style, transform or replace text.
- **Regex:** `{show regex("\w+"): ..}` \
  Select and transform text with a regular expression for even more flexibility.
  See the documentation of the [`regex` type]($regex) for details.
- **Function with fields:** `{show heading.where(level: 1): ..}` \
  Transform only elements that have the specified fields. For example, you might
  want to only change the style of level-1 headings.
- **Label:** `{show <..> ..}` \
  Select and transform elements that have the specified label. See the
  documentation of the [`label` type]($label) for more details.
</original>

```example
#show "Project": smallcaps
#show "badly": "great"

We started Project in 2019
and are still working on it.
Project is progressing badly.
```
