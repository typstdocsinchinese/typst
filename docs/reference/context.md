---
description: |
   How to deal with content that reacts to its location in the document.
en_title: Context
---

# 上下文

有时，我们需要根据在文档中所处的位置来创建元素，比如根据配置的文本语言来展示本地化的短语，或者更简单地，根据前文标题的个数来决定当前标题的标号。
然而 Typst 中的代码并不能直接获取到它在文档中的位置。有些放在开头的代码最终会在文末生成内容。

<original>
Sometimes, we want to create content that reacts to its location in the
document. This could be a localized phrase that depends on the configured text
language or something as simple as a heading number which prints the right
value based on how many headings came before it. However, Typst code isn't
directly aware of its location in the document. Some code at the beginning of
the source text could yield content that ends up at the back of the document.
</original>

要创建出能根据所处环境做出改变的内容，我们必须特地引导 Typst 这样做：把 `{context}` 关键字放在一个表达式的前面，从而表明它应当根据周围环境生成。
正由于是根据环境生成的，上下文表达式本身会变得不可见，我们并不能在代码中直接访问它的值。
准确地说，这是因为它依赖着上下文，导致它不存在唯一的结果，会在文档的不同位置发生变化。因此，每当需要获取这些上下文的数据时，必须将语句写在上下文表达式内部。

<original>
To produce content that is reactive to its surroundings, we must thus
specifically instruct Typst: We do this with the `{context}` keyword, which
precedes an expression and ensures that it is computed with knowledge of its
environment. In return, the context expression itself ends up opaque. We cannot
directly access whatever results from it in our code, precisely because it is
contextual: There is no one correct result, there may be multiple results in
different places of the document. For this reason, everything that depends on
the contextual data must happen inside of the context expression.
</original>

