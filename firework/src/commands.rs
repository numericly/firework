use async_recursion::async_recursion;
use firework_protocol::{
    core::{SerializeField, VarInt},
    data_types::SuggestionMatch,
    protocol_derive::SerializeField as SerializeFieldDerive,
};
use futures::future::BoxFuture;
use modular_bitfield::{bitfield, BitfieldSpecifier};
use serde_json::{json, Value};
use std::{fmt::Debug, io::Write};
use thiserror::Error;
use tokio::sync::Mutex;

use crate::{client::Client, Server, ServerHandler, ServerProxy};

type CommandFn<Handler, Proxy> = Box<
    dyn for<'a> Fn(
            Vec<Argument>,
            &'a Client<Handler, Proxy>,
            &'a Server<Handler, Proxy>,
            &'a Proxy,
        ) -> BoxFuture<'a, ()>
        + Send
        + Sync,
>;

pub struct CommandTree<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    commands: Vec<Command<Handler, Proxy>>,
}

pub struct Command<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    name: &'static str,
    description: &'static str,
    aliases: Option<Vec<CommandAlias>>,
    execution: Option<CommandFn<Handler, Proxy>>,
    children: Vec<CommandNode<Handler, Proxy>>,

    node_index: Mutex<Option<i32>>,
}

pub struct CommandNode<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    node_type: NodeType,
    execution: Option<CommandFn<Handler, Proxy>>,
    children: Vec<CommandNode<Handler, Proxy>>,

    node_index: Mutex<Option<i32>>,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Literal {
        name: String,
    },
    Argument {
        name: String,
        parser: ArgumentType,
    },
    ServerCompletedArgument {
        name: String,
        parser: ArgumentType,
        suggestions: Vec<String>,
    },
}

#[derive(Debug, PartialEq)]
pub enum SuggestionsType {
    AskServer,
    AllRecipes,
    AvailableSounds,
    AvailableBiomes,
    SummonableEntities,
}

#[derive(SerializeFieldDerive, Debug, PartialEq, Clone)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum StringType {
    SingleWord,
    QuotablePhrase,
    GreedyPhrase,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArgumentType {
    Bool,
    Float { min: Option<f32>, max: Option<f32> },
    String { string_type: StringType },
}

#[derive(Debug, Clone)]
pub enum Argument {
    Literal { value: String },
    Bool { value: bool },
    Float { value: f32 },
    String { value: String },
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("missing command")]
    EmptyCommand,
    #[error("no matches for data \"{data}\"")]
    NoMatches { data: String },
    #[error("\"{data}\" not in {suggestions:?}")]
    NotInSuggestions {
        suggestions: Vec<String>,
        data: String,
    },
    #[error("command is incomplete")]
    Incomplete,
    #[error("command is not executable")]
    NotExecutable,
}

#[derive(Debug)]
pub struct CommandAlias {
    name: &'static str,
    node_index: Mutex<Option<i32>>,
}

pub struct ParseResult<'a, Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    pub executable: Result<&'a CommandFn<Handler, Proxy>, CommandError>,
    pub suggestions: Option<Suggestions>,
}

impl<'a, Handler, Proxy> Debug for ParseResult<'a, Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseResult")
            .field("executable", &self.executable.as_ref().map(|_| "fn() {}"))
            .field("suggestions", &self.suggestions)
            .finish()
    }
}

#[derive(Debug)]
pub struct Suggestions {
    pub start: usize,
    pub length: usize,
    pub matches: Vec<SuggestionMatch>,
}

