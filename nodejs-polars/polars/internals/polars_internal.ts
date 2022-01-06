/* eslint-disable no-undef */
/* eslint-disable camelcase */
import {loadBinding} from "@node-rs/helper";
import {join} from "path";

// eslint-disable-next-line no-undef
const up2 = join(__dirname, "../", "../");


const isBrowser = () => typeof window === `undefined`;
let pl: any = null;

if (isBrowser()) {
  const browserPolars = require("../../browser/pkg/polars");
  console.log(`is browser`);
  pl = {
    series: browserPolars.series
  };

}
else {
  pl =  loadBinding(up2, "nodejs-polars", "nodejs-polars");
  console.log(`is node.js`);

}

export default pl;
