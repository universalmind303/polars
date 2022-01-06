const pl = require("./pkg/polars")

console.log(pl)
console.log(pl.series.new_str({name: "foo", values:["aaaa"]}).toString())