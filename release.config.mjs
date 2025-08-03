/**
 * @type {import('semantic-release').GlobalConfig}
 */
export default {
  branches: [
    { name: "canary", prerelease: "canary" },
    "main",
  ],
  plugins: [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    [
      "@semantic-release/changelog",
      { changelogFile: "CHANGELOG.md" },
    ],
    [
      "@semantic-release/exec",
      {
        prepareCmd: `
          # Update Cargo version
          sed -i 's/^version = ".*"/version = "\${nextRelease.version}"/' Cargo.toml

          # Inject into npm package.jsons
          VERSION=\${nextRelease.version}
          for pkg in npm/main npm/darwin npm/linux npm/win32; do
            sed -i "s|__CLI_VERSION__|\$VERSION|g" "\$pkg/package.json"
          done

          # Copy binaries from dist folder into npm packages
          mkdir -p npm/linux/bin npm/win32/bin npm/darwin/bin
          cp dist/seyuna-linux npm/linux/bin/seyuna
          cp dist/seyuna-windows.exe npm/win32/bin/seyuna.exe
          cp dist/seyuna-macos npm/darwin/bin/seyuna
        `,
        publishCmd: `
          TAG=\${branch.name}
          if [ "\$TAG" = "main" ]; then TAG=latest; fi
          for pkg in npm/*; do
            echo "Publishing package in \$pkg"
            cd "\$pkg"
            npm publish --access public --tag "\$TAG"
            cd - > /dev/null
          done
        `,
      },
    ],
    [
      "@semantic-release/git",
      {
        assets: ["Cargo.toml", "CHANGELOG.md"],
        message:
          "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}",
      },
    ],
    "@semantic-release/github",
  ],
};
