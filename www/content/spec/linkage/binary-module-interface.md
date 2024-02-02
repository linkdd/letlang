---
title: Binary Module Interface
layout: spec
weight: 2
---

# Binary Module Interface

Each Letlang module MUST provide metadata about the exported symbols.

This interface is used when linking against an already compiled module.

## Format

The file format MAY be a JSON document satisfying the following JSON schema:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://letlang.dev/schemas/bmi.json",
  "$defs": {
    "interface": {
      "type": "object",
      "properties": {
        "module": { "$ref": "#/$defs/module-path" },
        "symbols": {
          "type": "array",
          "items": { "$ref": "#/$defs/symbol" }
        }
      },
      "required": ["module", "symbols"]
    },
    "module-path": {
      "type": "string",
      "minLength": 1
    },
    "symbol": {
      "oneOf": [
        { "$ref": "#/$defs/symbol-class" },
        { "$ref": "#/$defs/symbol-function" },
        { "$ref": "#/$defs/symbol-effect" }
      ]
    },
    "symbol-class": {
      "type": "object",
      "properties": {
        "kind": { "type": "string", "enum": ["class"] },
        "name": { "type": "string", "minLength": 1 },
        "type-arity": { "type": "integer", "minimum": 0 }
      },
      "required": ["kind", "name", "type-arity"]
    },
    "symbol-function": {
      "type": "object",
      "properties": {
        "kind": { "type": "string", "enum": ["function"] },
        "name": { "type": "string", "minLength": 1 },
        "type-arity": { "type": "integer", "minimum": 0 },
        "call-arity": { "type": "integer", "minimum": 0 }
      },
      "required": ["kind", "name", "type-arity", "call-arity"]
    },
    "symbol-effect": {
      "type": "object",
      "properties": {
        "kind": { "type": "string", "enum": ["effect"] },
        "name": { "type": "string", "minLength": 1 },
        "type-arity": { "type": "integer", "minimum": 0 },
        "call-arity": { "type": "integer", "minimum": 0 }
      },
      "required": ["kind", "name", "type-arity", "call-arity"]
    }
  },
  "allOf": [
    { "$ref": "#/$defs/interface" }
  ]
}
```
