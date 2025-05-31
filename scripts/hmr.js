const livereload = require("livereload");
const path = require("path");

const RELOAD_DELAY = 4000;

const server = livereload.createServer({
  exts: ["html", "js", "css", "less"],
  debug: true,
});

const watchedDir = path.join(process.cwd(), "dist");
server.watch(watchedDir);

const originalRefresh = server.refresh.bind(server);
server.refresh = (filepath) => {
  setTimeout(() => {
    originalRefresh(filepath);
  }, RELOAD_DELAY);
};
