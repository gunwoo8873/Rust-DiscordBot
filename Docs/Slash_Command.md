# Slash Command Docs

### Slash command value table

| NAME              | VALUE |              NOTE              |
| :---------------- | :---: | :----------------------------: |
| SUB_COMMAND       |   1   |               -                |
| SUB_COMMAND_GROUP |   2   |               -                |
| STRING            |   3   |               -                |
| INTEGER           |   4   | integer between -2^53 amd 2^53 |
| BOOLEAN           |   5   |               -                |
| USER              |   6   |               -                |
| CHANNEL           |   7   |   channel types + categories   |
| ROLE              |   8   |               -                |
| MENTIONABLE       |   9   |        users and roles         |
| NUMBER            |  10   | double between -2^53 amd 2^53  |
| ATTACHMENT        |  11   |       attachment object        |

### Sub Command + Sub Command Group
```rust
CreateCommand::new("aws").description("AWS Command")
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "group_a", "Group A Command") // 2
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "sub_a", "Sub A") // 1
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "param_a", "Param A")) // 3
    )
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "sub_b", "Sub B")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "param_a", "Param A"))
    )
  )
```

### Sub Commands + Sub Options
* OptionType : Select to input OptionType value `String`, `Integer`, `Boolean`, `Number`
* requried : Need to input OptionType value `Boolean [true/false]`
```rust
.add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "status", "Instance Status")
  .add_sub_option(CreateCommandOption::new(CommandOptionType::Number, "instance_number", "Instance Number")
    .required(true)
  )
  .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "actions", "Instance Actions")
    .add_string_choice("start", "Start Instance")
    .add_string_choice("stop", "Stop Instance")
    .add_string_choice("reboot", "Reboot Instance")
    .add_string_choice("terminate", "Terminate Instance")
    .required(true)
  )
)
```
