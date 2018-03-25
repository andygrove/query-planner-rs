// Copyright 2018 Grove Enterprises LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::clone::Clone;
use std::iter::Iterator;
use std::rc::Rc;
use std::str;
use std::string::String;
use std::cmp::{Ordering, PartialOrd};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum DataType {
    Boolean,
    Float32,
    Float64,
    Int32,
    Int64,
    Utf8,
    Struct(Vec<Field>)
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Field {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool
}

impl Field {
    pub fn new(name: &str, data_type: DataType, nullable: bool) -> Self {
        Field {
            name: name.to_string(),
            data_type: data_type,
            nullable: nullable
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}: {:?}", self.name, self.data_type)
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Schema {
    pub columns: Vec<Field>
}

impl Schema {

    /// create an empty schema
    pub fn empty() -> Self { Schema { columns: vec![] } }

    pub fn new(columns: Vec<Field>) -> Self { Schema { columns: columns } }

    /// look up a column by name and return a reference to the column along with it's index
    pub fn column(&self, name: &str) -> Option<(usize, &Field)> {
        self.columns.iter()
            .enumerate()
            .find(|&(_,c)| c.name == name)
    }

    pub fn to_string(&self) -> String {
        let s : Vec<String> = self.columns.iter()
            .map(|c| c.to_string())
            .collect();
        s.join(",")
    }

}


#[derive(Debug)]
pub enum Array {
    BroadcastVariable(Value), //TODO remove .. not an arrow concept
    Boolean(Vec<bool>),
    Float32(Vec<f32>),
    Float64(Vec<f64>),
    Int32(Vec<i32>),
    Int64(Vec<i64>),
    Utf8(Vec<String>),
    Struct(Vec<Rc<Array>>)
}

impl Array {

    pub fn len(&self) -> usize {
        match self {
            &Array::BroadcastVariable(_) => 1,
            &Array::Boolean(ref v) => v.len(),
            &Array::Float32(ref v) => v.len(),
            &Array::Float64(ref v) => v.len(),
            &Array::Int32(ref v) => v.len(),
            &Array::Int64(ref v) => v.len(),
            &Array::Utf8(ref v) => v.len(),
            &Array::Struct(ref v) => v[0].len(),
        }
    }

    pub fn eq(&self, other: &Array) -> Vec<bool> {
        match (self, other) {
            // compare column to literal
            (&Array::Float32(ref l), &Array::BroadcastVariable(Value::Float32(b))) => l.iter().map(|a| a==&b).collect(),
            (&Array::Float64(ref l), &Array::BroadcastVariable(Value::Float64(b))) => l.iter().map(|a| a==&b).collect(),
            (&Array::Int32(ref l), &Array::BroadcastVariable(Value::Int32(b))) => l.iter().map(|a| a==&b).collect(),
            (&Array::Int64(ref l), &Array::BroadcastVariable(Value::Int64(b))) => l.iter().map(|a| a==&b).collect(),
            (&Array::Utf8(ref l), &Array::BroadcastVariable(Value::Utf8(ref b))) => l.iter().map(|a| a==b).collect(),
            // compare column to column
            (&Array::Float32(ref l), &Array::Float32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a==b).collect(),
            (&Array::Float64(ref l), &Array::Float64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a==b).collect(),
            (&Array::Int32(ref l), &Array::Int32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a==b).collect(),
            (&Array::Int64(ref l), &Array::Int64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a==b).collect(),
            (&Array::Utf8(ref l), &Array::Utf8(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a==b).collect(),
            _ => panic!(format!("ColumnData.eq() Type mismatch: {:?} vs {:?}", self, other))
        }
    }

    pub fn not_eq(&self, other: &Array) -> Vec<bool> {
        match (self, other) {
            // compare column to literal
            (&Array::Float32(ref l), &Array::BroadcastVariable(Value::Float32(b))) => l.iter().map(|a| a!=&b).collect(),
            (&Array::Float64(ref l), &Array::BroadcastVariable(Value::Float64(b))) => l.iter().map(|a| a!=&b).collect(),
            (&Array::Int32(ref l), &Array::BroadcastVariable(Value::Int32(b))) => l.iter().map(|a| a!=&b).collect(),
            (&Array::Int64(ref l), &Array::BroadcastVariable(Value::Int64(b))) => l.iter().map(|a| a!=&b).collect(),
            (&Array::Utf8(ref l), &Array::BroadcastVariable(Value::Utf8(ref b))) => l.iter().map(|a| a!=b).collect(),
            // compare column to column
            (&Array::Float32(ref l), &Array::Float32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a!=b).collect(),
            (&Array::Float64(ref l), &Array::Float64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a!=b).collect(),
            (&Array::Int32(ref l), &Array::Int32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a!=b).collect(),
            (&Array::Int64(ref l), &Array::Int64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a!=b).collect(),
            (&Array::Utf8(ref l), &Array::Utf8(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a!=b).collect(),
            _ => panic!(format!("ColumnData.eq() Type mismatch: {:?} vs {:?}", self, other))
        }
    }

    pub fn lt(&self, other: &Array) -> Vec<bool> {
        match (self, other) {
            // compare column to literal
            (&Array::Float32(ref l), &Array::BroadcastVariable(Value::Float32(b))) => l.iter().map(|a| a<&b).collect(),
            (&Array::Float64(ref l), &Array::BroadcastVariable(Value::Float64(b))) => l.iter().map(|a| a<&b).collect(),
            (&Array::Int32(ref l), &Array::BroadcastVariable(Value::Int32(b))) => l.iter().map(|a| a<&b).collect(),
            (&Array::Int64(ref l), &Array::BroadcastVariable(Value::Int64(b))) => l.iter().map(|a| a<&b).collect(),
            (&Array::Utf8(ref l), &Array::BroadcastVariable(Value::Utf8(ref b))) => l.iter().map(|a| a<b).collect(),
            // compare column to column
            (&Array::Float32(ref l), &Array::Float32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a<b).collect(),
            (&Array::Float64(ref l), &Array::Float64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a<b).collect(),
            (&Array::Int32(ref l), &Array::Int32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a<b).collect(),
            (&Array::Int64(ref l), &Array::Int64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a<b).collect(),
            (&Array::Utf8(ref l), &Array::Utf8(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a<b).collect(),
            _ => panic!(format!("ColumnData.lt() Type mismatch: {:?} vs {:?}", self, other))
        }
    }

    pub fn lt_eq(&self, other: &Array) -> Vec<bool> {
        match (self, other) {
            // compare column to literal
            (&Array::Float32(ref l), &Array::BroadcastVariable(Value::Float32(b))) => l.iter().map(|a| a<=&b).collect(),
            (&Array::Float64(ref l), &Array::BroadcastVariable(Value::Float64(b))) => l.iter().map(|a| a<=&b).collect(),
            (&Array::Int32(ref l), &Array::BroadcastVariable(Value::Int32(b))) => l.iter().map(|a| a<=&b).collect(),
            (&Array::Int64(ref l), &Array::BroadcastVariable(Value::Int64(b))) => l.iter().map(|a| a<=&b).collect(),
            (&Array::Utf8(ref l), &Array::BroadcastVariable(Value::Utf8(ref b))) => l.iter().map(|a| a<=b).collect(),
            // compare column to column
            (&Array::Float32(ref l), &Array::Float32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a<=b).collect(),
            (&Array::Float64(ref l), &Array::Float64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a<=b).collect(),
            (&Array::Int32(ref l), &Array::Int32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a<=b).collect(),
            (&Array::Int64(ref l), &Array::Int64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a<=b).collect(),
            (&Array::Utf8(ref l), &Array::Utf8(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a<=b).collect(),
            _ => panic!(format!("ColumnData.lt_eq() Type mismatch: {:?} vs {:?}", self, other))
        }
    }

    pub fn gt(&self, other: &Array) -> Vec<bool> {
        match (self, other) {
            // compare column to literal
            (&Array::Float32(ref l), &Array::BroadcastVariable(Value::Float32(b))) => l.iter().map(|a| a>&b).collect(),
            (&Array::Float64(ref l), &Array::BroadcastVariable(Value::Float64(b))) => l.iter().map(|a| a>&b).collect(),
            (&Array::Int32(ref l), &Array::BroadcastVariable(Value::Int32(b))) => l.iter().map(|a| a>&b).collect(),
            (&Array::Int64(ref l), &Array::BroadcastVariable(Value::Int64(b))) => l.iter().map(|a| a>&b).collect(),
            (&Array::Utf8(ref l), &Array::BroadcastVariable(Value::Utf8(ref b))) => l.iter().map(|a| a>b).collect(),
            // compare column to column
            (&Array::Float32(ref l), &Array::Float32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a>b).collect(),
            (&Array::Float64(ref l), &Array::Float64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a>b).collect(),
            (&Array::Int32(ref l), &Array::Int32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a>b).collect(),
            (&Array::Int64(ref l), &Array::Int64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a>b).collect(),
            (&Array::Utf8(ref l), &Array::Utf8(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a>b).collect(),
            _ => panic!(format!("ColumnData.gt() Type mismatch: {:?} vs {:?}", self, other))
        }
    }

    pub fn gt_eq(&self, other: &Array) -> Vec<bool> {
        match (self, other) {
            // compare column to literal
            (&Array::Float32(ref l), &Array::BroadcastVariable(Value::Float32(b))) => l.iter().map(|a| a>=&b).collect(),
            (&Array::Float64(ref l), &Array::BroadcastVariable(Value::Float64(b))) => l.iter().map(|a| a>=&b).collect(),
            (&Array::Int32(ref l), &Array::BroadcastVariable(Value::Int32(b))) => l.iter().map(|a| a>=&b).collect(),
            (&Array::Int64(ref l), &Array::BroadcastVariable(Value::Int64(b))) => l.iter().map(|a| a>=&b).collect(),
            (&Array::Utf8(ref l), &Array::BroadcastVariable(Value::Utf8(ref b))) => l.iter().map(|a| a>=b).collect(),
            // compare column to column
            (&Array::Float32(ref l), &Array::Float32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a>=b).collect(),
            (&Array::Float64(ref l), &Array::Float64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a>=b).collect(),
            (&Array::Int32(ref l), &Array::Int32(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a>=b).collect(),
            (&Array::Int64(ref l), &Array::Int64(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a>=b).collect(),
            (&Array::Utf8(ref l), &Array::Utf8(ref r)) => l.iter().zip(r.iter()).map(|(a,b)| a>=b).collect(),
            _ => panic!(format!("ColumnData.gt_eq() Type mismatch: {:?} vs {:?}", self, other))
        }
    }

    pub fn get_value(&self, index: usize) -> Value {
//        println!("get_value() index={}", index);
        let v = match self {
            &Array::BroadcastVariable(ref v) => v.clone(),
            &Array::Boolean(ref v) => Value::Boolean(v[index]),
            &Array::Float32(ref v) => Value::Float32(v[index]),
            &Array::Float64(ref v) => Value::Float64(v[index]),
            &Array::Int32(ref v) => Value::Int32(v[index]),
            &Array::Int64(ref v) => Value::Int64(v[index]),
            &Array::Utf8(ref v) => Value::Utf8(v[index].clone()),
            &Array::Struct(ref v) => {
                // v is Vec<ColumnData>
                // each field has its own ColumnData e.g. lat, lon so we want to get a value from each (but it's recursive)
                //            println!("get_value() complex value has {} fields", v.len());
                let fields = v.iter().map(|field| field.get_value(index)).collect();
                Value::Struct(fields)
            }
        };
        //  println!("get_value() index={} returned {:?}", index, v);

        v
    }

    pub fn filter(&self, bools: &Array) -> Array {
        match bools {
            &Array::Boolean(ref b) => match self {
                &Array::Boolean(ref v) => Array::Boolean(v.iter().zip(b.iter()).filter(|&(_,f)| *f).map(|(v,_)| *v).collect()),
                &Array::Float32(ref v) => Array::Float32(v.iter().zip(b.iter()).filter(|&(_,f)| *f).map(|(v,_)| *v).collect()),
                &Array::Float64(ref v) => Array::Float64(v.iter().zip(b.iter()).filter(|&(_,f)| *f).map(|(v,_)| *v).collect()),
                &Array::Int32(ref v) => Array::Int32(v.iter().zip(b.iter()).filter(|&(_,f)| *f).map(|(v,_)| *v).collect()),
                &Array::Int64(ref v) => Array::Int64(v.iter().zip(b.iter()).filter(|&(_,f)| *f).map(|(v,_)| *v).collect()),
                &Array::Utf8(ref v) => Array::Utf8(v.iter().zip(b.iter()).filter(|&(_,f)| *f).map(|(v,_)| v.clone()).collect()),
                _ => unimplemented!()
            },
            _ => panic!()
        }
    }

}


/// Value holder for all supported data types
#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub enum Value {
    Boolean(bool),
    Float32(f32),
    Float64(f64),
    Int32(i32),
    Int64(i64),
    Utf8(String),
    Struct(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {

        //TODO: implement all type coercion rules

        match self {
            &Value::Float64(l) => match other {
                &Value::Float64(r) => l.partial_cmp(&r),
                &Value::Int64(r) => l.partial_cmp(&(r as f64)),
                _ => unimplemented!("type coercion rules missing")
            },
            &Value::Int64(l) => match other {
                &Value::Float64(r) => (l as f64).partial_cmp(&r),
                &Value::Int64(r) => l.partial_cmp(&r),
                _ => unimplemented!("type coercion rules missing")
            },
            &Value::Utf8(ref l) => match other {
                &Value::Utf8(ref r) => l.partial_cmp(r),
                _ => unimplemented!("type coercion rules missing")
            },
            &Value::Struct(_) => None,
            _ => unimplemented!("type coercion rules missing")
        }

    }
}


impl Value {

    pub fn to_string(&self) -> String {
        match self {
            &Value::Boolean(b) => b.to_string(),
            &Value::Int32(l) => l.to_string(),
            &Value::Int64(l) => l.to_string(),
            &Value::Float32(d) => d.to_string(),
            &Value::Float64(d) => d.to_string(),
            &Value::Utf8(ref s) => s.clone(),
            &Value::Struct(ref v) => {
                let s : Vec<String> = v.iter()
                    .map(|v| v.to_string())
                    .collect();
                s.join(",")
            }
        }
    }

}
