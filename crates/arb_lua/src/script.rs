use super::tables::args_table::{load_args_table_global, LuaScriptArgsMap};
use crate::tables::dyn_table::load_dyn_table_global;
use piccolo::{Lua, StaticError};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct LuaScript {
    pub source: String,
}

impl LuaScript {
    pub fn full_lua(args_map: LuaScriptArgsMap) -> Lua {
        let mut lua = Lua::full();

        lua.enter(|ctx| {
            load_args_table_global(ctx, args_map);
            load_dyn_table_global(ctx);
        });

        return lua;
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub enum LuaScriptError {
    /// An error originating from within the Lua script.
    ///
    /// This variant captures errors generated by the Lua code itself,
    /// such as when the script explicitly invokes the `error(message)` function.
    Lua { message: String },
    /// A runtime error that occurs during the execution of the Lua script.
    ///
    /// This variant includes errors such as typos, syntax errors,
    /// or other issues that prevent the Lua script from executing correctly.
    Runtime { message: String },
    /// Indicates that the referenced Lua script could not be found.
    NotFound,
}

impl LuaScriptError {
    pub fn from_static_error(error: StaticError) -> Self {
        match error {
            StaticError::Lua(e) => LuaScriptError::Lua {
                message: e.to_string(),
            },
            StaticError::Runtime(e) => LuaScriptError::Runtime {
                message: e.to_string(),
            },
        }
    }
}
