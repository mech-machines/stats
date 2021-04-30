extern crate mech_core;
extern crate mech_utilities;
#[macro_use]
extern crate lazy_static;
use mech_core::{Transaction, ValueIterator, ValueMethods};
use mech_core::{Value, Table, TableIndex};
use mech_core::{Quantity, ToQuantity, QuantityMath, hash_string, make_quantity};

lazy_static! {
  static ref ROW: u64 = hash_string("row");
  static ref COLUMN: u64 = hash_string("column");
  static ref TABLE: u64 = hash_string("table");
}

#[no_mangle]
pub extern "C" fn stats_average(arguments: &Vec<(u64, ValueIterator)>) {                                        

  // TODO test argument count is 1
  let (in_arg_name, vi) = &arguments[0];
  let (_ , mut out) = arguments.last().unwrap().clone();

  let mut in_rows = vi.rows();
  let mut in_columns = vi.columns();

  if *in_arg_name == *ROW {
    out.resize(in_rows, 1);
    for i in 1..=in_rows {
      let mut sum: Value = Value::from_u64(0);
      for j in 1..=in_columns {
        match vi.get(&TableIndex::Index(i),&TableIndex::Index(j)) {
          Some((value,_)) => {
            match sum.add(value) {
              Ok(result) => sum = result,
              _ => (), // TODO Alert user that there was an error
            }
          }
          _ => ()
        }
      }
      out.set_unchecked(i, 1, Value::from_f64(sum.as_f64().unwrap() / vi.columns() as f64));
    }
  } else if *in_arg_name == *COLUMN {
    out.resize(1, in_columns);
    for (i,m) in (1..=in_columns).zip(vi.column_iter.clone()) {
      let mut sum: Value = Value::from_u64(0);
      for (j,k) in (1..=in_rows).zip(vi.row_iter.clone()) {
        match vi.get(&k,&m) {
          Some((value,_)) => {
            match sum.add(value) {
              Ok(result) => sum = result,
              _ => (), // TODO Alert user that there was an error
            }
          }
          _ => ()
        }
      }
      out.set_unchecked(1, i, Value::from_f64(sum.as_f64().unwrap() / vi.rows() as f64));
    }      
  } else if *in_arg_name == *TABLE {
    out.resize(1, 1);
    let mut sum: Value = Value::from_u64(0);
    for (value,_) in vi.clone() {
      match sum.add(value) {
        Ok(result) => sum = result,
        _ => (), // TODO Alert user that there was an error
      }
    }
    out.set_unchecked(1, 1, Value::from_f64(sum.as_f64().unwrap() / (vi.rows() * vi.columns()) as f64));
  } else {
    // TODO Warn about unknown argument
  }
}