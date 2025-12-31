/**
 * 从 JSON Schema 生成 TypeScript 类型
 *
 * 使用流程:
 * 1. bun run generate:schema (或 cd src-tauri && cargo run --example generate_schema)
 * 2. bun run generate:types
 *
 * 或直接运行: bun run generate
 */

import { compile } from 'json-schema-to-typescript';
import { readFile, writeFile } from 'fs/promises';

const SCHEMA_FILE = 'src/types/schemas/all.json';
const OUTPUT_FILE = 'src/types/generated.ts';

async function main() {
  let schemaContent: string;
  try {
    schemaContent = await readFile(SCHEMA_FILE, 'utf-8');
  } catch {
    console.error(`Schema file not found: ${SCHEMA_FILE}\nRun "bun run generate:schema" first.`);
    process.exit(1);
  }

  const schema = JSON.parse(schemaContent);

  console.log('Generating TypeScript types...');

  const ts = await compile(schema, 'AllTypes', {
    bannerComment: '',
    additionalProperties: false,
  });

  // 添加头部注释，移除 AllTypes 接口（只需要内部类型）
  const header = [
    '// This file is auto-generated from Rust types via JSON Schema.',
    '// Do not edit manually. Run "bun run generate" to regenerate.',
    '',
  ].join('\n');

  // 移除 AllTypes 接口定义
  const cleanedTs = ts.replace(/export interface AllTypes \{[\s\S]*?\n\}\n?/, '').trim();

  await writeFile(OUTPUT_FILE, header + '\n' + cleanedTs + '\n');
  console.log(`Generated: ${OUTPUT_FILE}`);
}

main().catch(console.error);
