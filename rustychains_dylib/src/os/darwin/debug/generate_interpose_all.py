f = open("src/os/darwin/debug/_interpose_all_.rs")

libc_fn_lines = []
for l in f:
  line = l.strip()
  
  if line.startswith("//"):
    continue
  
  if "pub fn " in line:
    libc_fn_lines.append(line.replace("pub fn ", ""))


for (i, line) in enumerate(libc_fn_lines):
  line = line.replace("::", "")
  line = line.replace(", ...", "")

  # line = line.replace("c_int", "libc::c_int")
  # line = line.replace("c_void", "libc::c_void")
  # line = line.replace("size_t", "libc::size_t")
  # line = line.replace("c_char", "libc::c_char")
  libc_fn_lines[i] = line

interposed_fn_lines = []
for line in libc_fn_lines:
  fn_name = line.split("(")[0]
  
  fn_signature = line.replace(fn_name, "", 1)
  fn_signature = fn_signature.replace(";", "")
  
  fn_args_with_type = fn_signature.split(" -> ")[0]
  fn_args_arr = []
  for arg_with_type in fn_args_with_type.split(","):
    arg_and_type = arg_with_type.split(": ")
    if len(arg_and_type) == 1:
      continue

    arg = arg_and_type[0]
    fn_args_arr.append(arg)
 
  if len(fn_args_arr) == 0:
    fn_args = "()"
  else:
    fn_args = ','.join(fn_args_arr)
    if fn_args[-1] != ")":
      fn_args += ")"

  interposed_fn_lines.append(
f"""interpose! {{
// {line}
libc::{fn_name} => fn local_{fn_name}{fn_signature} {{
  if !unsafe {{ INITIALIZED }} {{
      return unsafe {{ libc::{fn_name}{fn_args} }};
  }}

  let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();

  println!(\"{{now}} {fn_name} start\");
  let r = unsafe {{ libc::{fn_name}{fn_args} }};
  println!(\"{{now}} {fn_name} end\");
  r
}} 
}}"""
)

f2 = open("src/os/darwin/debug/interpose_all.rs", 'w')
f2.write(
"""
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

use std::time::SystemTime;

use libc::*;
use super::super::interpose_macro::interpose;
use crate::INITIALIZED;

""")
for line in interposed_fn_lines:
  f2.write(line+"\n")