除了明确指定上下文表达式以外，上下文也可以在文档中那些位置可知的部分被隐式地创建：[show rules]($styling/#show-rules) 自动提供上下文[^1]，另外例如大纲中的标号也提供了
用于计数的上下文。

<original>
Aside from explicit context expressions, context is also established implicitly
in some places that are also aware of their location in the document:
[Show rules]($styling/#show-rules) provide context[^1] and numberings in the
outline, for instance, also provide the proper context to resolve counters.
</original>

## 样式上下文

利用 set rules，我们可以修改文档中一小部分或者全文的样式属性，而如果没有已知的上下文，我们不可能访问到这些属性，因为它们在全文范围内会随时改变。
上下文存在时，仅需当做元素字段进行访问，即可获取到它们。

<original>
With set rules, we can adjust style properties for parts or the whole of our
document. We cannot access these without a known context, as they may change
throughout the course of the document. When context is available, we can
retrieve them simply by accessing them as fields on the respective element
function.
</original>

```example
#set text(lang: "de")
#context text.lang
```

如之前所说，上下文表达式会反映出它所处的位置。在下面的例子中，我们创建了一个上下文表达式，将其存储在了 `value` 变量里，并调用了多次，每次调用都正确地反映了它所处位置的信息。

<original>
As explained above, a context expression is reactive to the different
environments it is placed into. In the example below, we create a single context
expression, store it in the `value` variable and use it multiple times. Each use
properly reacts to the current surroundings.
</original>

```example
#let value = context text.lang
#value

#set text(lang: "de")
#value

#set text(lang: "fr")
#value
```

注意，在创建之时，`value` 就变成了不可见的[内容]($content)。它的值只会在它被放在某个地方的时候解析出来，因为只有在这时上下文才已知。
上下文表达式体可能被取值任意次，这取决于它被放在了多少个不同的位置。

<original>
Crucially, upon creation, `value` becomes opaque [content] that we cannot peek
into. It can only be resolved when placed somewhere because only then the
context is known. The body of a context expression may be evaluated zero, one,
or multiple times, depending on how many different places it is put into.
</original>

## 位置上下文

我们知道，借助上下文可以访问 set rule 的值。不过它的作用不仅限于此，它还可以让我们确定目前在文档中所处的位置，包括相对于其他元素的位置和对于整页的绝对位置。
借助这些信息，我们可以与文章的不同部分灵活互动。这为一些特性奠定了基础，例如标题的标号、页面目录（TOC）以及那些依赖小节标题内容的页面标题。

<original>
We've already seen that context gives us access to set rule values. But it can
do more: It also lets us know _where_ in the document we currently are, relative
to other elements, and absolutely on the pages. We can use this information to
create very flexible interactions between different document parts. This
underpins features like heading numbering, the table of contents, or page
headers dependent on section headings.
</original>

有些函数，例如 [`counter.get`]($counter.get)，会隐式地访问当前的位置。在下面的例子中，我们想要获取到标题计数器的值，由于它会在全文范围内发生变化，我们需要
先进入一个上下文表达式中，然后再用 `get` 来获取计数器的值。该函数从上下文中获取到当前的位置信息（location），并解析出计数器的具体值。计数器有多层，`get` 返回的是它解析出的一系列数字数组。
这样，我们就得到了下面的结果：

<original>
Some functions like [`counter.get`]($counter.get) implicitly access the current
location. In the example below, we want to retrieve the value of the heading
counter. Since it changes throughout the document, we need to first enter a
context expression. Then, we use `get` to retrieve the counter's current value.
This function accesses the current location from the context to resolve the
counter value. Counters have multiple levels and `get` returns an array with the
resolved numbers. Thus, we get the following result:
</original>

```example
#set heading(numbering: "1.")

= Introduction
#lorem(5)

#context counter(heading).get()

= Background
#lorem(5)

#context counter(heading).get()
```

更灵活地，我们还可以用 [`here`] 函数直接从上下文中解析出当前的[位置]($location)。下面的例子解释了这一点：

<original>
For more flexibility, we can also use the [`here`] function to directly extract
the current [location] from the context. The example below
demonstrates this:
</original>

- 我们先写了 `{counter(heading).get()}`，它和之前一样，被解析为 `{(2,)}`
- 然后，我们用到了功能更强的 [`counter.at`] 并配合 [`here`]，合起来相当于是 `get`，于是得到了 `{(2,)}`
- 最终，我们配合[标签]($label)，用 `at` 在另一个地方获取计数器的值。Typst 的上下文系统赋予了我们时间穿越的能力，让我们能够在文档的任何地方获取到任何一个计数器的值。

<original>
- We first have `{counter(heading).get()}`, which resolves to `{(2,)}` as
  before.
- We then use the more powerful  [`counter.at`] with [`here`], which in
  combination is equivalent to `get`, and thus get `{(2,)}`.
- Finally, we use `at` with a [label] to retrieve the value of the counter at a
  _different_ location in the document, in our case that of the introduction
  heading. This yields `{(1,)}`. Typst's context system gives us time travel
  abilities and lets us retrieve the values of any counters and states at _any_
  location in the document.
</original>

```example
#set heading(numbering: "1.")

= Introduction <intro>
#lorem(5)

= Background <back>
#lorem(5)

#context [
  #counter(heading).get() \
  #counter(heading).at(here()) \
  #counter(heading).at(<intro>)
]
```

如之前提到的那样，我们还可以用上下文来获取到元素在页面中的物理位置，这一点可通过 [`locate`] 函数实现，它与 `counter.at` 原理差不多：它接收一个 location 或者指代一个独立元素的[选择器]($selector)（也可以是一个标签），返回该元素在页面中的位置。

<original>
As mentioned before, we can also use context to get the physical position of
elements on the pages. We do this with the [`locate`] function, which works
similarly to `counter.at`: It takes a location or other [selector] that resolves
to a unique element (could also be a label) and returns the position on the
pages for that element.
</original>

```example
Background is at: \
#context locate(<back>).position()

= Introduction <intro>
#lorem(5)
#pagebreak()

= Background <back>
#lorem(5)
```

还有一些其它的函数也会用到位置上下文（location context），尤其是 [`query`]。可参考[内检]($category/introspection)这一分类了解更多。

<original>
There are other functions that make use of the location context, most
prominently [`query`]. Take a look at the
[introspection]($category/introspection) category for more details on those.
</original>

## 嵌套上下文

当函数调用嵌套于上下文块中的时候，也可以访问到上下文。在下面的例子中，`foo` 和 [`to-absolute`]($length.to-absolute) 一样变成了一个上下文函数（contextual function）。

<original>
Context is also accessible from within function calls nested in context blocks.
In the example below, `foo` itself becomes a contextual function, just like
[`to-absolute`]($length.to-absolute) is.
</original>

```example
#let foo() = 1em.to-absolute()
#context {
  foo() == text.size
}
```

上下文块是可以嵌套的。与上下文相关的代码总是会访问最内层的上下文。下面的例子说明了这一点：第一个 `text.lang` 会访问到外层上下文块的样式，由于其本身的特性，它**不会**受到 `{set text(lang: "fr")}` 的影响。然而，第二个 `text.lang` 所在的嵌套上下块在 set rule 之后，所以会展示出相应的效果来。

<original>
Context blocks can be nested. Contextual code will then always access the
innermost context. The example below demonstrates this: The first `text.lang`
will access the outer context block's styles and as such, it will **not**
see the effect of `{set text(lang: "fr")}`. The nested context block around the
second `text.lang`, however, starts after the set rule and will thus show
its effect.
</original>

```example
#set text(lang: "de")
#context [
  #set text(lang: "fr")
  #text.lang \
  #context text.lang
]
```

你可能会好奇为什么在上面的例子中 Typst 在计算第一个 `text.lang` 时忽略了法语 set rule（译者注：即 `#set text(lang: "fr")`）的影响。这是因为，在一般情况下 Typst 不可能在内容被构建之时就知道即将应用到它上的所有样式。
下面的 `text.lang` 在 template 函数应用之前就已经被计算了，所以它不可能知道在 template 内部还有对语言到法语的切换。

<original>
You might wonder why Typst ignores the French set rule when computing the first
`text.lang` in the example above. The reason is that, in the general case, Typst
cannot know all the styles that will apply as set rules can be applied to
content after it has been constructed. Below, `text.lang` is already computed
when the template function is applied. As such, it cannot possibly be aware of
the language change to French in the template.
</original>

```example
#let template(body) = {
  set text(lang: "fr")
  upper(body)
}

#set text(lang: "de")
#context [
  #show: template
  #text.lang \
  #context text.lang
]
```

然而第二个 `text.lang` 的确受到了语言切换的应先向，因为它所处的上下文块被延缓到所有相关样式都已知之后才执行。
这说明了选择插入 context 的正确位置的重要性，只有选对了才能准确访问到正确的样式。

<original>
The second `text.lang`, however, _does_ react to the language change because
evaluation of its surrounding context block is deferred until the styles for it
are known. This illustrates the importance of picking the right insertion point for a context to get access to precisely the right styles.
</original>

这对于位置上下文同样适用。下面的第一个 `{c.display()}` 调用只访问外部的上下文块，因此不会受到 `{c.update(2)}` 的影响，而第二个 `{c.display()}` 会访问内部的上下文，于是会受到影响。

<original>
The same also holds true for the location context. Below, the first
`{c.display()}` call will access the outer context block and will thus not see
the effect of `{c.update(2)}` while the second `{c.display()}` accesses the inner context and will thus see it.
</original>

```example
#let c = counter("mycounter")
#c.update(1)
#context [
  #c.update(2)
  #c.display() \
  #context c.display()
]
```

## 编译器迭代

为了实现上下文的交互，Typst 编译器会将你的文档处理多次。比如，为了解析对 `locate` 的调用，Typst 先提供了一个占位的位置（译者注：只用作占位，不准确）进去，然后再去处理你文档的布局，最后再从最终的布局获取到确切的位置填进去。同样的方法也用在解析计数器（counters）、状态（states）和查询（queries）上面。在某些情况下，Typst 甚至还需要迭代超过两次才能解析所有东西。尽管这可能是必要的，这还可能是误用上下文函数或者[状态]($state/#caution)的标志。如果 Typst 尝试五次都无法将所有东西解析出来，就会停止，并输出警告 "layout did not converge within 5
attempts."（经过五次尝试后仍然无法集中内容）。

<original>
To resolve contextual interactions, the Typst compiler processes your document
multiple times. For instance, to resolve a `locate` call, Typst first provides a
placeholder position, layouts your document and then recompiles with the known
position from the finished layout. The same approach is taken to resolve
counters, states, and queries. In certain cases, Typst may even need more than
two iterations to resolve everything. While that's sometimes a necessity, it may
also be a sign of misuse of contextual functions (e.g. of
[state]($state/#caution)). If Typst cannot resolve everything within five
attempts, it will stop and output the warning "layout did not converge within 5
attempts."
</original>

如果你在很谨慎地阅读文档，你可能注意到并不是上面举出的所有函数都利用了当前的位置信息。虽然 `{counter(heading).get()}` 肯定用到了，而像 `{counter(heading).at(<intro>)}` 就没有，但是它仍然需要上下文才行。它的值在单一的编译 _之中_ 是保持一致的，但在迭代多次之后它的值也可能发生变化。如果有人在模块的顶层直接调用它，这整个模块及其导出内容会在数次迭代编译的过程中不断变化，这并不是我们想要的效果。

<original>
A very careful reader might have noticed that not all of the functions presented
above actually make use of the current location. While
`{counter(heading).get()}` definitely depends on it,
`{counter(heading).at(<..>)}`, for instance, does not. However, it still
requires context. While its value is always the same _within_ one compilation
iteration, it may change over the course of multiple compiler iterations. If one
could call it directly at the top level of a module, the whole module and its
exports could change over the course of multiple compiler iterations, which
would not be desirable.
</original>

[^1]: 目前，所有的 show rule 都提供了样式上下文，但是只有针对[可定位的]($location/#locatable)元素的 show rule 会提供位置上下文。
