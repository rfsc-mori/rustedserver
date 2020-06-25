#![allow(clippy::integer_arithmetic)] // Bit operations for lua.

use anyhow::Result;
use mlua::{Lua, UserData, UserDataMethods, Integer, Variadic};

#[derive(Clone)]
struct LuaBit;

impl UserData for LuaBit {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        macro_rules! bit_op_lua_fn {
            ($op:tt) => {
                |_, values: Variadic<Integer>| {
                    Ok(values.into_iter().fold_first(|a, b| a $op b))
                }
            }
        }

        macro_rules! shift_op_lua_fn {
            ($op:tt) => { |_, (a, b): (Integer, Integer)| Ok(a $op b) }
        }

        methods.add_function("bnot"  , |_, value: Integer| Ok(!value));
        methods.add_function("band"  , bit_op_lua_fn!(&));
        methods.add_function("bor"   , bit_op_lua_fn!(|));
        methods.add_function("bxor"  , bit_op_lua_fn!(^));
        methods.add_function("lshift", shift_op_lua_fn!(<<));
        methods.add_function("rshift", shift_op_lua_fn!(>>));
    }
}

pub fn maybe_register_bit(lua: &Lua) -> Result<()> {
    if !lua.globals().contains_key("bit")? {
        lua.globals().set("bit", LuaBit)?;
    }

    Ok(())
}
