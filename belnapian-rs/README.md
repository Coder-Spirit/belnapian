# Belnapian

Belnapian is a library that provides basic types and operations for
[multiple-valued logics](https://en.wikipedia.org/wiki/Many-valued_logic).

## 3-Valued Logic

For [3-valued logics](https://en.wikipedia.org/wiki/Three-valued_logic), we
provide the `TernaryTruth` type:
```rust
pub enum TernaryTruth {
    False,
    True,
    Unknown,
}
```

Belnapian also provides the basic operations `and`, `or`, `not` and `xor`,
but it does not provide "implication" connectives, as they differ between
different 3-valued logic systems such as Kleene logic, RM3 logic, or
[Łukasiewicz logic](https://en.wikipedia.org/wiki/%C5%81ukasiewicz_logic).

My personal recommendation on using these "truth values" is to treat them not as
the actual truth values attributed to a proposition, but to treat them as our
subjective knowledge of them (hence the term "Unknown", which implies the need
of a sentient being not-knowing something).

## 4-Valued Belnap Logic

Belnapian, unsurprisingly, also provides basic support for
[Belnap's 4-valued logic](https://en.wikipedia.org/wiki/Four-valued_logic#Belnap).

As in the case of support for 3-valued logics, this library only provides basic
building blocks, and not any kind of fully fledged inference system.

```rust
pub enum Belnapian {
    // The `Neither` truth value is useful to identify propositions to
    // which we cannot assign any classical truth value. This often
    // happens when the proposition is not well-formed or when it is
    // self-contradictory.
    Neither,

    False,
    True,

    // We can understand `Both` as a superposition of `True` and
    // `False`. A natural case where it makes sense to assign this
    // truth value is when we have a proposition that, given our
    // current set of axioms, could be either `True` or `False`
    // (remember Gödel's incompleteness theorems).
    //
    // In other words, in case that a proposition (or its negation) is
    // independent of our axioms and could be added as a new axiom
    // without causing any inconsistency, then we can assign the
    // `Both` truth value to it.
    Both,
}
```

In constrast to the case of the 3-valued logics with an `Unknown` value, the
`Both` and `Neither` truth values aren't necessarily tied to our subjective
knowledge of the truth value for a given proposition.

Assuming that we operate with a well-known set of axioms, we could use them to
talk about the "real" underlying truth value for a given proposition.

## 15-Valued Extended Belnap Logic (with Unknown values)

The most important feature of the Belnapian library is its support for a
15-valued logic combining Belnap's 4-valued logic with subjective unknown
values.

```rust
pub enum EBelnapian {
    Known(Belnapian),
    Unknown(Unknown),
}

// The enum variants' names are ugly, but once we know what they
// represent, it becomes much easier to use & understand them.
pub enum Unknown {
    NF__, // Could be Neither or False
    N_T_,
    _FT_, // Could be False or True
    NFT_,
    N__B,
    _F_B,
    NF_B,
    __TB, // Could be True or Both
    N_TB,
    _FTB,
    NFTB,
}
```

Once we have more than 2 "objective" truth values, our unknowns can represent
more than one set of possible values (in 3-valued logic, the `Unknown` value
represents the set `{False, True}`).

Our "unknown values" represent the sets present in the power set of
`{Neither, False, True, Both}`, except for the null set `ø` and the singletons
`{Neither}`, `{False}`, `{True}`, and `{Both}` (that is, `2⁴-5 = 16-5 = 11`
values).

The amazing aspect of these "unknown values" is that we can still apply classic
logic operations to them and obtain useful results. This library relies on
pre-computed tables to save you a ton of time when dealing with uncertainty in
logic calculations.
