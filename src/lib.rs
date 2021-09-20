use neon::prelude::*;
use std::collections::HashMap;


fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn map_key(mut cx: FunctionContext) -> JsResult<JsObject> {
    let my_obj: Handle<JsObject> = cx.argument(0)?; //0
    let names = my_obj.get_own_property_names(&mut cx)?; //2

    let result = cx.empty_object(); //0
    let length = names.len(&mut cx); //0
    for i in 0..length {
        let name = names.get(&mut cx, i)?; //0.5
        let val = my_obj.get(&mut cx, name)?; //1.5
        let val: Handle<JsObject> = val.downcast_or_throw(&mut cx)?; //0.5
        let new_val = val.get(&mut cx, "propA")?; //3
        result.set(&mut cx, name, new_val)?; //3ms !!
    }

    Ok(result)
}

//by measuring each step it seems we can do mucj better here..
fn test(mut cx: FunctionContext) -> JsResult<JsObject>{
    let mcx = &mut cx;

    let my_obj: Handle<JsObject> = mcx.argument(0).expect("arg"); //0
    let names = my_obj.get_own_property_names(mcx).expect("names"); //2
    let length = names.len(mcx); //0
    let mut m: HashMap<String, Handle<JsValue>> = HashMap::with_capacity(length as usize); //0.2
    for i in 0..length {
        let key = names.get(mcx, i).expect("key"); //0.5
        let key:Handle<JsString> = key.downcast(mcx).expect("downcast"); //0.5

        let value = my_obj.get(mcx, key).expect("value"); //0.5
        m.insert(key.value(mcx), value); //4 !!!
    }

     let o = mcx.empty_object();

     for (key, val) in m.iter(){
         let k = mcx.string(key); //0.5
         o.set(mcx, k, *val)?; //6 !!!!
     }

     Ok(o)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("test", test)?;
    cx.export_function("map_key", map_key)?;

    Ok(())
}
