// deno-install.ts
import { join, dirname } from "jsr:@std/path@^0.224.0";
import { ensureDir } from "jsr:@std/fs@^0.224.0";

async function getVersion(): Promise<string> {
  const text = await Deno.readTextFile("deno.json");
  const config = JSON.parse(text);
  return config.version;
}

async function downloadFile(url: string, dest: string): Promise<void> {
  const res = await fetch(url);
  if (!res.ok || !res.body) {
    throw new Error(`Failed to download ${url}: ${res.status} ${res.statusText}`);
  }

  await ensureDir(dirname(dest));
  const file = await Deno.open(dest, { create: true, write: true, truncate: true });
  await res.body.pipeTo(file.writable);
}

async function main() {
  const version = await getVersion();
  const platform = Deno.build.os;

  let target: string;
  let output: string;

  switch (platform) {
    case "windows":
      target = "seyuna-windows.exe";
      output = "seyuna.exe";
      break;
    case "darwin":
      target = "seyuna-macos";
      output = "seyuna";
      break;
    case "linux":
      target = "seyuna-linux";
      output = "seyuna";
      break;
    default:
      console.error(`Unsupported platform: ${platform}`);
      Deno.exit(1);
  }

  const destPath = join("bin", output);
  const releaseUrl = `https://github.com/seyuna-corp/seyuna-cli/releases/download/v${version}/${target}`;

  try {
    await downloadFile(releaseUrl, destPath);
    if (platform !== "windows") {
      await Deno.chmod(destPath, 0o755);
    }
    console.log(`seyuna CLI v${version} installed to ${destPath}`);
  } catch (err) {
    console.error("Installation failed:", err);
    Deno.exit(1);
  }
}

await main();
