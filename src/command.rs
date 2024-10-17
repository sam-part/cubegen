enum CommandGroup {}

struct CommandInfo {
    name: String,
    group: CommandGroup,
}

impl CommandInfo {
    fn new(name: String, group: CommandGroup) -> Self {
        CommandInfo { name, group }
    }
}
