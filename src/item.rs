use crate::function::Function;
use crate::module::Module;
use crate::dis_variant::DisVariant;
use crate::var_def::VarDef;

use crate::r#enum::Enum;
use crate::r#impl::Impl;
use crate::r#struct::Struct;
use crate::r#trait::Trait;


#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Item {
    Module(Module),
    Struct(Struct),
    Function(Function),
    Trait(Trait),
    Enum(Enum),
    DisEnum(Enum<DisVariant>),
    Impl(Impl),
    VarDef(VarDef),
    Raw(String),
}
