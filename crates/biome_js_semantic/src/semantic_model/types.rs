use std::rc::Rc;

use biome_js_syntax::{JsLanguage, JsNumberLiteralExpression, JsFunctionDeclaration};
use biome_rowan::AstNode;

use crate::SemanticModelData;

/// Marker trait that groups all "AstNode" that have declarations
pub trait HasTypeAstNode: AstNode<Language = JsLanguage> {}

impl HasTypeAstNode for JsNumberLiteralExpression {}
impl HasTypeAstNode for JsFunctionDeclaration {}

pub struct SemanticType {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) idx: usize
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum KnownType {
    Void,
    Any,
    Number,
    Callable {
        arguments: Vec<KnownType>,
        ret: Box<KnownType>
    }
}

impl KnownType {
    pub fn notation(&self) -> String {
        match &self {
            KnownType::Void => "()".to_string(),
            KnownType::Any => "any".to_string(),
            KnownType::Number => "number".to_string(),
            KnownType::Callable { arguments, ret } => {
                let arguments: Vec<String> = arguments.iter().map(|x| x.notation()).collect();
                let ret = ret.notation();
                format!("({}) -> {ret}", arguments.join(", "))
            },
        }
    }
}

#[derive(Debug)]
pub struct SemanticTypeData {
    pub t: KnownType
}

impl SemanticType {
    pub fn notation(&self) -> String {
        let data = &self.data.type_data[self.idx];
        data.t.notation()
    }
}