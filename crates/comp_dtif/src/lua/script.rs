#![cfg(feature = "lua_scripts")]

use super::{
    freeze::{Freeze, Frozen},
    lib::{
        args::{load_args_table_global, LuaScriptArgsMap},
        comp::load_comp_table_global,
    },
};
use bevy_ecs::world::World;
use piccolo::{Closure, Executor, Function, Lua, StashedExecutor, StaticError};
use std::collections::HashMap;

/// A frozen reference to the ECS [`World`].
pub type FrozenWorld = Frozen<Freeze![&'freeze mut World]>;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct LuaScript {
    pub source: String,
}

// TODO: Optimize & Improve, e.g. run multiple scripts with one Lua instance, error handling, ..
impl LuaScript {
    pub fn run(
        &self,
        world: FrozenWorld,
        args_map: LuaScriptArgsMap,
    ) -> Result<(), LuaScriptError> {
        let mut lua = Lua::full();

        let executor = lua.enter(|ctx| ctx.stash(Executor::new(ctx)));

        lua.enter(|ctx| {
            load_comp_table_global(ctx, world);
            load_args_table_global(ctx, args_map);
        });

        return match Self::run_code(&mut lua, &executor, &self.source) {
            Ok(_) => {
                log::info!("[LuaScript::run] Successfully ran Lua code!");
                Ok(())
            }
            Err(err) => {
                log::error!(
                    "[LuaScript::run] Failed to run Lua code by exception: {:?}",
                    err
                );
                Err(LuaScriptError::from_static_error(err))
            }
        };
    }

    fn run_code(lua: &mut Lua, executor: &StashedExecutor, code: &str) -> Result<(), StaticError> {
        lua.try_enter(|ctx| {
            let closure = Closure::load(ctx, None, code.as_bytes())?;
            let function = Function::compose(&ctx, [closure.into()]);
            ctx.fetch(executor).restart(ctx, function, ());
            Ok(())
        })?;

        return lua.execute::<()>(executor);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct LuaScriptWithId {
    pub id: String,
    pub source: String,
}

impl LuaScriptWithId {
    pub fn into_lua_script(self) -> (String, LuaScript) {
        (
            self.id,
            LuaScript {
                source: self.source,
            },
        )
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
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

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct ToRunLuaScript {
    pub id: String,
    pub args_map: LuaScriptArgsMap,
}

impl ToRunLuaScript {
    pub fn run(
        self,
        scripts: &HashMap<String, LuaScript>,
        frozen_world: FrozenWorld,
    ) -> Result<(), LuaScriptError> {
        if let Some(script) = scripts.get(&self.id) {
            script.run(frozen_world, self.args_map)
        } else {
            Err(LuaScriptError::NotFound)
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct ToRunLuaScripts(pub Vec<ToRunLuaScript>);

impl ToRunLuaScripts {
    pub fn run_batch(self, scripts: &HashMap<String, LuaScript>, world: &mut World) {
        Frozen::in_scope(world, |world| {
            for to_run_script in self.0 {
                to_run_script.run(scripts, world.clone());
            }
        });
    }
}
