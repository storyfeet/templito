Templito
=======

The Back end Templating language behind the Siter - a static website generator.

What makes templito different
========================

Templito templates largely resemble those of handlebars and Go templates, but they have a few standout features.

* Language
    * Multiple parameters to templates
    * Functions can be run on the result of a block
    * Whitespace can be escaped
* Rust
    * Template functions can be closures, so they can have data attached to the functions.
    * Template and Function Managers are separate from the template and can be switched in and out generically.


Examples
=========

Basic structure of a template

```html
{{let title = "Page"}}\
<h1>{{first $title "Default Title"}}</h1>
{{if $0.cat}}\
    {{for k v in $0.cat}}\
        <p>{{$k}} = {{$v}}</p>
    {{/for}}\
{{/if}}\
```

Using Blocks
```html
{{@md}}
Markdown
============================

Everthing inside an '@md' block will be treated as markdown.
(Assuming the md function is included by the FuncManager)

Even ranges like this:

{{for k v in $0.items}}
* {{$v}}
{{/for}}

{{/md}}
```

In fact any string to string function can be used as an "@block"

But they are more powerful than that.

```html
{{@let lines}}
apple
cat
sandwhich
{{/let}}\
{{for k v in split $lines}}
    <p>{{$v}}</p>
{{/for}}
```
will output:

```html
    <p>apple</p>
    <p>cat</p>
    <p>sandwhich</p>
```

Depending on the security situation, you can opt in or out to allowing "exec" functions. ("exec" is not included in the defaults)

```html
{{for k v in split (exec "cat" "path/to/file") "\n"}}
    <p>{{$k}} = {{$v}}</p>
{{/for}}
```











