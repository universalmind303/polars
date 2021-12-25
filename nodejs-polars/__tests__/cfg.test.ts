import pl from "@polars";

describe("Config", () => {
  let env = process.env;
  let cfg = pl.Config();

  // reset env
  afterEach(() => {
    process.env = env;
  });
  test("tables", () => {
    cfg.setAsciiTables();
    expect(process.env["POLARS_FMT_NO_UTF8"]).toStrictEqual("1");
    cfg.setUtf8Tables();
    expect(process.env["POLARS_FMT_NO_UTF8"]).toBeUndefined();
  });
  test("table width chars", () => {
    cfg.setTblWidthChars(100);
    expect(process.env["POLARS_TABLE_WIDTH"]).toStrictEqual("100");
    cfg.setTblWidthChars(200);
    expect(process.env["POLARS_TABLE_WIDTH"]).toStrictEqual("200");
  });
  test("table cols", () => {
    cfg.setTblCols(50);
    expect(process.env["POLARS_FMT_MAX_COLS"]).toStrictEqual("50");
    cfg.setTblCols(60);
    expect(process.env["POLARS_FMT_MAX_COLS"]).toStrictEqual("60");
  });
  test("table rows", () => {
    cfg.setTblRows(50);
    expect(process.env["POLARS_FMT_MAX_ROWS"]).toStrictEqual("50");
    cfg.setTblRows(60);
    expect(process.env["POLARS_FMT_MAX_ROWS"]).toStrictEqual("60");
  });
});
