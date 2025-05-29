<script lang="ts">
    import {onMount} from 'svelte';
    import {Direction, World} from '../pkg';
    import {random} from "./utils/random";

    const CELL_SIZE = 20;
    const GRID_COLOR = '#122334';
    const WORLD_SIZE = 8;
    const FPS = 10;

    let world: World;
    let canvas: HTMLCanvasElement;

    function initializeCanvas(ctx: CanvasRenderingContext2D) {
        const width = CELL_SIZE * world.width();
        const height = CELL_SIZE * world.height();

        canvas.width = width;
        canvas.height = height;

        // Clear the canvas
        ctx.clearRect(0, 0, width, height);
    }

    function drawGrid(ctx: CanvasRenderingContext2D) {
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
        const rewardCellIdx = world.reward_cell();
        const col = rewardCellIdx % world.width();
        const row = Math.floor(rewardCellIdx / world.height());
        ctx.fillStyle = '#78db78';
        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        ctx.stroke();
    }

    function drawSnake(ctx: CanvasRenderingContext2D) {
        const snakeCells = world.snake_cells();
        snakeCells.forEach((cellIdx, i) => {
            const col = cellIdx % world.width();
            const row = Math.floor(cellIdx / world.height());

            ctx.fillStyle = i === 0 ? '#db7878' : '#000000';
            ctx.beginPath();
            ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        })

        ctx.stroke();
    }

    function paint(ctx: CanvasRenderingContext2D) {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        drawGrid(ctx);
        drawSnake(ctx);
        drawRewardCell(ctx);
    }

    function update(ctx: CanvasRenderingContext2D) {
        setTimeout(() => {
            world.step();
            paint(ctx);
            requestAnimationFrame(() => update(ctx));
        }, 1000 / FPS);
    }

    onMount(() => {
        document.addEventListener('keydown', (event) => {
            if (event.code.startsWith('Arrow')) {
                world.set_snake_direction(Direction[event.code.replace('Arrow', '') as keyof typeof Direction]);
            }
        })
        world = World.new(WORLD_SIZE, random(WORLD_SIZE * WORLD_SIZE));

        const ctx = canvas.getContext('2d');
        if (!ctx) {
            console.error('Could not get 2D context from canvas');
            return;
        }

        initializeCanvas(ctx);
        update(ctx);
    });
</script>

<main>
    <div class="top-0 left-0 w-full h-full absolute flex items-center justify-center flex-col">
        <canvas
                bind:this={canvas}
        ></canvas>
    </div>
</main>
