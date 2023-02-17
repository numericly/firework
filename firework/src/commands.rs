use async_recursion::async_recursion;
use firework_protocol::data_types::SuggestionMatch;
use firework_protocol_core::{SerializeField, VarInt};
use firework_protocol_derive::SerializeField;
use futures::future::BoxFuture;
use std::{fmt::Debug, io::Write, marker::PhantomData};
use thiserror::Error;
use tokio::sync::Mutex;

use crate::{client::Client, Server, ServerHandler, ServerProxy};

#[derive(Debug)]
pub struct Suggestion {
    pub suggestions: Vec<SuggestionMatch>,
    pub start: usize,
    pub length: usize,
}

pub struct CommandNode<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    pub node_type: NodeType,
    pub redirect: Option<Box<CommandNode<Handler, Proxy>>>,
    pub execution: Option<
        Box<
            dyn for<'a> Fn(
                    Vec<Argument>,
                    &'a Client<Handler, Proxy>,
                    &'a Server<Handler, Proxy>,
                    &'a Proxy,
                ) -> BoxFuture<'a, ()>
                + Send
                + Sync,
        >,
    >,
    pub children: Vec<CommandNode<Handler, Proxy>>,

    node_index: Mutex<Option<i32>>,
    h: PhantomData<Handler>,
    p: PhantomData<Proxy>,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Root,
    Literal { name: String },
    Argument { name: String, parser: ArgumentType },
}

#[derive(Debug, PartialEq)]
pub enum SuggestionsType {
    AskServer,
    AllRecipes,
    AvailableSounds,
    AvailableBiomes,
    SummonableEntities,
}

#[derive(SerializeField, Debug, PartialEq)]
#[protocol(typ = "firework_protocol_core::VarInt")]
pub enum StringTypes {
    SingleWord,
    QuotablePhrase,
    GreedyPhrase,
}

#[derive(Debug, PartialEq)]
pub enum ArgumentType {
    Bool,
    Float {
        min: Option<f32>,
        max: Option<f32>,
    },
    String {
        string_type: StringTypes,
        suggestions: Option<Vec<String>>,
    },
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
    UnknownData,
    #[error("command is not executable")]
    NotExecutable,
}

