const pl = require("../bin/index.js")


const s = pl.Series([1.11, 2.22, 3.33, null])
console.log(pl.Series.isSeries(s))