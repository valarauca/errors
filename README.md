# Errors!

A relatively complex crate that does a massive amount of error handling behind your back.

* Type reflection? Sure.
* Stack Traces? Of course.
* Light weight cloning? why not!

## How to use?

```toml
[dependencies]
errors = { git = "https://github.com/valarauca/errors" }
```

## How do I understand it?

Docs Link => [linky](https://valarauca.github.io/errors/errors/index.html)

## Back Trace Info

Requires a backtrace library, and you'll want ot ensure you build your programs with
`BACKTRACE=1` set to ensure enough debugging information is populated to fill stack
traces.


