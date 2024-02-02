---
title: IO Capabilities
layout: spec
weight: 2
---

# IO Capabilities

IO capabilities are an abstraction of lower-level resources such as file
descriptors, sockets, and other IO-related handles.

The runtime MUST provide an implementation for such capabilities, and
encapsulate them into an `iocap` value that is opaque to Letlang.

IO operations are effectful, therefore the runtime MUST provide effect handlers
to acquire, dispose, write and read from IO capabilities.

The signature of those effects is implementation-defined.
