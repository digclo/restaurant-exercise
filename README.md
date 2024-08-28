# Summary
This project is an exercise in Rust to simulate a restaurant ordering system. Tablets are used to create, view, or delete orders that are assigned to a table. Running the application will only output a count of the number of API requests the app has handled.

# Requirements
The target requirements can be found in the `tests/test.rs` file.

The top of `main.rs` has several consts that can be used to adjust behavior including the number of tables, the number of tablets making orders, and the max duration a tablet should wait in between orders.
# Parameters

# Dependencies
The two crates that are being used are as follows:
- `uuid`: This is the primary identifier used for all normalized data objects. I chose this over an enumerator to simplify the `Default` boilerplate for structs.
- `rng`: This is primarily used for simulating tablet behavior. Since the service should be taking in multiple requests over a duration of time, rng helps to randomize request behavior.

# Comments
I decided to do something a bit different instead of the typical scenario (`tokio`, `serde`, `sqlx`, `reqwest`) and instead desired to create something with as few crates as possible.

This led to two design decisions for this exercise:
1. While a module will exist to interact with data storage, all storage will be done in-memory.
2. To mimic asynchronous requests without `tokio`, channels from the `mpsc` standard library will be used.

## Data Modeling
I took the advice of one Rust content creator and started by mapping out the requirements in a data model using Rust's rich type system. The goal of this data model is to avoid mistakes, and to normalize the data so the in-memory data store can mimic a collection of database tables.

### Uuid as Structs
One example of protecting from mistakes is the use of `uuid`s. One could easily define a `uid` field in each struct and have it's type be `Uuid`. However, this can lead to potential issues such as mistakingly assigning the `uuid` to the wrong struct identifier. To avoid this, I encapsulated every `uuid` inside a struct (ex: `TableId(Uuid)`). While this requires more boilerplate in the type definition, it makes working with these `uuid`s much easier because the value provides its own type check. It also makes it impossible to accidentally assign a `TableId(Uuid)` to an order identifier, because that type can only be `OrderId(Uuid)`.

### Data Normalization
My original data model looked more like my TypeScript roots. I had a table struct that contained a vector of its own orders. However, my personal requirement of making the data store like a DB table made this data model difficult to work with. It was then I realized I should normalize my data for a table instead of an app. This resulted in me creating the `Order` struct which contains its own `uuid`, and the `uuid` of its associated table, and menu item. With this setup, I was easily able to create a data store struct where each field is a "table", each "table" contains a vector of "rows".

## The "API"
Because the scenario being played is a collection of tablets making requests to a central "server", it seemed that `mpsc` would be the best fit since each tablet is a `Sender` while our server is the `Receiver`. This was met with a complexity, because my initial setup allowed the tablets to communicate with the app, but not the inverse.

Fortunately, I previously learned about a feature with enums that could help. I knew I wanted to use a enum for each request type so the compiler can inform me of all possible request types I need to handle. Additionally, enums can also contain structs, which is the strategy I used in how the app can communicate back to the tablets. 

I defined the enums in they contain two core attributes:
1. The arguments required in order to make a data store query.
2. A `Sender` that the app can use to return the resulting data back to the tablet.

I really liked this design as each `Sender` must have a defined generic of the expected response type. And each unique `Sender` was self contained in the request variant, again making it impossible to accidentally use the wrong `Sender` in a different request.

## Tablets
The first thing I addressed in the tablet module was defining the API request methods. I wanted to treat this as if each method could likely have a `reqwest` call to a server. But in my case, would be using an `mpsc::Sender`. Once this was done I created a function that attempts to trigger random behavior with some rules in place. Such as if a table has no orders, we should only create an order. 

After some initial testing, I found that as tables contained an unrealistic number of orders (into the thousands), the performance of the app was slowing down due to the amount and frequency of heap memory allocation. To combat this, (as well as having a more realistic "restaurant" scenario) I added an additional rule. If a table reaches 10 orders then the party leaves and all orders are cleared to make way for new orders. 

## Testing
I typically find when building an API (be it with a crate or something more unique like this), having integration tests is more sensible compared to unit tests where you're mocks are dependant on the API implementation. For this exercise, I decided to create a separate test file to cover the expected requirements. Because is API is meant to service tablets, I want the majority of my tests to rely on their API request methods.

Setting this up required a bit of refactoring. For one, I needed to update `Cargo.toml` to have this project include both a library and a binary, otherwise I cannot import anything from `main` into my test. Additionally, I wanted to have some control over how the app behaved during tests. To do this I extracted the main request loop into its own module so I can start it in a separate thread. Once our assessments pass, the test ends and the thread is dropped.

## Had I More Time
The next thing I would have liked to do if I had more time was to create a UI (possibly through a terminal UI crate like `cursive`) that neatly presents the automatic activity of the tablets. This would include stats on each table and their collection of orders. How many of each menu item was ordered. And the number of requests being handled per second.
