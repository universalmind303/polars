const pl = require("./pkg/polars")

console.log(pl.JsSeries.new_bool({name: "foo", values:[true]}))