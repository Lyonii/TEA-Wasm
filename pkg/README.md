# TEA 加密

## 生成 WASM 包
```bash
    # wasm-pack build --scope tea
    wasm-pack build --scope <SCOPE>
```

## 抛出的方法
```JS
    // ./site 目录下有示例
    import { base64_encrypt, base64_decrypt, tea_encrypt, tea_decrypt } from './tea-wasm' 
```

## Vite 配置
```bash
    # 安装 vite-plugin-wasm 插件
    pnpm i vite-plugin-wasm -D
```
```JS
    // vite.config.(js|ts)
    import wasm from "vite-plugin-wasm";

    export default defineConfig(({ mode }) => ({
        ...
        plugins: [
            ...
            wasm(),
        ]
    }))
```
