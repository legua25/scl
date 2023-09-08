# `scl` &ndash; A modern configuration language
![0.1.0](https://img.shields.io/badge/version-0.1.0-blue?style=flat-square)
![MIT](https://img.shields.io/badge/MIT-grey?style=flat-square)

`scl` (pronounced as *scale*) is a modern configuration language inspired by the `nginx` configuration syntax which  
features a rich type set and interchangeable textual and binary representations which are also human-and-machine 
friendly. It is designed as both a configuration and exchange format, geared towards tool and service configuration, as 
well as efficient transport for distributed applications.

The reference implementation is written in Rust. Additional language bindings will come once the Rust API is stabilized.

## Examples
See [`scl.g4`](./packages/scl/scl.g4) for the full grammar specification for the language.

```scl
# This is a comment. `scl` allows including comments in its textual representation
total_count: 1
items "users" [
    {
        # Entries in `scl` can include metadata which describe, i.e: its type, context or other information
        id "uuid": "85026ad9-2b84-4b95-9389-cd4d6a2bd739",
        
        # `scl` supports decimal numbers as well as regular floating-point numbers for added precision
        account: 1234.56d,
        
        # Strings in `scl` are quoted (") strings, which can also span multiple lines
        display_name: "John Doe",
        profile: """
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras ac quam felis. Nulla facilisi. Pellentesque id 
        mi sapien. Duis luctus eget ex et congue.
        """,
        
        # `scl` includes ISO-8601 timestamps which convert to the target language's date/time types
        created_at: 2017-02-17T14:32:16Z,
         
        # Raw binary data can be embedded in an `scl` document
        secret: {{ bm90IGEgc2VjcmV0 }},
    }
]
```

## Legal
This project is licensed under the MIT license. See [`LICENSE`](./LICENSE) file for license information.  
**Copyright &copy; 2022-2023** Luis Gutierrez.
