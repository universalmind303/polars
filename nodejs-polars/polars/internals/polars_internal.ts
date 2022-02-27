const isBrowser = () => typeof window !== `undefined`;
let pl: any = null;

if (isBrowser()) {
  console.log("loading from browser");
  const _pl = await import("../../browser/pkg/index.js");
  pl = _pl;
}
else {
  console.log(`is node.js`);
  let loadBinding: any = await import("@node-rs/helper");
  pl = loadBinding("../../", "nodejs-polars", "nodejs-polars");

}

export default pl;
