## NPM Link To WASM
1. In the `pkg` folder of the `sokoban-wasm` folder, execute `npm link`
1. In the `sokoban-html` folder:
    1. `npm link sokoban-wasm`
    1. `npm install`
    1. `npm run serve`