impl<Handler, Proxy> CommandTree<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    pub fn new() -> Self {
        CommandTree {
            commands: Vec::new(),
        }
    }
    pub fn register_command(mut self, command: Command<Handler, Proxy>) -> Self {
        self.commands.push(command);
        self
    }
    pub fn build_help_command(self) -> Self {
        let suggestions: Vec<_> = self.commands.iter().map(|c| c.name.to_string()).collect();

        let help_command = Command::new("help", "list commands and their usages").add_node(
            CommandNode::server_argument(
                "command",
                ArgumentType::String {
                    string_type: StringType::SingleWord,
                },
                suggestions,
            )
            .with_execution(Box::new(move |args, client, server, proxy| {
                Box::pin(help(args, client, server, proxy))
            })),
        );

        async fn help<Handler, Proxy>(
            args: Vec<Argument>,
            client: &Client<Handler, Proxy>,
            server: &Server<Handler, Proxy>,
            proxy: &Proxy,
        ) where
            Proxy: ServerProxy + Send + Sync + 'static,
            Handler: ServerHandler<Proxy> + Send + Sync + 'static,
        {
            let root = server.handler.get_commands(server, proxy).await.unwrap();

            if let Some(arg) = args.get(0) {
                if let Argument::String { value } = arg {
                    let command = root.commands.iter().find(|c| c.name == *value).unwrap();
                    let mut extra = Value::Array(Vec::new());

                    extra.as_array_mut().unwrap().extend(vec![
                        json!({
                            "text": "\n - ",
                            "color": "white"
                        }),
                        json!({
                            "text": command.name,
                            "color": "yellow"
                        }),
                    ]);

                    if let Some(aliases) = command.aliases.as_ref() {
                        let aliases = aliases
                            .iter()
                            .map(|a| a.name.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");

                        extra.as_array_mut().unwrap().push(json!({
                            "text": format!(", aliases [{}]", aliases),
                            "color": "dark_gray"
                        }));
                    }

                    extra.as_array_mut().unwrap().push(json!({
                        "text": format!(", {}", command.description),
                        "color": "gray"
                    }));

                    client.send_system_chat_message(
                        json!({
                            "text": "Help for: ",
                            "color": "green",
                            "extra": extra
                        })
                        .to_string(),
                        false,
                    );
                    return;
                }
            }

            let mut extra = Value::Array(Vec::new());

            for command in &root.commands {
                extra.as_array_mut().unwrap().extend(vec![
                    json!({
                        "text": "\n - ",
                        "color": "white"
                    }),
                    json!({
                        "text": command.name,
                        "color": "yellow"
                    }),
                ]);

                if let Some(aliases) = command.aliases.as_ref() {
                    let aliases = aliases
                        .iter()
                        .map(|a| a.name.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");

                    extra.as_array_mut().unwrap().push(json!({
                        "text": format!(", aliases [{}]", aliases),
                        "color": "dark_gray"
                    }));
                }

                extra.as_array_mut().unwrap().push(json!({
                    "text": format!(", {}", command.description),
                    "color": "gray"
                }));
            }

            client.send_system_chat_message(
                json!({
                    "text": "List of commands:",
                    "color": "green",
                    "extra": extra
                })
                .to_string(),
                false,
            );
        }

        self.register_command(help_command.with_execution(Box::new(
            move |args, client, server, proxy| Box::pin(help(args, client, server, proxy)),
        )))
    }
    pub(crate) async fn serialize<W: Write + Send + Sync>(&self, mut writer: &mut W) {
        let mut index = 1;

        let mut children_indexes = Vec::with_capacity(self.commands.len());

        for command in &self.commands {
            command.assign_index(&mut index).await;
            if let Some(aliases) = &command.aliases {
                for alias in aliases {
                    children_indexes.push(VarInt::from(
                        alias.node_index.lock().await.expect("no index"),
                    ));
                }
            }
            children_indexes.push(VarInt::from(
                command.node_index.lock().await.expect("no index"),
            ));
        }

        VarInt::from(index).serialize(&mut writer);

        let root_node = RawCommandNode {
            flags: CommandFlags::new().with_node_type(RawNodeType::Root),
            children: children_indexes,
            redirect: None,
            name: None,
            parser: None,
            suggestions_type: None,
        };

        root_node.serialize(&mut writer);

        for command in &self.commands {
            command.serialize(&mut writer).await;
        }

        VarInt::from(0).serialize(&mut writer);
    }
    pub async fn parse(
        &self,
        arguments: &mut Vec<Argument>,
        input: &str,
        index: i32,
    ) -> ParseResult<Handler, Proxy> {
        let (word, next_data) = read_word(input);

        for command in &self.commands {
            let is_alias = if let Some(alias) = &command.aliases {
                alias.iter().any(|alias| alias.name == word)
            } else {
                false
            };
            if command.name == word || is_alias {
                match (&command.execution, next_data) {
                    (Some(execution), None) => {
                        return ParseResult {
                            executable: Ok(&execution),
                            suggestions: None,
                        };
                    }
                    (_, Some(next_data)) => {
                        return command
                            .parse(arguments, next_data, index + word.len() as i32 + 1)
                            .await;
                    }
                    (None, None) => {
                        return ParseResult {
                            executable: Err(CommandError::NotExecutable),
                            suggestions: None,
                        };
                    }
                }
            }
        }

        ParseResult {
            executable: Err(CommandError::NoMatches {
                data: word.to_string(),
            }),
            suggestions: None,
        }
    }
}

impl<Handler, Proxy> Command<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    pub fn new(name: &'static str, description: &'static str) -> Self {
        Command {
            name,
            description,
            aliases: None,
            execution: None,
            children: Vec::new(),
            node_index: Mutex::new(None),
        }
    }
    pub fn with_execution(mut self, execution: CommandFn<Handler, Proxy>) -> Self {
        self.execution = Some(execution);
        self
    }
    pub fn set_aliases(mut self, aliases: Vec<&'static str>) -> Self {
        self.aliases = Some(
            aliases
                .iter()
                .map(|alias| CommandAlias {
                    name: alias,
                    node_index: Mutex::new(None),
                })
                .collect(),
        );
        self
    }
    pub fn add_node(mut self, node: CommandNode<Handler, Proxy>) -> Self {
        self.children.push(node);
        self
    }
    async fn parse(
        &self,
        arguments: &mut Vec<Argument>,
        input: &str,
        index: i32,
    ) -> ParseResult<Handler, Proxy> {
        for child in &self.children {
            let result = child.parse(arguments, input, index).await;
            if result.executable.is_ok() || result.suggestions.is_some() {
                return result;
            }
        }

        ParseResult {
            executable: Err(CommandError::Incomplete),
            suggestions: None,
        }
    }
    async fn serialize<W: Write + Send + Sync>(&self, mut writer: W) {
        let mut child_indexes = Vec::with_capacity(self.children.len());

        for child in &self.children {
            child_indexes.push(VarInt::from(
                child.node_index.lock().await.expect("no index"),
            ));
        }

        let node = RawCommandNode {
            flags: CommandFlags::new().with_node_type(RawNodeType::Literal),
            children: child_indexes,
            redirect: None,
            name: Some(self.name.to_string()),
            parser: None,
            suggestions_type: None,
        };

        node.serialize(&mut writer);

        for child in &self.children {
            child.serialize(&mut writer).await;
        }

        if let Some(aliases) = &self.aliases {
            let command_index = self.node_index.lock().await.expect("no index");
            for alias in aliases {
                let alias_node = RawCommandNode {
                    flags: CommandFlags::new()
                        .with_node_type(RawNodeType::Literal)
                        .with_has_redirect(true),
                    children: Vec::new(),
                    redirect: Some(VarInt::from(command_index)),
                    name: Some(alias.name.to_string()),
                    parser: None,
                    suggestions_type: None,
                };

                alias_node.serialize(&mut writer);
            }
        }
    }
    async fn assign_index(&self, index: &mut i32) {
        *self.node_index.lock().await = Some(*index);

        *index += 1;

        for child in &self.children {
            child.assign_index(index).await;
        }

        if let Some(aliases) = &self.aliases {
            for alias in aliases {
                *alias.node_index.lock().await = Some(*index);
                *index += 1;
            }
        }
    }
}

