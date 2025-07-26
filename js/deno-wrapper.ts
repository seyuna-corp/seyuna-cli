#!/usr/bin/env -S deno run --allow-run --allow-read --allow-write --allow-net

import { join } from "jsr:@std/path@1";
import { existsSync } from "jsr:@std/fs@1";

async function main() {
  const platform = Deno.build.os; // "windows", "linux", or "darwin"
  const binName = platform === "windows" ? "seyuna.exe" : "seyuna";
  const binPath = join(Deno.cwd(), "bin", binName);

  // If binary does not exist, run deno-install.ts to download it
  if (!existsSync(binPath)) {
    console.log("seyuna binary not found, running deno-install.ts to download it...");
    const installProcess = Deno.run({
      cmd: ["deno", "run", "--allow-write", "--allow-net", "deno-install.ts"],
      stdin: "inherit",
      stdout: "inherit",
      stderr: "inherit",
    });
    const installStatus = await installProcess.status();
    installProcess.close();
    if (!installStatus.success) {
      console.error("Failed to run deno-install.ts");
      Deno.exit(installStatus.code);
    }
  }

  // Now run the seyuna binary with passed arguments
  const cmd = [binPath, ...Deno.args];
  const process = Deno.run({
    cmd,
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  });

  const status = await process.status();
  process.close();
  Deno.exit(status.code);
}

main();
