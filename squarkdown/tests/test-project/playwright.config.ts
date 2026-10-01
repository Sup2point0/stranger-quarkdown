import { defineConfig } from "@playwright/test";


export default defineConfig({
	webServer: {
		command: "npm run buildx && npm run preview",
		port: 4173,
	},
	testDir: "playwright/"
});