impl<Handler, Proxy> CommandNode<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    #[deprecated]
    pub fn new() -> Self {
        CommandNode {
            node_type: NodeType::Literal {
                name: "".to_string(),
            },
            execution: None,
            children: Vec::new(),
            node_index: Mutex::new(None),
        }
    }
    pub fn literal(name: &'static str) -> Self {
        CommandNode {
            node_type: NodeType::Literal {
                name: name.to_string(),
            },
            execution: None,
            children: Vec::new(),
            node_index: Mutex::new(None),
        }
    }
    pub fn argument(name: &'static str, arg: ArgumentType) -> Self {
        CommandNode {
            node_type: NodeType::Argument {
                name: name.to_string(),
                parser: arg,
            },
            execution: None,
            children: Vec::new(),
            node_index: Mutex::new(None),
        }
    }
    pub fn server_argument(
        name: &'static str,
        arg: ArgumentType,
        suggestions: Vec<String>,
    ) -> Self {
        CommandNode {
            node_type: NodeType::ServerCompletedArgument {
                name: name.to_string(),
                parser: arg,
                suggestions,
            },
            execution: None,
            children: Vec::new(),
            node_index: Mutex::new(None),
        }
    }
    pub fn with_execution(mut self, execution: CommandFn<Handler, Proxy>) -> Self {
        self.execution = Some(execution);
        self
    }
    pub fn add_node(mut self, node: CommandNode<Handler, Proxy>) -> Self {
        self.children.push(node);
        self
    }
    #[async_recursion]
    async fn parse(
        &self,
        arguments: &mut Vec<Argument>,
        input: &str,
        index: i32,
    ) -> ParseResult<Handler, Proxy> {
        match &self.node_type {
            NodeType::Literal { name } => {
                let (word, next_data) = read_word(input);

                if word == name {
                    arguments.push(Argument::Literal {
                        value: name.to_string(),
                    });
                    if let Some(next_data) = next_data {
                        let new_index = index + word.len() as i32 + 1;
                        for child in &self.children {
                            let result = child.parse(arguments, next_data, new_index).await;
                            if result.executable.is_ok() || result.suggestions.is_some() {
                                return result;
                            }
                        }
                    } else {
                        if let Some(execution) = &self.execution {
                            return ParseResult {
                                executable: Ok(execution),
                                suggestions: None,
                            };
                        }
                    }
                    return ParseResult {
                        executable: Err(CommandError::NotExecutable),
                        suggestions: None,
                    };
                }
            }
            NodeType::Argument { parser, .. } => {
                let (data, next_data) = parser.capture_data(input);

                arguments.push(parser.parse(data));

                if let Some(next_data) = next_data {
                    let new_index = index + data.len() as i32 + 1;

                    for child in &self.children {
                        let result = child.parse(arguments, next_data, new_index).await;
                        if result.executable.is_ok() || result.suggestions.is_some() {
                            return result;
                        }
                    }
                } else {
                    return ParseResult {
                        executable: self.execution.as_ref().ok_or(CommandError::NotExecutable),
                        suggestions: None,
                    };
                };
            }
            NodeType::ServerCompletedArgument {
                parser,
                suggestions,
                ..
            } => {
                let (data, next_data) = parser.capture_data(input);

                if let Some(next_data) = next_data {
                    if suggestions.contains(&data.to_string()) {
                        arguments.push(parser.parse(data));
                    } else {
                        return ParseResult {
                            executable: Err(CommandError::NotInSuggestions {
                                suggestions: suggestions.clone(),
                                data: data.to_string(),
                            }),
                            suggestions: None,
                        };
                    }

                    let new_index = index + data.len() as i32 + 1;

                    for child in &self.children {
                        let result = child.parse(arguments, next_data, new_index).await;
                        if result.executable.is_ok() || result.suggestions.is_some() {
                            return result;
                        }
                    }
                } else {
                    let mut matches = Vec::new();
                    for suggestion in suggestions {
                        if suggestion.starts_with(data) && suggestion != data {
                            matches.push(SuggestionMatch {
                                r#match: suggestion.to_string(),
                                tooltip: None,
                            });
                        }
                    }

                    let suggestions_response = if !matches.is_empty() {
                        Some(Suggestions {
                            matches,
                            start: index as usize,
                            length: data.len(),
                        })
                    } else {
                        None
                    };

                    if suggestions.contains(&data.to_string()) {
                        arguments.push(parser.parse(data));
                    } else {
                        return ParseResult {
                            executable: Err(CommandError::NotInSuggestions {
                                suggestions: suggestions.clone(),
                                data: data.to_string(),
                            }),
                            suggestions: suggestions_response,
                        };
                    }
                    return ParseResult {
                        executable: self.execution.as_ref().ok_or(CommandError::NotExecutable),
                        suggestions: suggestions_response,
                    };
                };
            }
        }
        ParseResult {
            executable: Err(CommandError::Incomplete),
            suggestions: None,
        }
    }
    #[async_recursion]
    pub async fn serialize<W: Write + Send + Sync>(&self, mut writer: &mut W) {
        let mut children_indexes = Vec::with_capacity(self.children.len());

        for child in &self.children {
            children_indexes.push(VarInt::from(
                child.node_index.lock().await.expect("no index"),
            ));
        }

        let node = match &self.node_type {
            NodeType::Literal { ref name } => RawCommandNode {
                flags: CommandFlags::new().with_node_type(RawNodeType::Literal),
                children: children_indexes,
                redirect: None,
                name: Some(name.to_string()),
                parser: None,
                suggestions_type: None,
            },
            NodeType::Argument { name, parser } => RawCommandNode {
                flags: CommandFlags::new().with_node_type(RawNodeType::Argument),
                children: children_indexes,
                redirect: None,
                name: Some(name.to_string()),
                parser: Some(parser.clone()),
                suggestions_type: None,
            },
            NodeType::ServerCompletedArgument { name, parser, .. } => RawCommandNode {
                flags: CommandFlags::new()
                    .with_node_type(RawNodeType::Argument)
                    .with_has_suggestions(true),
                children: children_indexes,
                redirect: None,
                name: Some(name.to_string()),
                parser: Some(parser.clone()),
                suggestions_type: Some(SuggestionsType::AskServer),
            },
        };

        node.serialize(&mut writer);

        for child in &self.children {
            child.serialize(writer).await;
        }
    }
    #[async_recursion]
    async fn assign_index(&self, index: &mut i32) {
        self.node_index.lock().await.replace(*index);

        *index += 1;

        for child in &self.children {
            child.assign_index(index).await;
        }
    }
}

