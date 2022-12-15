// use std::{cell::Cell, fmt::Debug};

// use protocol_core::{SerializeField, VarInt};

// #[derive(Debug)]
// struct CommandNode {
//     pub node_type: NodeType,
//     pub redirect: Option<Box<CommandNode>>,
//     pub is_executable: bool,
//     pub children: Vec<CommandNode>,

//     node_index: Cell<Option<i32>>,
// }

// #[derive(Debug)]
// enum NodeType {
//     Root,
//     Literal {
//         name: String,
//     },
//     Argument {
//         name: String,
//         parser: Parser,
//         suggestions_type: SuggestionsType,
//     },
// }

// #[derive(Debug)]
// enum Parser {
//     Bool,
// }

// #[derive(Debug)]
// enum SuggestionsType {
//     None,
//     AskServer,
//     AllRecipes,
//     AvailableSounds,
//     AvailableBiomes,
//     SummonableEntities,
// }

// impl SerializeField for CommandNode {
//     fn serialize<W: std::io::Write>(&self, mut writer: W) {
//         let mut start_index = 0;
//         self.assign_index(&mut start_index);

//         self.write(&mut writer);
//     }
// }

// impl CommandNode {
//     pub fn new(
//         node_type: NodeType,
//         redirect: Option<Box<CommandNode>>,
//         is_executable: bool,
//         children: Vec<CommandNode>,
//     ) -> Self {
//         CommandNode {
//             node_type,
//             redirect,
//             is_executable,
//             children,
//             node_index: Cell::new(None),
//         }
//     }
//     fn write<W: std::io::Write>(&self, mut writer: &mut W) {
//         let flags = {
//             let mut flags = 0x00u8;

//             match self.node_type {
//                 NodeType::Root => flags |= 0x00,
//                 NodeType::Literal { .. } => flags |= 0x01,
//                 NodeType::Argument {
//                     ref suggestions_type,
//                     ..
//                 } => {
//                     flags |= 0x02;
//                     match suggestions_type {
//                         SuggestionsType::None => (),
//                         _ => flags |= 0x10,
//                     }
//                 }
//             }

//             if self.is_executable {
//                 flags |= 0x04;
//             }

//             if let Some(_) = self.redirect {
//                 flags |= 0x08;
//             }

//             flags
//         };

//         flags.serialize(&mut writer);

//         VarInt(self.children.len() as i32).serialize(&mut writer);
//         for child in &self.children {
//             VarInt(child.node_index.get().expect("Node indexes not calculated"))
//                 .serialize(&mut writer);
//         }

//         if let Some(redirect) = &self.redirect {
//             VarInt(
//                 redirect
//                     .node_index
//                     .get()
//                     .expect("Node indexes not calculated"),
//             )
//             .serialize(&mut writer);
//         }

//         match &self.node_type {
//             NodeType::Root => (),
//             NodeType::Literal { name } => {
//                 name.serialize(&mut writer);
//             }
//             NodeType::Argument {
//                 name,
//                 parser,
//                 suggestions_type,
//             } => {
//                 unimplemented!()
//                 // name.serialize(&mut writer);
//                 // parser.serialize(&mut writer);
//                 // suggestions_type.serialize(&mut writer);
//             }
//         }

//         for child in &self.children {
//             child.write(writer);
//         }
//     }
//     fn assign_index(&self, current_index: &mut i32) {
//         self.node_index.set(Some(*current_index));
//         *current_index += 1;

//         for child in &self.children {
//             child.assign_index(current_index);
//         }
//     }
// }

// pub mod test {
//     use super::*;

//     #[test]
//     fn test() {
//         let node = CommandNode::new(
//             NodeType::Root,
//             None,
//             false,
//             vec![CommandNode::new(
//                 NodeType::Literal {
//                     name: "test".to_string(),
//                 },
//                 None,
//                 false,
//                 vec![],
//             )],
//         );
//         let mut buffer = Vec::new();

//         node.serialize(&mut buffer);

//         println!("{:?}", buffer);
//         panic!();
//     }
// }
