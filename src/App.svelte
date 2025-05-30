<script lang="ts">
    import {onMount} from 'svelte';
    import {Direction, GameStatus, World} from '/pkg/snake_game.js';
    import {random} from "./utils/random";

    const CELL_SIZE = 20;
    const GRID_COLOR = '#122334';
    const WORLD_SIZE = 8;
    const FPS = 5;

    let world = $state<World | null>(null);
    let canvas: HTMLCanvasElement;
    let gameControlBtn: HTMLButtonElement;
    let gameStatus = $state<string | null>(null);
    let gameStatusText = $derived(gameStatus ? gameStatus.toString() : 'None');
    let points = $state(0);


    function initializeCanvas(ctx: CanvasRenderingContext2D) {
        if (!world) return;

        const width = CELL_SIZE * world.width();
        const height = CELL_SIZE * world.height();

        canvas.width = width;
        canvas.height = height;

        // Clear the canvas
        ctx.clearRect(0, 0, width, height);
    }

    function drawGrid(ctx: CanvasRenderingContext2D) {
        if (!world) return;

        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;

        // Draw vertical lines
        for (let x = 0; x <= world.width(); x++) {
            ctx.moveTo(x * CELL_SIZE, 0);
            ctx.lineTo(x * CELL_SIZE, canvas.height);
        }

        // Draw horizontal lines
        for (let y = 0; y <= world.height(); y++) {
            ctx.moveTo(0, y * CELL_SIZE);
            ctx.lineTo(canvas.width, y * CELL_SIZE);
        }

        ctx.stroke();
    }

    function drawRewardCell(ctx: CanvasRenderingContext2D) {
        if (!world) return;

        const rewardCellIdx = world.reward_cell();
        const col = rewardCellIdx! % world.width();
        const row = Math.floor(rewardCellIdx! / world.height());
        ctx.fillStyle = '#78db78';
        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        ctx.stroke();
    }

    function drawSnake(ctx: CanvasRenderingContext2D) {
        if (!world) return;

        const snakeCells = world.snake_cells();
        for (let i = snakeCells.length - 1; i >= 0; i--) {
            const cellIdx = snakeCells[i];
            const col = cellIdx % world!.width();
            const row = Math.floor(cellIdx / world!.height());

            ctx.fillStyle = i === 0 ? '#db7878' : '#000000';
            ctx.beginPath();
            ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        }

        ctx.stroke();
    }

    function paint(ctx: CanvasRenderingContext2D) {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        drawGrid(ctx);
        drawSnake(ctx);
        drawRewardCell(ctx);
    }

    function updateGameControlButton(status: GameStatus | undefined) {
        if (status === GameStatus.Running) {
            gameControlBtn.innerText = 'Playing...';
            gameControlBtn.disabled = true;
        } else if (status === GameStatus.Won || status === GameStatus.Lost) {
            gameControlBtn.innerText = 'Play Again';
            gameControlBtn.disabled = false;
        } else {
            gameControlBtn.innerText = 'Play';
            gameControlBtn.disabled = false;
        }
    }

    function play(ctx: CanvasRenderingContext2D) {
        setTimeout(() => {
            if (!world) return;

            world.step();
            paint(ctx);
            updateGameControlButton(world.game_status());
            gameStatus = world.game_status_text();
            points = world.points();
            if (world.game_status() === GameStatus.Won || world.game_status() === GameStatus.Lost) return;
            requestAnimationFrame(() => play(ctx));
        }, 1000 / FPS);
    }

    onMount(() => {
        document.addEventListener('keydown', (event) => {
            if (event.code.startsWith('Arrow')) {
                if (!world) return;
                world.set_snake_direction(Direction[event.code.replace('Arrow', '') as keyof typeof Direction]);
            }
        })
        world = World.new(WORLD_SIZE, random(WORLD_SIZE * WORLD_SIZE));
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            console.error('Could not get 2D context from canvas');
            return;
        }

        gameControlBtn.addEventListener('click', () => {
            if (!world) return;
            if (world.game_status() === GameStatus.Won || world.game_status() === GameStatus.Lost) {
                world.reset_game(random(WORLD_SIZE * WORLD_SIZE));
                play(ctx);
            }
            world.start_game()
        });

        initializeCanvas(ctx);
        play(ctx);
    });
</script>

<main>
    <div class="top-0 left-0 w-full h-full absolute flex items-center justify-center flex-col">
        <div class="mb-5 justify-center">
            <div class="flex">
                <div class="font-bold mr-10">
                    Status:
                </div>
                <div class="font-medium text-gray-900">
                    {gameStatusText}
                </div>
            </div>
            <div class="flex">
                <div class="font-bold mr-10">
                    Points:
                </div>
                <div class="font-medium text-gray-900">
                    {points}
                </div>
            </div>
            <div class="flex justify-center mt-5">
                <button bind:this={gameControlBtn}
                        class="text-gray-900 hover:text-white border border-gray-800 hover:bg-gray-900 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2 dark:border-gray-600 dark:text-gray-800 dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-gray-800">
                    Play
                </button>
            </div>
        </div>
        <canvas
                bind:this={canvas}
        ></canvas>
    </div>
</main>