fn read_word(input: &str) -> (&str, Option<&str>) {
    if let Some((word, next_data)) = input.split_once(" ") {
        (word, Some(next_data))
    } else {
        (input, None)
    }
}

impl ArgumentType {
    fn capture_data<'a>(&self, input: &'a str) -> (&'a str, Option<&'a str>) {
        match self {
            ArgumentType::String { string_type } => match string_type {
                StringType::SingleWord => read_word(input),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
    fn parse(&self, input: &str) -> Argument {
        match self {
            ArgumentType::String { string_type } => match string_type {
                StringType::SingleWord => Argument::String {
                    value: input.to_string(),
                },
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(BitfieldSpecifier)]
#[bits = 2]
enum RawNodeType {
    Root,
    Literal,
    Argument,
}

#[bitfield(bits = 5)]
struct CommandFlags {
    node_type: RawNodeType,
    is_executable: bool,
    has_redirect: bool,
    has_suggestions: bool,
}

struct RawCommandNode {
    flags: CommandFlags,
    children: Vec<VarInt>,
    redirect: Option<VarInt>,
    name: Option<String>,
    parser: Option<ArgumentType>,
    suggestions_type: Option<SuggestionsType>,
}

impl RawCommandNode {
    pub fn serialize<W: Write>(self, mut writer: W) {
        // Write the flags
        self.flags.into_bytes()[0].serialize(&mut writer);

        // Write the child count and children
        VarInt(self.children.len() as i32).serialize(&mut writer);
        for child in &self.children {
            child.serialize(&mut writer);
        }

        // Write the redirect
        if let Some(redirect) = &self.redirect {
            redirect.serialize(&mut writer);
        }

        // Write the name
        if let Some(name) = &self.name {
            name.to_string().serialize(&mut writer);
        }

        // Write the parser
        if let Some(parser) = &self.parser {
            parser.serialize(&mut writer);
        }

        // Write the suggestions
        if let Some(suggestions) = &self.suggestions_type {
            suggestions.serialize(&mut writer);
        }
    }
}

impl SerializeField for SuggestionsType {
    fn serialize<W: std::io::Write>(&self, mut writer: W) {
        match self {
            SuggestionsType::AskServer => {
                "minecraft:ask_server".to_string().serialize(&mut writer);
            }
            SuggestionsType::AllRecipes => {
                "minecraft:all_recipes".to_string().serialize(&mut writer);
            }
            SuggestionsType::AvailableSounds => {
                "minecraft:available_sounds"
                    .to_string()
                    .serialize(&mut writer);
            }
            SuggestionsType::AvailableBiomes => {
                "minecraft:available_biomes"
                    .to_string()
                    .serialize(&mut writer);
            }
            SuggestionsType::SummonableEntities => {
                "minecraft:summonable_entities"
                    .to_string()
                    .serialize(&mut writer);
            }
        }
    }
}

impl SerializeField for ArgumentType {
    fn serialize<W: std::io::Write>(&self, mut writer: W) {
        match self {
            ArgumentType::Bool => {
                VarInt(0).serialize(&mut writer);
            }
            ArgumentType::Float { min, max } => {
                VarInt(1).serialize(&mut writer);
                let flags = {
                    let mut flags = 0x00u8;
                    if min.is_some() {
                        flags |= 0x01;
                    }
                    if max.is_some() {
                        flags |= 0x02;
                    }
                    flags
                };
                flags.serialize(&mut writer);
                if let Some(min) = min {
                    min.serialize(&mut writer);
                }
                if let Some(max) = max {
                    max.serialize(&mut writer);
                }
            }
            ArgumentType::String { string_type } => {
                VarInt(5).serialize(&mut writer);
                string_type.serialize(&mut writer);
            }
        }
    }
}

impl<Handler, Proxy> Debug for CommandNode<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("CommandNode");
        debug.field("node_type", &self.node_type);
        debug.field("children", &self.children);
        debug.field("node_index", &self.node_index);
        debug.finish()
    }
}

// #[tokio::test]
// async fn test_parse() {
//     let command_tree = CommandNode::root().sub_command(
//         CommandNode::literal("test")
//             .set_aliases(vec!["t", "t1"])
//             .sub_command(CommandNode::literal("test2").sub_command(CommandNode::literal("test3"))),
//     );

//     let mut args = Vec::new();

//     command_tree.parse("", &mut args);
// }
