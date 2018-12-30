# Counter using virtual-dom-rs

I wanted to offer this code sample as a way of demonstrating many implentations of React/Redux style programming in Rust. This is a simple interface for incrementing, decrementing and updating a counter, but it shows off a number of patterns:

* data store with state "immutable-style" reducers and listeners
* selectors of state for memoized calculations of state
* actions and async actions
* containers that help map store to components using a "connect" like mechanism
* components using virtual-dom-rs/Percy and how to manage the dom updates

I'm really curious for feedback!
