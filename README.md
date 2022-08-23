# debug_unwraps

Adds debug only unwrap functionality to Option and Result types for Rust

## Motivation

When writing a new high-level structure use of `.unwrap()` and `.expect()` can 
be useful to ensure that internal invariants are upheld. This allows unit and 
integration tests to quickly fail if a code-change violates some expected 
structure. 

On the other hand, in release mode it would be nice to strip such checks because
it is expected that the API itself will enforce the invariant.

Consider the following example structure: 

```rust
/// A silly dag structure which creates a heirachichal group of sets of numbers
struct NumTreeBuilder {
    /// Stack of currently edited nodes
    stack: Vec<usize>,
    /// An adjacency list of children
    nodes: Vec<Vec<usize>>,
    /// List of numbers associated with each node
    data: Vec<Vec<u32>>,
}

impl NumTreeBuilder {
    /// Creates an empty `NumTreeBuilder`
    /// 
    /// Editing will start at the root
    pub fn new() -> Self {
        Self {
            // Editing starts at the `0` node which is the root
            stack: vec![0],
            // Insert an empty list of children for node 0
            nodes: vec![Vec::new()],
            // Default name for the root
            names: vec![Vec::new()],
        }
    }

    /// Add a new number to the set of the currently edited node
    pub fn add_num(&mut self, num: u32) {
        if let Some(current) = self.stack.last() {
            self.data.get_mut(*current)
                .expect("A node ID was on the stack but didn't have associated data")
                .push(num);
        } else {
            unreachable!("The stack should never be empty because the root cannot be popped");
        }
    }

    /// Start a new child under the current node
    pub fn start_child(&mut self) {
        // Create a new child node
        let child_id = self.nodes.len();
        // INVARIANT: Children must have adjacency and data structures created
        // before they can be edited
        self.nodes.push(Vec::new());
        self.data.push(Vec::new());

        // Update the child list of the current node and push the new child
        // onto the edit stack
        if let Some(current) = self.stack.last() {
            self.nodes.get_mut(*current)
                .expect("A node ID was on the stack but didn't have associated data")
                .push(child_id);

            self.stack.push(child_id);
        } else {
            unreachable!("The stack should never be empty because the root cannot be popped");
        }
    }

    /// Stop editing a child and return to its parent
    pub fn finish_child(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
        // INVARIANT: Don't pop if we are at the root
    }
}
```

The API is designed to not allow invalid states such as an empty stack or 
missing node data, but we are forced to always do run-time checks. 
Currently the only way to get debug only checks is to use `debug_assert!()`:

```rust
impl NumTreeBuilder {
    /// Start a new child under the current node
    pub fn start_child(&mut self) {
        // Create a new child node
        let child_id = self.nodes.len();
        // INVARIANT: Children must have adjacency and data structures created
        // before they can be edited
        self.nodes.push(Vec::new());
        self.data.push(Vec::new());

        // Edit the parent adjacency list

        debug_assert!(self.stack.last().is_some(), "The stack should never be empty because the root cannot be popped");
        // SAFETY: this invariant should be upheld by the API because `finish_child()` 
        // only pops an item if the stack is greater than 1 item long
        // In debug mode this will panic in the above debug_assert!
        let current = unsafe {self.stack.last().unwrap_unchecked()};

        debug_assert!(self.nodes.get_mut(*current).is_some(), "A node ID was on the stack but didn't have associated data");
        // SAFETY: this invariatn is upheld above because all new child nodes
        // have nodes and data items pushed by `start_child()`
        // In debug mode this will panic in the above debug_assert!
        unsafe { self.nodes.get_mut(*current).unwrap_uncheckd().push(child_id) };

        // Start editing the new child
        self.stack.push(child_id);
    }
}
```

This has some disadvantages because it separates the error check from the location
that the error can be generated. Additionally if each operation is in its own
`unsafe` block it increases the number of locations to audit when checking for
crate safety. 

## Solution

This crate provides extension traits for `Option<T>` and `Result<T,E>` which
conditionally enable debugging only when compiled with `debug-assertions`. 

```rust
impl NumTreeBuilder {
    /// Start a new child under the current node
    pub fn start_child(&mut self) {
        // Use extension trait
        use debug_unwraps::DebugUnwrapExt;

        // Create a new child node
        let child_id = self.nodes.len();

        // INVARIANT: Children must have adjacency and data structures created
        // before they can be edited
        self.nodes.push(Vec::new());
        self.data.push(Vec::new());

        // SAFETY: The invariants that the stack is not empty and all nodes have
        // an adjacency list are upheld by the rest of the API. In debug mode
        // these invariants will be checked
        unsafe {
            let current = self.stack.last().debug_expect_unchecked("The stack should never be empty because the root cannot be popped");
            self.nodes.get_mut(*current)
                .debug_expect_unchecked("A node ID was on the stack but didn't have associated data")
                .push(child_id);
        }

        // Start editing the new child
        self.stack.push(child_id);
    }
}
```

With this code, the error checking is once again inline and failures during 
refactorign can be caught during unit/integration tests. But in Release the
code will not bother checking. 