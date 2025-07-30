// use crate::{
//     lex::token::{Keyword, Operator, Token},
//     parser::{
//         ParserContext, ast::Variable, expression::ExpressionParser, parser_error::ParserError,
//         token_stream::TokenStream,
//     },
// };

// pub struct VariableParser;
//
// impl VariableParser {
//     pub fn parse(
//         ctx: &mut ParserContext,
//         stream: &mut TokenStream,
//     ) -> Result<(Variable, usize), ParserError> {
//         let start_position = stream.position();
//
//         // Parse variable name
//         let name_token = stream.consume(Token::Identifier(String::new()))?;
//         let name = match name_token {
//             Token::Identifier(name) => name,
//             _ => return Err(ParserError::UnexpectedToken(name_token)),
//         };
//
//         // Check what kind of assignment operator we have
//         let next_token = stream.peek().ok_or(ParserError::UnexpectedEndOfInput)?;
//
//         let (is_decl, is_mut) = match next_token {
//             Token::Operator(Operator::Assign) => {
//                 stream.advance(1)?;
//
//                 match (stream.peek(), stream.peek_next()) {
//                     // :mut =
//                     (
//                         Some(Token::Keyword(Keyword::Mut)),
//                         Some(Token::Operator(Operator::Reassign)),
//                     ) => {
//                         stream.advance(2)?;
//                         (true, true) // TODO: This should be (true, true) when implementing mutability
//                     }
//
//                     // :=
//                     (Some(Token::Operator(Operator::Reassign)), _) => {
//                         stream.advance(1)?;
//                         (true, false) // TODO: This should be (true, true) when implementing mutability
//                     }
//
//                     _ => {
//                         return Err(ParserError::UnexpectedToken(stream.current()?.clone()));
//                     }
//                 }
//             }
//             Token::Operator(Operator::Reassign) => {
//                 stream.advance(1)?;
//                 // TODO: Validate that the variable has already been declared
//                 (false, true)
//             }
//             _ => {
//                 return Err(ParserError::UnexpectedToken(next_token.clone()));
//             }
//         };
//
//         let expression = ExpressionParser::parse(ctx, stream)?;
//
//         let end_position = stream.position();
//
//         Ok((
//             Variable {
//                 name,
//                 is_decl,
//                 expression: Some(Box::new(expression)),
//                 // TODO: Proper type info
//                 type_info: None,
//             },
//             end_position - start_position,
//         ))
//     }
// }
//
