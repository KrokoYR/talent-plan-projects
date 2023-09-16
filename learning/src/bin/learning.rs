use learning::ExampleCommand;
use learning::TheTrait;
use learning::{f, Flags};

fn main() {
    f(Flags::FLAG_A | Flags::FLAG_C);

    let mut example_command = ExampleCommand::new(String::from("ExampleCommand"));
    example_command
        .args(&["asd".to_owned()])
        .arg("another arg".to_owned());

    println!("Example command: {:?}", example_command);
    println!("Example command: {}", example_command.program);

    let the_trait: usize = 4;
    usize::some_pub_function(the_trait);
    // ?? this should not be running, I guess ?? //
    usize::some_hidden_function();
}
