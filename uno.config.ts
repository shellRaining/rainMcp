import { defineConfig, presetUno } from 'unocss';
import presetAnimations from 'unocss-preset-animations';
import { presetShadcn } from 'unocss-preset-shadcn';

export default defineConfig({
  presets: [
    presetUno(),
    presetAnimations(),
    presetShadcn(
      {
        color: 'neutral',
      },
      {
        componentLibrary: 'reka',
      }
    ),
  ],
  content: {
    pipeline: {
      include: [
        /\.(vue|svelte|[jt]sx|mdx?|astro|elm|php|phtml|html)($|\?)/,
        '(components|src)/**/*.{js,ts}',
      ],
    },
  },
  shortcuts: {},
  rules: [],
});
