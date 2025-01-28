import adapter from "@sveltejs/adapter-static";
import { sveltePreprocess } from "svelte-preprocess";

import { mdsvex } from "mdsvex";

import scss_config from "./scss-config.js";


const config = {
  extensions: [".svelte", ".md", ".svx"],

  kit: {
    adapter: adapter({
      pages: "build",
      assets: "build",
      fallback: "404.html",
      precompress: false,
      strict: true,
    }),
    paths: {
      base: process.argv.includes("dev") ? "" : process.env.BASE_PATH
    },
    alias: {
      "#src": "./src/",
      "#parts": "./src/parts",
      "#styles": "./src/styles",
      "#scripts": "./src/scripts",
    },
    prerender: {
      handleHttpError: "warn",
    },
  },

  preprocess: [
    mdsvex({
      extensions: [".md", ".svx"],
    }),
    sveltePreprocess({
      scss: scss_config,
    }),
  ],
};

export default config;
