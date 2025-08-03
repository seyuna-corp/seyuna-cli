#!/usr/bin/env node
const { spawn } = require("child_process");
const path = require("path");
const os = require("os");

const exe = os.platform() === "win32" ? "seyuna.exe" : "seyuna";
const binPath = path.join(__dirname, exe);

const proc = spawn(binPath, process.argv.slice(2), { stdio: "inherit" });
proc.on("exit", code => process.exit(code));
