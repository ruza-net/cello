# Design
## Table behaviour
A table consists of:

1. a title
2. a grid of children

Each child is a table.

## Scoping
A table title can access:

1. the titles of other tables that are on the same level as this table
2. the titles of this table's children *(and their children, recursively)*

## Persistence
A table is saved as a folder *(its name is either a UUID or a user-specified
identifier)* containing a `contents` file and its children.

# Future growth
## Custom semantics
Instead of a table view, the user may supply custom frontend.

A good one would be such that table titles act as articles,
their children as subarticles, and references as links. This way,
one could repurpose a table editor as a personal knowledge base.
