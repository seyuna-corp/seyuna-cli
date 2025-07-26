import { mkdir, chmod, readFile } from "fs/promises";
import { createWriteStream } from "fs";
import { Readable } from "stream";
import path from "path";
import { fileURLToPath } from "url";
import process from "process";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

async function downloadFile(url, dest) {
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error(`Failed to download ${url}: ${res.status} ${res.statusText}`);
  }

  await mkdir(path.dirname(dest), { recursive: true });

  const readable = Readable.fromWeb(res.body);
  const fileStream = createWriteStream(dest);

  return new Promise((resolve, reject) => {
    readable.pipe(fileStream);
    readable.on("error", reject);
    fileStream.on("finish", resolve);
    fileStream.on("error", reject);
  });
}

async function getVersion() {
  const pkgPath = path.join(__dirname, "package.json");
  const pkgJson = await readFile(pkgPath, "utf8");
  return JSON.parse(pkgJson).version;
}

async function main() {
  const version = await getVersion();

  const platform = process.platform;
  let target;
  if (platform === "win32") {
    target = "seyuna-windows.exe";
  } else if (platform === "darwin") {
    target = "seyuna-macos";
  } else if (platform === "linux") {
    target = "seyuna-linux";
  } else {
    console.error(`Unsupported platform: ${platform}`);
    process.exit(1);
  }

  const releaseUrl = `https://github.com/seyuna-corp/seyuna-cli/releases/download/v${version}/${target}`;
  const dest = path.join(__dirname, "bin", platform === "win32" ? "seyuna.exe" : "seyuna");

  try {
    await downloadFile(releaseUrl, dest);
    if (platform !== "win32") {
      await chmod(dest, 0o755);
    }
    console.log(`seyuna CLI v${version} installed to ${dest}`);
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
}

main();
