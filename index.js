const { hello, test, map_key } = require("./index.node");
const bench = require("./js-lib/bench")
const { v4 } = require('uuid')

function generateRandomObj(num){
    const obj = {}
    for (let i = 0; i < num; i++) {
        const key = v4();
        obj[key] = {
            propA: 6,
            propB: 4
        }
    }
    return obj
}

function map_prop_js(my_map) {
    const val = {}
    for (const valKey in my_map) {
        val[valKey] = my_map[valKey].propA
    }
}

console.log(hello());

const obj = generateRandomObj(10000);
// const trans = test(obj);
// console.log(trans);
console.log(
    'RUST obj',
    bench(() => test(obj)).toString()
)

console.log(
    'Rust map key',
    bench(() => map_key(obj)).toString()
)

console.log(
    'JS map key',
    bench(() => map_prop_js(obj)).toString()
)