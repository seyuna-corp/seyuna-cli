const os = require("os");
const path = require("path");
const fs = require("fs");

const platform = os.platform();
let pkg;

switch (platform) {
  case "darwin":
    pkg = "@seyuna/cli-darwin";
    break;
  case "linux":
    pkg = "@seyuna/cli-linux";
    break;
  case "win32":
    pkg = "@seyuna/cli-win32";
    break;
  default:
    console.error(`Unsupported platform: ${platform}`);
    process.exit(1);
}

try {
  const binSrc = require.resolve(`${pkg}/bin/seyuna${platform === "win32" ? ".exe" : ""}`);
  const binDest = path.join(__dirname, "bin", `seyuna${platform === "win32" ? ".exe" : ""}`);

  fs.mkdirSync(path.dirname(binDest), { recursive: true });
  fs.copyFileSync(binSrc, binDest);
  
  if (platform !== "win32") {
    fs.chmodSync(binDest, 0o755);
  } else {
    // Create Windows CMD shim
    const cmdShimPath = path.join(__dirname, "bin", "seyuna.cmd");
    fs.writeFileSync(cmdShimPath, `@echo off\r\n"${binDest}" %*`);
  }

  console.log(`seyuna installed for ${platform}`);
} catch (err) {
  console.error(`Failed to install seyuna for ${platform}:`, err);
  process.exit(1);
}