impl<Handler, Proxy> CommandNode<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    pub fn root() -> Self {
        Self {
            node_type: NodeType::Root,
            redirect: None,
            execution: None,
            children: Vec::new(),
            node_index: Mutex::new(None),
            h: PhantomData,
            p: PhantomData,
        }
    }
    pub fn literal(name: &'static str) -> Self {
        Self {
            node_type: NodeType::Literal {
                name: name.to_string(),
            },
            redirect: None,
            execution: None,
            children: Vec::new(),
            node_index: Mutex::new(None),
            h: PhantomData,
            p: PhantomData,
        }
    }
    pub fn argument(name: &'static str, argument: ArgumentType) -> Self {
        Self {
            node_type: NodeType::Argument {
                name: name.to_string(),
                parser: argument,
            },
            redirect: None,
            execution: None,
            children: Vec::new(),
            node_index: Mutex::new(None),
            h: PhantomData,
            p: PhantomData,
        }
    }
    pub fn sub_command(mut self, node: CommandNode<Handler, Proxy>) -> Self {
        self.children.push(node);
        self
    }
    pub fn executable(
        mut self,
        exec: Box<
            dyn for<'a> Fn(
                    Vec<Argument>,
                    &'a Client<Handler, Proxy>,
                    &'a Server<Handler, Proxy>,
                    &'a Proxy,
                ) -> BoxFuture<'a, ()>
                + Send
                + Sync,
        >,
    ) -> Self {
        self.execution = Some(exec);
        self
    }
    pub async fn serialize<W: Write + Send + Sync>(&self, mut writer: W) {
        let mut start_index = 0;
        self.assign_index(&mut start_index).await;

        VarInt(start_index).serialize(&mut writer);

        self.write(&mut writer).await;

        VarInt(0).serialize(&mut writer);
    }
    #[async_recursion]
    pub async fn suggestions(&self, input: &str, index: usize) -> Option<Suggestion> {
        match &self.node_type {
            NodeType::Root => {
                if input.len() == 0 {
                    return None;
                }
                for child in &self.children {
                    if let Some(suggestion) = child.suggestions(&input[1..], 1).await {
                        return Some(suggestion);
                    }
                }
                None
            }
            NodeType::Literal { name } => {
                if input.len() == 0 {
                    return None;
                }
                let (data, next_data) = if let Some((data, next_data)) = input.split_once(" ") {
                    (data, Some(next_data))
                } else {
                    (input, None)
                };
                if data == name {
                    if let Some(next_data) = next_data {
                        for child in &self.children {
                            if let Some(suggestion) =
                                child.suggestions(next_data, index + data.len() + 1).await
                            {
                                return Some(suggestion);
                            }
                        }
                    }
                }
                None
            }
            NodeType::Argument { parser, .. } => match parser {
                ArgumentType::String {
                    string_type,
                    suggestions,
                } => {
                    match string_type {
                        StringTypes::SingleWord => {
                            let (data, next_data) =
                                if let Some((data, next_data)) = input.split_once(" ") {
                                    (data, Some(next_data))
                                } else {
                                    (input, None)
                                };

                            if let Some(next_data) = next_data {
                                for child in &self.children {
                                    child.suggestions(next_data, index + data.len() + 1).await;
                                }
                            }
                            if let Some(suggestions) = suggestions {
                                return Some(Suggestion {
                                    suggestions: suggestions
                                        .iter()
                                        .filter(|suggestion| suggestion.contains(data))
                                        .map(|suggestion| SuggestionMatch {
                                            r#match: suggestion.clone(),
                                            tooltip: None,
                                        })
                                        .collect(),
                                    start: index,
                                    length: data.len(),
                                });
                            }
                        }
                        _ => {}
                    }
                    None
                }
                _ => return None,
            },
        }
    }
    #[async_recursion]
    pub async fn execute(
        &self,
        input: &str,
        index: usize,
        mut args: &mut Vec<Argument>,
    ) -> Result<
        Option<(
            &Box<
                dyn for<'a> Fn(
                        Vec<Argument>,
                        &'a Client<Handler, Proxy>,
                        &'a Server<Handler, Proxy>,
                        &'a Proxy,
                    ) -> BoxFuture<'a, ()>
                    + Send
                    + Sync,
            >,
            Vec<Argument>,
        )>,
        CommandError,
    > {
        match &self.node_type {
            NodeType::Root => {
                if input.len() == 0 {
                    return Err(CommandError::EmptyCommand);
                }
                for child in &self.children {
                    if let Some(exec) = child.execute(input, 0, &mut args).await? {
                        return Ok(Some(exec));
                    }
                }
                Err(CommandError::NoMatches {
                    data: input.to_owned(),
                })
            }
            NodeType::Literal { name } => {
                if input.len() == 0 {
                    return Ok(None);
                }
                let (data, next_data) = if let Some((data, next_data)) = input.split_once(" ") {
                    (data, Some(next_data))
                } else {
                    (input, None)
                };
                if data == name {
                    args.push(Argument::Literal {
                        value: data.to_string(),
                    });
                    if let Some(next_data) = next_data {
                        for child in &self.children {
                            if let Some(exec) = child
                                .execute(next_data, index + data.len() + 1, &mut args)
                                .await?
                            {
                                return Ok(Some(exec));
                            };
                        }
                        Err(CommandError::UnknownData)
                    } else {
                        if let Some(execution) = &self.execution {
                            return Ok(Some((execution, args.to_vec())));
                        }
                        Err(CommandError::NotExecutable)
                    }
                } else {
                    Ok(None)
                }
            }
            NodeType::Argument { parser, .. } => match parser {
                ArgumentType::String {
                    string_type,
                    suggestions,
                } => match string_type {
                    StringTypes::SingleWord => {
                        let (data, next_data) =
                            if let Some((data, next_data)) = input.split_once(" ") {
                                (data, Some(next_data))
                            } else {
                                (input, None)
                            };

                        if let Some(suggestions) = suggestions {
                            if !suggestions.contains(&data.to_string()) {
                                return Err(CommandError::NotInSuggestions {
                                    suggestions: suggestions.to_owned(),
                                    data: data.to_owned(),
                                });
                            }
                        }

                        args.push(Argument::String {
                            value: data.to_string(),
                        });

                        if let Some(next_data) = next_data {
                            for child in &self.children {
                                if let Some(exec) = child
                                    .execute(next_data, index + data.len() + 1, &mut args)
                                    .await?
                                {
                                    return Ok(Some(exec));
                                };
                            }
                        } else {
                            if let Some(execution) = &self.execution {
                                return Ok(Some((execution, args.to_vec())));
                            }
                        }
                        Ok(None)
                    }
                    _ => Ok(None),
                },
                _ => return Ok(None),
            },
        }
    }
    #[async_recursion]
    async fn write<W: Write + Send + Sync>(&self, mut writer: &mut W) {
        let flags = {
            let mut flags = 0x00u8;

            match &self.node_type {
                NodeType::Root => flags |= 0x00,
                NodeType::Literal { .. } => flags |= 0x01,
                NodeType::Argument { parser, .. } => {
                    flags |= 0x02;
                    match parser {
                        ArgumentType::String { suggestions, .. } => {
                            if suggestions.is_some() {
                                flags |= 0x10
                            }
                        }
                        _ => {}
                    }
                }
            }

            if self.execution.is_some() {
                flags |= 0x04;
            }

            if let Some(_) = self.redirect {
                flags |= 0x08;
            }

            flags
        };

        flags.serialize(&mut writer);

        VarInt(self.children.len() as i32).serialize(&mut writer);
        for child in &self.children {
            VarInt(
                child
                    .node_index
                    .lock()
                    .await
                    .expect("Node indexes not calculated"),
            )
            .serialize(&mut writer);
        }

        if let Some(redirect) = &self.redirect {
            VarInt(
                redirect
                    .node_index
                    .lock()
                    .await
                    .expect("Node indexes not calculated"),
            )
            .serialize(&mut writer);
        }

        match &self.node_type {
            NodeType::Root => (),
            NodeType::Literal { name } => {
                name.serialize(&mut writer);
            }
            NodeType::Argument { name, parser } => {
                name.serialize(&mut writer);
                parser.serialize(&mut writer);
            }
        }

        for child in &self.children {
            child.write(writer).await;
        }
    }
    #[async_recursion]
    async fn assign_index(&self, current_index: &mut i32) {
        self.node_index.lock().await.replace(*current_index);
        *current_index += 1;

        for child in &self.children {
            child.assign_index(current_index).await;
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
            ArgumentType::String {
                string_type,
                suggestions,
            } => {
                VarInt(5).serialize(&mut writer);
                string_type.serialize(&mut writer);
                if let Some(_) = suggestions {
                    SuggestionsType::AskServer.serialize(&mut writer);
                }
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
        debug.field("redirect", &self.redirect);
        debug.field("children", &self.children);
        debug.field("node_index", &self.node_index);
        debug.finish()
    }
}
