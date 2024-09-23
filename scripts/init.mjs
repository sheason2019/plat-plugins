//@ts-check
import fs from "fs";
import path from "path";

function main() {
  const platPlugins = JSON.parse(
    fs.readFileSync("./plat-plugins.json").toString()
  );

  for (const plugin of platPlugins.plugins) {
    const expressTarget = path.join(plugin, "src/express");
    if (fs.existsSync(expressTarget)) {
      fs.rmSync(expressTarget);
    }
    fs.symlinkSync(
      path.resolve("../web_tool/src/express"),
      expressTarget,
      "dir"
    );

    const witTarget = path.join(plugin, "wit");
    if (fs.existsSync(witTarget)) {
      fs.rmSync(witTarget);
    }
    fs.symlinkSync(path.resolve("../web_tool/wit"), witTarget, "dir");
  }
}

main();
