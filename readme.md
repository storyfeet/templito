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
{{if .cat}}\
    {{for k v in .cat}}\
        <p>{{$k}} = {{$v}}</p>
    {{/for}}\
{{/if}}\
```

Using Blocks
```html
{{@md}}
Markdown
============================

Everthing between an '@<func>' and '\func' tags will run the function on the result of the contents.

for example the because this code is within an '@md' block, the contents will be treated as markdown.

(Assuming the md function is included by the FuncManager)

Ranges like this will create a bullet point list of items:

{{for k v in .items}}
* {{$v}} 
{{- /for}}

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

## Control Flow

* "let"
    
    Create variables (Used in rest of docs)
    {{let x=3;y=9;list=["fish","dog"]}}
* "export"

    Export variables can be returned to the calling program in a map of exports

* "if","elif" and "else" 

    Control flow, only resolve the contents if the contition is met otherwise look at the elif and elses
    {{if gt $x 3}}{{$x}}{{elif gt $y 3}}{{$y}}{{else}}Something else{{/if}}
    //output: 9

* "return"
    
    return a value instead of the main string

* "for" 

    Loop over a list or value. This requires the index, and value be named (eg k and v)
    {{for k v in $list}}list at {{$k}} equals {{$v}};{{/for}}

    //output: list at 0 equals fish;list at 1 equals dog;

* "switch" and "case"
 
    choose one option based on the value matched by a pattern
    {{- switch $list -}}
        {{- case [_,<b>]}}List with second element {{$b}}
        {{- case {fish:<f>} }}Map with fish element {{$f}}
    {{- /switch -}}
    //output: List with second element dog

 * "as"
    
    Create a single switch case

    {{as $list: [_,<b>]}}{{$b}}{{/as}}
    //output: dog

* "define" and  "global" 
    
    Create functions either as variables to run or globals that are added to the func manager
    {{define cat age}}I'm a cat aged {{$age}}{/define}}
    {{run $cat 55}}
    //output: I'm a cat aged 55
 
* "@let" and "@export"

    Create a local or exported variable from the contents of the block;
    {{- @let a -}}
        I love {{for k v in $list}}{{if eq $k 0}} and {{/if}}{{$v}}{{/for}}
    {{- /let -}}
    {{$a}} 
    //output: I love fish and dog




Changelog
=========

V 0.4.0
-------
Keyword "as" now exists which could possibly break some previous functions of the same name.
Can now use ```{{as val:pattern}}with pattern captures here{{/as}}```

V 0.2.0
--------

Breaking Change : Functions now require a description to make documentation much easier for users of the system.

Funcmanagers now also provide/require a method to provide those descriptions so they can be printed by any software that uses the system.











