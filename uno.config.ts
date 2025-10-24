import {defineConfig, presetWind4} from 'unocss'

export default defineConfig({
    presets: [
        presetWind4(),
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
    }
})