import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const packagePath = join(process.cwd(), "package.json");
const packageJson = JSON.parse(readFileSync(packagePath, "utf8"));
const version = packageJson.version;

try {
  const tauriConfPath = join(process.cwd(), "src-tauri", "tauri.conf.json");
  const tauriConf = JSON.parse(readFileSync(tauriConfPath, "utf8"));
  tauriConf.version = version;
  writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + "\n");

  const cargoPath = join(process.cwd(), "src-tauri", "Cargo.toml");
  let cargoContent = readFileSync(cargoPath, "utf8");
  cargoContent = cargoContent.replace(
    /^version = ".*"$/m,
    `version = "${version}"`,
  );
  writeFileSync(cargoPath, cargoContent);

  const { execSync } = await import("child_process");
  execSync("cargo check --manifest-path src-tauri/Cargo.toml", {
    stdio: "inherit",
  });
  execSync("git add src-tauri", { stdio: "inherit" });
  execSync("git commit --amend --no-edit", { stdio: "inherit" });

  console.log(`All version files synced to ${version}`);
} catch (error) {
  console.error("Error syncing versions:", error.message);
  process.exit(1);
}
