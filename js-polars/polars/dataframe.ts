import pl_rs from "./polars_internal";
import util from "util"

/**
 * 
 * @description  
 * A DataFrame is a two-dimensional data structure that represents data as a table 
 * with rows and columns.
 */
export default class Dataframe {
  private df: Dataframe;
  static from = (df: Dataframe) => new Dataframe(df);
  constructor(df: Dataframe) {
    this.df = df
  }
  private of(method: string, args: Object) {
    return new Dataframe(pl_rs[`dataframe_${method}`]({_df: this.df, ...args}))
  }
  private unwrap(method: string) {
    return pl_rs[`dataframe_${method}`]({_df: this.df})
  }

  head = (length = 5) => this.of('head', {length});
  height = () => this.unwrap('height');
  isEmpty = () => this.unwrap('is_empty');
  shape = () => this.unwrap('shape');
  width = () => this.unwrap('width');

  [util.inspect.custom]() {
    
    return this.unwrap('get_fmt')
  }
}



