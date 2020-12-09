# The Builder Pattern

Roughly speaking, this pattern involves creating a struct that holds some
(private) internal state about parameters to a function, mutating the state via
method calls or setters, then eventually calling build `build()` method. The
builder pattern is prevalent in Rust in large part because as of December 2020,
keyword, optional, and default arguments are not language features. [The
history](https://github.com/rust-lang/rfcs/issues/323) is a topic for another
day, but one can understand why the builder pattern finds it way into may
libraries, given its many advantages. It is reasonably concise, taking one
extra character per argument when compared to Python syntax. 

The builder pattern can either consume `self` when creating its next iteration
or mutably borrow. We discuss the trade offs later, but in practice, the pattern
looks like this.

```rust
struct Term{};
struct Date{};

struct QueryBuilder{
    // More fields
    query: String
}

impl QueryBuilder{
    // More methods

    pub fn add_search_term(self, term: Term) -> Self{
       // Update `self` in some way
        self
    }

    pub fn add_date_range(&mut self, start: &Date, end: &Date) -> &mut Self{
       // Update `self` in some way
        self
    }
}
```
The function `add_search_term` will manipulate a couple of pointers, steal the
contents of `self`, and likely avoid copying the query string. The second
method avoid a string copy as well, but it returns a mutable reference, leaving
ownership unaffected and preserving existing identities.  

In practice, the following libraries rely on the builder pattern. 

1. [`hyper::server::Builder`](https://docs.rs/hyper/0.13.9/hyper/server/struct.Builder.html) v 0.13.5
2. [`reqwest::RequestBuilder`](https://docs.rs/reqwest/0.10.9/reqwest/struct.RequestBuilder.html) v 0.10.9
3. [`amethyst::GameDataBuilder`](https://docs.amethyst.rs/stable/amethyst/struct.GameDataBuilder.html) v 0.15.3
4. [`rayon::ThreadPoolBuilder`](https://docs.rs/rayon/1.5.0/rayon/struct.ThreadPoolBuilder.html) v 1.5.0
5. [`arrow::csv::ReaderBuilder`](https://docs.rs/arrow/2.0.0/arrow/csv/reader/struct.ReaderBuilder.html) v 2.0.0
6. [`clap::App`](https://docs.rs/clap/2.33.3/clap/struct.App.html) v 2.33.3
7. [`mysql::OptsBuilder`](https://docs.rs/mysql/20.1.0/mysql/struct.OptsBuilder.html) v 20.1.0

Moving out of `self` in the builder methods permits incrementally constructing
a complicated collection of arguments with low overhead, since each method call
will typically mutate a couple of fields and adjust a couple of pointers.

It should be reiterated that the builder pattern exists as an alternative to
default and keyword arguments. Languages such as Python take advantage of this for some killer functions.

```python
import pandas as pd
# pandas
def read_csv(
    filepath_or_buffer: FilePathOrBuffer,
    sep=",",
    delimiter=None,
    # Column and Index Locations and Names
    header="infer",
    ...
):
    pass


# 95 % of use cases
my_dataframe = pd.read_csv("myfile.txt")

# 99 % of use cases
# Data accretion that can be domain specific, as the use case requires.
my_dataframe2 = pd.read_csv("myfile.txt", **postgres_dtypes, **hardware_constraints)
```

The 1.0.0 of Pandas shipped with approximately 50 parameters on
[`pd.read_csv`](https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.read_csv.html).
This is not isolated. Although few fields have as many customization options as
graphics, matplotlib and the data viz stack built on top of it may take
advantage of keyword and default arguments more and any other space in Python.
It is hard to imagine the builder pattern matching this type of power and
terseness. Of course handling this at compile time seems out of scope.

```python
import matplotlib.pyplot as plt

my_dataframe2.plot.hist(
    bins=100, figsize=(12, 12), **mpl_colors, **labels, **mpl_markers
)
plt.show()
```

While the builder pattern would struggle with something as highly customizable
as the scientific stack (consider that much of that stack has expanded well
beyond the realm of integers and floats), rustc does not pay the price of
requiring an allocation for every single function call, as is the case in
CPython. But more on this later.

The ability to pass around these parameters liberally as maps, apparent degree
of polymorphism, composibility and sheer amount of forgiveness are all tempting.
[Rich Hickey](https://clojure.org/) has [more to say on this
matter](https://www.youtube.com/watch?v=YR5WdGrpoug). 
