---
title: Modules and Imports
layout: spec
weight: 4
---

# Modules

A module is a Letlang source file that MUST start with the module declaration
statement.

### Syntax

{% apply grammkit %}
module_declaration_statement = "module" module_path ";"
module_path = identifier ("::" identifier)*
{% endapply %}


### Example

```letlang
module foo::main;

# ...
```

# Imports

Other modules can be imported in the current module scope, and optionnally
aliased.

Symbols from other modules can be imported in the current module scope, and each
of them can be optionnally aliased.

### Syntax

{% apply grammkit %}
import_statement
  = ( "import" module_path ("as" identifier)? ";")
  / ( "from" module_path "import" "{" import_symbol ("," import_symbol)* ","? "}" ";")

import_symbol = identifier ("as" identifier)?
{% endapply %}


### Examples

```letlang
import std::io as stdio;
from std::proc import { self, send as send_message };
```

To access a symbol in an imported module, the complete path to the symbol is
required:

{% apply grammkit %}
symbol_path = identifier ("::" identifier)*
{% endapply %}


### Example

```letlang
stdio::println("hello world");
```

```letlang
import std::io as stdio;
import std::proc;

let pub main: func[() -> @ok] {
  () -> {
    stdio::println(std::proc::self());
    @ok;
  },
};
```
