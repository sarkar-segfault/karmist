# karmist: todoist, but for gigachads
karmist is a small rust application to manage tasks. it is still very much under development, and lacks many features, but has been deemed usable for small cases by its author (me).

# features
- COLORFUL!!
- invokable at the command-line
- create, read, update and delete tasks
- tasks have properties - id, title, description, completed

# installation
with cargo, you can `cargo install karmist`. if you like to feign independence, you can build it from source.

# usage
here are some karmist commands and their uses:
- `karmist --database <db.json>`: specify the task database for karmist, defaults to `.karmist.json`.
- `karmist create <id?>`: create a task with id `<id?>`. if id is not supplied, the user is prompted for it.
- `karmist read <id?>`: read a task with id `<id?>`, or all tasks in the database otherwise.
- `karmist update <id?>`: update a task with id `<id?>`. if id is not supplied, the user is prompted for it.
- `karmist delete <id?>`: delete a task with id `<id?>`, or all tasks in the database otherwise.

# contributions
first of all, there is absolutely no reason you'd want to commit to this project. if you still wanna, [i can't stop you](https://github.com/sarkar-segfault/karmist).
