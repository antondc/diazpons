const fs = require("fs");
const path = require("path");
const glob = require("glob");

// Resolve relative paths for Webpack from project root (./index.js)
const toRelativeImportPath = (absolutePath) =>
  "../" + path.relative(process.cwd(), absolutePath).replace(/\\/g, "/");

// Find .ts and .less files
const tsFiles = glob.sync("./src/**/*.ts");
const lessFiles = glob.sync("./src/**/*.less");

// Create import statements with proper relative paths
const content = [
  ...tsFiles.map((filePath) => `import '${toRelativeImportPath(filePath)}';`),
  ...lessFiles.map((filePath) => `import '${toRelativeImportPath(filePath)}';`),
].join("\n");

// Write to index.js at project root
fs.writeFileSync("./webpack/index.js", content);
