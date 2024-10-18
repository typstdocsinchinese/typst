---
description: |
   How to deal with content that reacts to its location in the document.
en_title: Context
---

# 上下文

有时，我们需要根据在文档中所处的位置来创建元素，比如根据配置的文本语言来展示本地化的短语，或者更简单地，根据前文标题的个数来决定当前标题的标号。
然而 Typst 中的代码并不能直接获取到它在文档中的位置。有些放在开头的代码最终会在文末生成内容。

Sometimes, we want to create content that reacts to its location in the
document. This could be a localized phrase that depends on the configured text
language or something as simple as a heading number which prints the right
value based on how many headings came before it. However, Typst code isn't
directly aware of its location in the document. Some code at the beginning of
the source text could yield content that ends up at the back of the document.

要创建出能根据所处环境做出改变的内容，我们必须特地引导 Typst 这样做：把 `{context}` 关键字放在一个表达式的前面，从而表明它应当根据周围环境生成。
正由于是根据环境生成的，上下文表达式本身会变得不可见，我们并不能在代码中直接访问它的值。
准确地说，这是因为它依赖着上下文，导致它不存在唯一的结果，会在文档的不同位置发生变化。因此，每当需要获取这些上下文的数据时，必须将语句写在上下文表达式内部。

To produce content that is reactive to its surroundings, we must thus
specifically instruct Typst: We do this with the `{context}` keyword, which
precedes an expression and ensures that it is computed with knowledge of its
environment. In return, the context expression itself ends up opaque. We cannot
directly access whatever results from it in our code, precisely because it is
contextual: There is no one correct result, there may be multiple results in
different places of the document. For this reason, everything that depends on
the contextual data must happen inside of the context expression.

除了明确指定上下文表达式以外，上下文也可以在文档中那些位置可知的部分被隐式地创建：[show rules]($styling/#show-rules) 自动提供上下文[^2]，另外例如大纲中的标号也提供了
用于计数的上下文。

Aside from explicit context expressions, context is also established implicitly
in some places that are also aware of their location in the document:
[Show rules]($styling/#show-rules) provide context[^1] and numberings in the
outline, for instance, also provide the proper context to resolve counters.

## 样式上下文

利用 set rules，我们可以修改文档中一小部分或者全文的样式属性，而如果没有已知的上下文，我们不可能访问到这些属性，因为它们在全文范围内会随时改变。
上下文存在时，仅需当做元素字段进行访问，即可获取到它们。

With set rules, we can adjust style properties for parts or the whole of our
document. We cannot access these without a known context, as they may change
throughout the course of the document. When context is available, we can
retrieve them simply by accessing them as fields on the respective element
function.

```example
#set text(lang: "de")
#context text.lang
```

如之前所说，上下文表达式会反映出它所处的位置。在下面的例子中，我们创建了一个上下文表达式，将其存储在了 `value` 变量里，并调用了多次，每次调用都正确地反映了它所处位置的信息。

As explained above, a context expression is reactive to the different
environments it is placed into. In the example below, we create a single context
expression, store it in the `value` variable and use it multiple times. Each use
properly reacts to the current surroundings.

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

Crucially, upon creation, `value` becomes opaque [content] that we cannot peek
into. It can only be resolved when placed somewhere because only then the
context is known. The body of a context expression may be evaluated zero, one,
or multiple times, depending on how many different places it is put into.

## Location context
We've already seen that context gives us access to set rule values. But it can
do more: It also lets us know _where_ in the document we currently are, relative
to other elements, and absolutely on the pages. We can use this information to
create very flexible interactions between different document parts. This
underpins features like heading numbering, the table of contents, or page
headers dependent on section headings.

Some functions like [`counter.get`]($counter.get) implicitly access the current
location. In the example below, we want to retrieve the value of the heading
counter. Since it changes throughout the document, we need to first enter a
context expression. Then, we use `get` to retrieve the counter's current value.
This function accesses the current location from the context to resolve the
counter value. Counters have multiple levels and `get` returns an array with the
resolved numbers. Thus, we get the following result:

```example
#set heading(numbering: "1.")

= Introduction
#lorem(5)

#context counter(heading).get()

= Background
#lorem(5)

#context counter(heading).get()
```

For more flexibility, we can also use the [`here`] function to directly extract
the current [location] from the context. The example below
demonstrates this:

- We first have `{counter(heading).get()}`, which resolves to `{(2,)}` as
  before.
- We then use the more powerful  [`counter.at`] with [`here`], which in
  combination is equivalent to `get`, and thus get `{(2,)}`.
- Finally, we use `at` with a [label] to retrieve the value of the counter at a
  _different_ location in the document, in our case that of the introduction
  heading. This yields `{(1,)}`. Typst's context system gives us time travel
  abilities and lets us retrieve the values of any counters and states at _any_
  location in the document.

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

As mentioned before, we can also use context to get the physical position of
elements on the pages. We do this with the [`locate`] function, which works
similarly to `counter.at`: It takes a location or other [selector] that resolves
to a unique element (could also be a label) and returns the position on the
pages for that element.

```example
Background is at: \
#context locate(<back>).position()

= Introduction <intro>
#lorem(5)
#pagebreak()

= Background <back>
#lorem(5)
```

There are other functions that make use of the location context, most
prominently [`query`]. Take a look at the
[introspection]($category/introspection) category for more details on those.

## Nested contexts
Context is also accessible from within function calls nested in context blocks.
In the example below, `foo` itself becomes a contextual function, just like
[`to-absolute`]($length.to-absolute) is.

```example
#let foo() = 1em.to-absolute()
#context {
  foo() == text.size
}
```

Context blocks can be nested. Contextual code will then always access the
innermost context. The example below demonstrates this: The first `text.lang`
will access the outer context block's styles and as such, it will **not**
see the effect of `{set text(lang: "fr")}`. The nested context block around the
second `text.lang`, however, starts after the set rule and will thus show
its effect.

```example
#set text(lang: "de")
#context [
  #set text(lang: "fr")
  #text.lang \
  #context text.lang
]
```

You might wonder why Typst ignores the French set rule when computing the first
`text.lang` in the example above. The reason is that, in the general case, Typst
cannot know all the styles that will apply as set rules can be applied to
content after it has been constructed. Below, `text.lang` is already computed
when the template function is applied. As such, it cannot possibly be aware of
the language change to French in the template.

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

The second `text.lang`, however, _does_ react to the language change because
evaluation of its surrounding context block is deferred until the styles for it
are known. This illustrates the importance of picking the right insertion point for a context to get access to precisely the right styles.

The same also holds true for the location context. Below, the first
`{c.display()}` call will access the outer context block and will thus not see
the effect of `{c.update(2)}` while the second `{c.display()}` accesses the inner context and will thus see it.

```example
#let c = counter("mycounter")
#c.update(1)
#context [
  #c.update(2)
  #c.display() \
  #context c.display()
]
```

## Compiler iterations
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

A very careful reader might have noticed that not all of the functions presented
above actually make use of the current location. While
`{counter(heading).get()}` definitely depends on it,
`{counter(heading).at(<intro>)}`, for instance, does not. However, it still
requires context. While its value is always the same _within_ one compilation
iteration, it may change over the course of multiple compiler iterations. If one
could call it directly at the top level of a module, the whole module and its
exports could change over the course of multiple compiler iterations, which
would not be desirable.

[^1]: Currently, all show rules provide styling context, but only show rules on
      [locatable]($location/#locatable) elements provide a location context.
