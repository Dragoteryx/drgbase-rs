#![allow(clippy::missing_safety_doc)]
#![feature(c_unwind)]

use gmod::*;

unsafe fn print(lua: lua::State, text: &str) {
  // get the print function and put it in the stack
  lua.get_global(lua_string!("print"));
  // put the text to print in the stack
  lua.push_string(text);
  // call the function removing it and the string from the stack
  lua.call(1, 0);
}

#[lua_function]
unsafe fn test_binary(lua: lua::State) -> i32 {
  print(lua, "it works!");

  0
}

#[gmod13_open]
pub unsafe fn gmod13_open(lua: lua::State) -> i32 {
  // get the DrGBase global
  lua.get_global(lua_string!("DrGBase"));

  // push the function in the stack
  lua.push_function(test_binary);
  // assign it to DrGBase.TestBinary, removing it from the stack
  lua.set_field(-2, lua_string!("TestBinary"));

  // push a boolean in the stack
  lua.push_boolean(true);
  // assign it to DrGBase.UsesBinaryModule, removing it from the stack
  lua.set_field(-2, lua_string!("UsesBinaryModule"));

  // pop the DrGBase global from the stack
  lua.pop();
  0
}

#[gmod13_close]
pub unsafe fn gmod13_close(lua: lua::State) -> i32 {

  0
}