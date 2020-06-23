## NOTES

## Invariants

There are some invariants I want to maintain here, that I can't always
get the compiler to help me enforce. I mention these here:


* Parameters and return values with types from dependencies
    * TLDR: Don't expose types from dependencies except
        in very special circumstances
    * Cargo will very easily allow you to mix multiple versions
        of a dependency if a transitive dependency requires
        a different version.
        This means that if you have a function that accepts or
        returns a type from a third party crate, the
        values may not actually be compatible
* Performance and number of sprite batches
    * It's just assumed that if you have too many sprite batches
        things will run slow.
