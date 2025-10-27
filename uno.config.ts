import { defineConfig, presetIcons, presetWind4 } from 'unocss'

export default defineConfig({
    presets: [
        presetWind4(),
        presetIcons(),
    ],
    content: {
        filesystem: [
            './src/**/*.rs',
        ]
    },
    theme: {
        colors: {
            primary: '#165DFF',
        },
    },
    // 手动指定需要生成的动态类名
    safelist: [
        'i-mynaui-one',
        'i-mynaui-two',
        'i-mynaui-three',
        'i-mynaui-four',
        'i-mynaui-five',
        'i-mynaui-letter-q',
        'i-mynaui-letter-w',
        'i-mynaui-letter-e',
        'i-mynaui-letter-r',
        'i-mynaui-letter-t',
        'i-mynaui-letter-a',
        'i-mynaui-letter-s',
        'i-mynaui-letter-d',
        'i-mynaui-letter-f',
        'i-mynaui-letter-g',
        'i-mynaui-letter-z',
        'i-mynaui-letter-x',
        'i-mynaui-letter-c',
        'i-mynaui-letter-v',
        'i-mynaui-letter-b',
    ]
})