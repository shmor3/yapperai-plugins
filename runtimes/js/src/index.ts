import createPlugin from '@extism/extism';

const plugin = await createPlugin(
    'https://github.com/mikezamora/yapperai-plugins/raw/refs/heads/assmebly-script/assembly-script/dist/output.wasm',
    { useWasi: false }
);


const input = "Hello World";
let out = await plugin.call("count_vowels", input);
console.log(out?.text());