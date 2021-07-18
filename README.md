# Rust Linked List

## WORK ON THIS REPO IS FINISHED

This repo contains a simple (singly) linked list implementation in Rust. There are no special restrictions or conditions imposed on the implementation, it's simply for the sake of learning: I've never implemented a data structure in Rust and I'd like to see how the implementation would look like in this language.

The repo is licensed under The Unlicense, so it's public domain. You can do whatever you want with it.

The linked list in this implemenetation:
- Is a singly linked list, with the only pointer being the head pointer;
- Contains the push() method that pushes a new element to the list's head;
- Contains the pop() method that pops one element from the list's head, makes the next element the head and returns the popped value;
- Contains the for_each() method that takes a closure and applies it to each element of the list;
- Implements the fmt::Display trait that makes the list printable with, for example, a println! macro.